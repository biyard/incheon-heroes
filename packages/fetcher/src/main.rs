pub mod config;

use alloy::dyn_abi::EventExt;
use alloy::eips::BlockNumberOrTag;
use alloy::json_abi::{Event, JsonAbi};
use alloy::primitives::{Address, keccak256};
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::rpc::types::{BlockTransactionsKind, Filter, Log};
use dto::events::{EventRepository, UserNftTransferRepository};
use sqlx::Postgres;
use tokio::net::TcpListener;
use tracing::subscriber::set_global_default;

use by_types::DatabaseConfig;
use dto::*;
use futures_util::stream::StreamExt;
use reqwest::Url;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use tokio::sync::broadcast;

async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    tracing::info!("Running migration");
    let event = dto::events::Event::get_repository(pool.clone());
    let user_nft_trasfer = dto::events::UserNftTransfer::get_repository(pool.clone());

    event.create_this_table().await?;
    user_nft_trasfer.create_this_table().await?;

    event.create_table().await?;
    user_nft_trasfer.create_table().await?;

    tracing::info!("Migration done");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let conf = config::get();

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    migration(&pool).await?;

    let sub = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(config::log_level())
        .finish();

    let _ = set_global_default(sub);

    let (last_block_number, last_tx_hash) = match dto::events::Event::query_builder()
        .order_by_sort_key_desc()
        .query()
        .map(|row| {
            let v: dto::events::Event = row.into();
            (v.block_number as u64, v.tx_hash)
        })
        .fetch_optional(&pool)
        .await
    {
        Ok(row) => {
            if let Some(last) = row {
                (BlockNumberOrTag::Number(last.0 as u64), last.1.clone())
            } else {
                (BlockNumberOrTag::Earliest, "".to_string())
            }
        }
        Err(e) => {
            tracing::error!("Error in get last block number: {:?}", e);
            (BlockNumberOrTag::Earliest, "".to_string())
        }
    };

    let contract_address = conf.contracts.incheon_contents.parse::<Address>().unwrap();
    let url = Url::parse(&conf.klaytn.endpoint).unwrap();
    let provider = ProviderBuilder::new().on_http(url);

    // Get Contract Creation Timestamp
    let init_block_time = get_contract_init_timestamp(provider.clone(), contract_address).await?;
    let event_map: HashMap<String, (String, Event)> = get_event_signature().await?;

    // Fetch logs from the last block stored in the db to the latest block
    let logs = get_prev_event_logs(
        provider.clone(),
        contract_address,
        last_block_number,
        BlockNumberOrTag::Latest,
    )
    .await?;

    for log in logs {
        if let Some(tx_hash) = log.transaction_hash {
            if tx_hash.to_string() == last_tx_hash {
                continue;
            }
        }
        let event_logs = parse_log(provider.clone(), &log, init_block_time, &event_map).await?;
        for event_log in event_logs {
            let _ = insert_db(pool.clone(), event_log).await?;
        }
    }

    let (tx, mut rx) = broadcast::channel::<Log>(100);
    tokio::spawn(async move {
        tracing::debug!("Starting realtime event listener");
        if let Err(e) = realtime_event_listener(tx).await {
            tracing::error!("Error in realtime_event_listener: {:?}", e);
        }
    });

    tokio::spawn(async move {
        tracing::debug!("Starting event Receiver");
        while let Ok(log) = rx.recv().await {
            tracing::debug!("Received MAIN log: {:?}", log);
            match parse_log(provider.clone(), &log, init_block_time, &event_map).await {
                Ok(event_logs) => {
                    for event_log in event_logs {
                        match insert_db(pool.clone(), event_log).await {
                            Ok(_) => {}
                            Err(e) => {
                                tracing::error!("Error in insert_db: {:?}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Error in parse_log: {:?}", e);
                    continue;
                }
            };
        }
    });

    let app = by_axum::new();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let _ = by_axum::serve(listener, app).await;

    Ok(())
}
async fn insert_db(pool: sqlx::Pool<Postgres>, log: EventLog) -> Result<()> {
    let event_repo = EventRepository::new(pool.clone());
    let user_trasnfer_repo = UserNftTransferRepository::new(pool.clone());
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;
    let user = User::query_builder()
        .evm_address_equals(log.to_address.clone())
        .query()
        .map(|r| Into::<User>::into(r))
        .fetch_optional(&pool)
        .await?;

    //FIXME: Check event is duplicated

    let event = match event_repo
        .insert_with_tx(
            &mut *tx,
            log.from_address,
            log.to_address,
            log.tx_hash,
            log.sort_key as i64,
            log.timestamp as i64,
            log.tx_index as i64,
            log.log_index as i64,
            log.block_number as i64,
            log.operator,
            log.token_id as i64,
        )
        .await?
    {
        Some(v) => v,
        None => {
            tracing::error!("Failed to insert event log");
            return Err(Error::Unknown("Failed to insert event log".to_string()));
        }
    };
    if let Some(user) = user {
        let _ = user_trasnfer_repo
            .insert_with_tx(&mut *tx, user.id, event.id, log.amount as i64)
            .await?;
    }

    tx.commit().await?;

    Ok(())
}
/// Extracts function and event signatures from a JSON ABI
async fn get_event_signature() -> Result<HashMap<String, (String, Event)>> {
    let json_abi: JsonAbi = serde_json::from_str(&include_str!(
        "../../main-api/src/abi/incheon-contents.json"
    ))
    .map_err(|e| Error::Klaytn(format!("Failed to parse ABI: {}", e)))?;

    let mut event_map = HashMap::new();

    for target in ["TransferBatch", "TransferSingle"] {
        let event = json_abi
            .events
            .get(target)
            .ok_or_else(|| Error::InvalidType)?;
        let event = &event[0];
        let sig = keccak256(event.signature().as_bytes());
        event_map.insert(sig.to_string(), (target.to_string(), event.clone()));
    }

    Ok(event_map)
}

async fn get_contract_init_timestamp<P: Provider>(provider: P, address: Address) -> Result<u64> {
    let filter = Filter::new()
        .from_block(BlockNumberOrTag::Number(0))
        .to_block(BlockNumberOrTag::Latest)
        .address(address);

    let logs = provider
        .get_logs(&filter)
        .await
        .map_err(|e| Error::Klaytn(e.to_string()))?;
    if let Some(log) = logs.first() {
        if let Some(tx_hash) = log.transaction_hash {
            if let Some(tx) = provider
                .get_transaction_by_hash(tx_hash)
                .await
                .map_err(|e| Error::Klaytn(format!("Failed to get transaction: {:?}", e)))?
            {
                if let Some(block_number) = tx.block_number {
                    if let Some(block) = provider
                        .get_block_by_number(
                            BlockNumberOrTag::Number(block_number),
                            BlockTransactionsKind::Full,
                        )
                        .await
                        .map_err(|e| Error::Klaytn(format!("Failed to get block: {:?}", e)))?
                    {
                        return Ok(block.header.timestamp);
                    }
                }
            }
        }
    }
    Err(Error::Klaytn(
        "Failed to get contract init timestamp".to_string(),
    ))
}
// let event_map: HashMap<String, (String, Event)> = get_event_signature().await?;

async fn parse_log<P>(
    provider: P,
    log: &Log,
    init_timestamp: u64,
    event_map: &HashMap<String, (String, Event)>,
) -> Result<Vec<EventLog>>
where
    P: Provider,
{
    if let Some(sig) = log.topic0() {
        let sig = sig.to_string();
        if let Some((event_name, event)) = event_map.get(&sig) {
            if let Some(hash) = log.block_hash {
                let block = provider
                    .get_block_by_hash(hash, BlockTransactionsKind::Full)
                    .await
                    .map_err(|e| Error::Klaytn(e.to_string()))?
                    .unwrap();
                let block_timestamp = block.header.timestamp;
                match event_name.as_ref() {
                    "TransferSingle" => {
                        let decoded_event = event.decode_log(&log.data(), true).map_err(|e| {
                            Error::Klaytn(format!("Failed to decode log data: {:?}", e))
                        })?;
                        let token_id = match TryInto::<u64>::try_into(
                            decoded_event.body[0].as_uint().unwrap().0,
                        ) {
                            Err(e) => {
                                tracing::error!("{:?}", e);
                                return Err(Error::Klaytn(
                                    "Failed to convert token_id".to_string(),
                                ));
                            }
                            Ok(v) => v,
                        };

                        let amount = match TryInto::<u64>::try_into(
                            decoded_event.body[1].as_uint().unwrap().0,
                        ) {
                            Err(e) => {
                                tracing::error!("{:?}", e);
                                return Err(Error::Klaytn("Failed to convert amount".to_string()));
                            }
                            Ok(v) => v,
                        };

                        return Ok(vec![EventLog {
                            operator: decoded_event.indexed[0].as_address().unwrap().to_string(),
                            from_address: decoded_event.indexed[1]
                                .as_address()
                                .unwrap()
                                .to_string(),
                            to_address: decoded_event.indexed[2].as_address().unwrap().to_string(),
                            timestamp: block_timestamp,
                            sort_key: dto::events::Event::generate_sort_key(
                                block_timestamp - init_timestamp,
                                log.transaction_index.unwrap(),
                                log.log_index.unwrap(),
                            ),
                            tx_index: log.transaction_index.unwrap(),
                            log_index: log.log_index.unwrap(),
                            tx_hash: log.transaction_hash.unwrap().to_string(),
                            block_number: log.block_number.unwrap(),
                            token_id,
                            amount,
                        }]);
                    }
                    "TransferBatch" => {
                        let decoded_event = event.decode_log(&log.data(), true).map_err(|e| {
                            Error::Klaytn(format!("Failed to decode log data: {:?}", e))
                        })?;

                        let operator = decoded_event.indexed[0].as_address().unwrap().to_string();
                        let from_address =
                            decoded_event.indexed[1].as_address().unwrap().to_string();
                        let to_address = decoded_event.indexed[2].as_address().unwrap().to_string();

                        // Get the token_ids array
                        let token_ids_array = decoded_event.body[0].as_array().unwrap();
                        // Get the amounts array
                        let amounts_array = decoded_event.body[1].as_array().unwrap();

                        if token_ids_array.len() != amounts_array.len() {
                            return Err(Error::Klaytn(
                                "Mismatched token_ids and amounts arrays".to_string(),
                            ));
                        }

                        let mut event_logs = Vec::with_capacity(token_ids_array.len());

                        for i in 0..token_ids_array.len() {
                            let token_id = match TryInto::<u64>::try_into(
                                token_ids_array[i].as_uint().unwrap().0,
                            ) {
                                Err(e) => {
                                    tracing::error!("{:?}", e);
                                    return Err(Error::Klaytn(
                                        "Failed to convert token_id".to_string(),
                                    ));
                                }
                                Ok(v) => v,
                            };

                            let amount = match TryInto::<u64>::try_into(
                                amounts_array[i].as_uint().unwrap().0,
                            ) {
                                Err(e) => {
                                    tracing::error!("{:?}", e);
                                    return Err(Error::Klaytn(
                                        "Failed to convert amount".to_string(),
                                    ));
                                }
                                Ok(v) => v,
                            };

                            event_logs.push(EventLog {
                                operator: operator.clone(),
                                from_address: from_address.clone(),
                                to_address: to_address.clone(),
                                timestamp: block_timestamp,
                                sort_key: dto::events::Event::generate_sort_key(
                                    block_timestamp - init_timestamp,
                                    log.transaction_index.unwrap(),
                                    log.log_index.unwrap(),
                                ),
                                tx_index: log.transaction_index.unwrap(),
                                log_index: log.log_index.unwrap(),
                                tx_hash: log.transaction_hash.unwrap().to_string(),
                                block_number: log.block_number.unwrap(),
                                token_id,
                                amount,
                            });
                        }

                        return Ok(event_logs);
                    }
                    _ => {
                        tracing::debug!("Unknown event: {}", event_name);
                    }
                }
            }
        }
    }
    Ok(vec![])
}
async fn realtime_event_listener(tx: broadcast::Sender<Log>) -> Result<()> {
    let conf = config::get();
    let endpoint = conf.klaytn.endpoint.replacen("https", "wss", 1);
    let endpoint = format!("{}/ws", endpoint);
    tracing::debug!("Connecting to: {}", endpoint);
    let ws = WsConnect::new(endpoint);
    let provider = match ProviderBuilder::new().on_ws(ws).await {
        Ok(provider) => provider,
        Err(e) => {
            tracing::error!("Failed to connect to WebSocket: {:?}", e);
            return Err(Error::Klaytn(e.to_string()));
        }
    };
    tracing::info!("WebSocket connection established");
    //NOTE: An error occurred in "alloy_transport_ws".
    //Error Msg(WS connection error err=IO error: peer closed connection without sending TLS close_notify: https://docs.rs/rustls/latest/rustls/manual/_03_howto/index.html#unexpected-eof)
    let contract_address = conf.contracts.incheon_contents.parse::<Address>().unwrap();
    let filter = Filter::new().address(contract_address);
    let sub = match provider.subscribe_logs(&filter).await {
        Ok(sub) => sub,
        Err(e) => {
            tracing::error!("Failed to subscribe to logs: {:?}", e);
            return Err(Error::Klaytn(e.to_string()));
        }
    };
    tracing::info!("Subscribed to logs");

    let mut stream = sub.into_stream();
    while let Some(log) = stream.next().await {
        match tx.send(log) {
            Ok(v) => {
                tracing::debug!("Sent log to receiver: {:?}", v);
            }
            Err(e) => {
                tracing::error!("Failed to send log to receiver: {:?}", e);
            }
        }
    }
    Ok(())
}
async fn get_prev_event_logs<P>(
    provider: P,
    contract_address: Address,
    start_block: BlockNumberOrTag,
    end_block: BlockNumberOrTag,
) -> Result<Vec<Log>>
where
    P: Provider,
{
    let filter = Filter::new()
        .from_block(start_block)
        .to_block(end_block)
        .address(contract_address);

    let logs = provider
        .get_logs(&filter)
        .await
        .map_err(|e| Error::Klaytn(e.to_string()))?;
    Ok(logs)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EventLog {
    pub from_address: String,
    pub to_address: String,
    pub operator: String,
    pub timestamp: u64,
    pub tx_index: u64,
    pub log_index: u64,
    pub tx_hash: String,
    pub block_number: u64,
    pub token_id: u64,
    pub amount: u64,
    pub sort_key: u64,
}
