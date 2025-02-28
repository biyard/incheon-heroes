pub mod config;

use alloy::dyn_abi::EventExt;
use alloy::eips::BlockNumberOrTag;
use alloy::json_abi::{Event, JsonAbi};
use alloy::primitives::{keccak256, Address};
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::rpc::types::{BlockTransactionsKind, Filter, Log};
use dto::events::EventRepository;
use sqlx::Postgres;
use tracing::subscriber::set_global_default;

use by_types::DatabaseConfig;
use dto::*;
use futures_util::stream::StreamExt;
use reqwest::Url;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use tokio::sync::broadcast;

async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    tracing::info!("Running migration");
    let event = dto::events::Event::get_repository(pool.clone());
    event.create_this_table().await?;
    event.create_table().await?;
    tracing::info!("Migration done");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // let app = by_axum::new();
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

    tracing::debug!("config: {:?}", conf);
    let contract_address = conf.contracts.incheon_contents.parse::<Address>().unwrap();

    let url = Url::parse(&conf.klaytn.endpoint).unwrap();
    let provider = ProviderBuilder::new().on_http(url);

    let init_block_time = get_contract_init_timestamp(provider.clone(), contract_address).await?;
    let event_map: HashMap<String, (String, Event)> = get_event_signature().await?;

    // FIXME: The starting block number must be fetched from the database, not 0.
    // let logs = get_prev_event_logs(
    //     provider.clone(),
    //     contract_address,
    //     BlockNumberOrTag::Number(0),
    //     BlockNumberOrTag::Latest,
    // )
    // .await?;

    // for log in logs {
    //     let event_logs = parse_log(provider.clone(), &log, init_block_time, &event_map).await?;
    //     for event_log in event_logs {
    //         let _ = insert_db(pool.clone(), event_log).await?;
    //     }
    // }

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
                        let _ = insert_db(pool.clone(), event_log).await;
                    }
                }
                Err(e) => {
                    tracing::error!("Error in parse_log: {:?}", e);
                    continue;
                }
            };
        }
    });

    // Wait for a termination signal
    tokio::signal::ctrl_c()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    tracing::info!("Shutting down gracefully");

    Ok(())
}
async fn insert_db(pool: sqlx::Pool<Postgres>, log: EventLog) -> Result<()> {
    let repo = EventRepository::new(pool);
    let sort_key =
        dto::events::Event::generate_sort_key(log.timestamp, log.tx_index, log.log_index);

    //FIXME: Check event is duplicated
    repo.insert(
        log.from_address,
        log.to_address,
        log.tx_hash,
        sort_key as i64,
        log.timestamp as i64,
        log.tx_index as i64,
        log.log_index as i64,
        log.block_number as i64,
        log.operator,
        log.token_id as i64,
    )
    .await?;

    Ok(())
}
/// Extracts function and event signatures from a JSON ABI
async fn get_event_signature() -> Result<HashMap<String, (String, Event)>> {
    let mut file = tokio::fs::File::open("../main-api/src/abi/incheon-contents.json")
        .await
        .map_err(|e| Error::Klaytn(format!("Failed to open ABI file: {}", e)))?;

    let mut abi_json = String::new();
    file.read_to_string(&mut abi_json)
        .await
        .map_err(|e| Error::Klaytn(format!("Failed to read ABI file: {}", e)))?;

    let json_abi: JsonAbi = serde_json::from_str(&abi_json)
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
                            timestamp: block_timestamp - init_timestamp,
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
                                timestamp: block_timestamp - init_timestamp,
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

    let endpoint = format!("wss://{}/ws", conf.klaytn.endpoint);
    let ws = WsConnect::new(endpoint);
    let provider = ProviderBuilder::new().on_ws(ws).await;
    let contract_address = conf.contracts.incheon_contents.parse::<Address>().unwrap();
    let filter = Filter::new().address(contract_address);

    if let Ok(provider) = provider {
        let sub = provider
            .subscribe_logs(&filter)
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?;
        let mut stream = sub.into_stream();
        while let Some(log) = stream.next().await {
            tracing::debug!("Received log LISTENER: {:?}", log);
            let _ = tx.send(log);
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
}
