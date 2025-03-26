use std::{collections::HashMap, str::FromStr, sync::Arc, thread::sleep, time::Duration};

use alloy::{
    eips::BlockNumberOrTag,
    primitives::{Address, FixedBytes},
    providers::{Provider, ProviderBuilder},
    rpc::types::{BlockTransactionsKind, Filter, Log, TransactionReceipt},
    sol,
};
use dto::{
    Error, Result, User,
    events::{EventRepository, UserNftTransferRepository},
};
use reqwest::Url;
use sqlx::Postgres;

use crate::config;

pub async fn incheon_contents_etl(pool: &sqlx::Pool<Postgres>) -> Result<()> {
    let url = Url::parse(config::get().klaytn.endpoint).unwrap();
    let provider = Arc::new(ProviderBuilder::new().on_http(url));
    let contract_address = Address::from_str(config::get().contracts.incheon_contents).unwrap();
    let init_block_time = get_contract_init_timestamp(provider.clone(), contract_address).await?;

    let mut next = get_offset(pool).await;

    loop {
        tracing::info!("Extracting logs from block: {:?}", next);
        sleep(Duration::from_secs(1));
        let logs = match extract(provider.clone(), next).await {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!("Failed to extract logs: {:?}", e);
                continue;
            }
        };
        if logs.is_empty() {
            continue;
        }
        let events = transform(provider.clone(), logs, init_block_time).await;
        next = match load(pool, events).await {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!("Failed to load events: {:?}", e);
                continue;
            }
        };
    }
}

pub async fn get_offset(pool: &sqlx::Pool<Postgres>) -> BlockNumberOrTag {
    match dto::events::Event::query_builder()
        .order_by_sort_key_desc()
        .query()
        .map(|row| {
            let v: dto::events::Event = row.into();
            v.block_number as u64
        })
        .fetch_optional(pool)
        .await
    {
        Ok(Some(last)) => BlockNumberOrTag::Number(last),
        _ => BlockNumberOrTag::Earliest,
    }
}

pub async fn extract<T: Provider>(provider: Arc<T>, start: BlockNumberOrTag) -> Result<Vec<Log>> {
    let addr = config::get().contracts.incheon_contents;
    let filter = Filter::new()
        .from_block(start)
        .events([
            "TransferSingle(address,address,address,uint256,uint256)",
            "TransferBatch(address,address,address,uint256[],uint256[])",
        ])
        .to_block(BlockNumberOrTag::Latest)
        .address(Address::from_str(addr).unwrap());

    let logs = provider.get_logs(&filter).await.map_err(|e| {
        tracing::warn!("Failed to get logs: {:?}", e);
        Error::Unknown("Failed to get logs".to_string())
    })?;

    Ok(logs)
}

sol! {
    #[derive(Debug, Default, PartialEq)]
    event TransferSingle(address indexed operator, address indexed from, address indexed to, uint256 id, uint256 value);
    #[derive(Debug, Default, PartialEq)]
    event TransferBatch(address indexed operator, address indexed from, address indexed to, uint256[] ids, uint256[] values);

}

pub async fn transform<P: Provider>(
    provider: Arc<P>,
    logs: Vec<Log>,
    init_block_time: u64,
) -> Vec<(dto::events::Event, i64)> {
    let mut events = Vec::new();
    let mut tx_map: HashMap<FixedBytes<32>, TransactionReceipt> = HashMap::new();

    for log in logs {
        let tx_hash = log.transaction_hash.unwrap_or_default();
        let tx_receipt: TransactionReceipt = match tx_map.get(&tx_hash) {
            Some(v) => v.clone(),
            None => {
                let receipt = match provider
                    .get_transaction_receipt(tx_hash.clone())
                    .await
                    .unwrap_or_default()
                {
                    Some(v) => v,
                    None => {
                        tracing::error!(
                            "Failed to get transaction receipt: {}",
                            tx_hash.to_string()
                        );
                        return vec![];
                    }
                };
                tx_map.insert(tx_hash.clone(), receipt.clone());
                receipt
            }
        };

        let block = provider
            .get_block_by_hash(
                log.block_hash.unwrap_or_default(),
                BlockTransactionsKind::Full,
            )
            .await
            .unwrap_or_default()
            .unwrap_or_default();

        let base = dto::events::Event {
            block_number: log.block_number.unwrap_or_default() as i64,
            tx_hash: tx_hash.to_string(),
            log_index: tx_receipt.transaction_index.unwrap_or_default() as i64,
            timestamp: block.header.timestamp as i64,
            sort_key: dto::events::Event::generate_sort_key(
                block.header.timestamp - init_block_time,
                tx_receipt.transaction_index.unwrap_or_default(),
                log.log_index.unwrap_or_default() as u64,
            ) as i64,
            ..Default::default()
        };
        if let Ok(transfer) = log.log_decode::<TransferSingle>() {
            tracing::debug!("TransferSignle: {:?}", log.data());
            events.push((
                dto::events::Event {
                    from_address: transfer.data().from.to_string(),
                    to_address: transfer.data().to.to_string(),
                    operator: transfer.data().operator.to_string(),
                    token_id: transfer.data().id.to_string().parse().unwrap(),

                    ..base.clone()
                },
                transfer.data().value.to_string().parse().unwrap(),
            ));
        } else if let Ok(log) = log.log_decode::<TransferBatch>() {
            tracing::debug!("TransferBatch: {:?}", log.data());
            for (id, value) in log.data().ids.iter().zip(log.data().values.iter()) {
                events.push((
                    dto::events::Event {
                        from_address: log.data().from.to_string(),
                        to_address: log.data().to.to_string(),
                        operator: log.data().operator.to_string(),
                        token_id: id.to_string().parse().unwrap(),

                        ..base.clone()
                    },
                    value.to_string().parse().unwrap(),
                ));
            }
        } else {
            continue;
        }
    }

    events
}

pub async fn load(
    pool: &sqlx::Pool<Postgres>,
    events: Vec<(dto::events::Event, i64)>,
) -> dto::Result<BlockNumberOrTag> {
    let event_repo = EventRepository::new(pool.clone());
    let user_trasnfer_repo = UserNftTransferRepository::new(pool.clone());

    let mut tx = pool.begin().await?;
    let mut last = 0;

    tracing::info!("Starting to load {} events", events.len());

    for (log, amount) in events {
        tracing::debug!(
            "Processing event - tx_hash: {}, token_id: {}, block: {}",
            log.tx_hash,
            log.token_id,
            log.block_number
        );

        tracing::info!(
            "Processing event - tx_hash: {}, token_id: {}, from: {}, to: {}",
            log.tx_hash,
            log.token_id,
            log.from_address,
            log.to_address
        );

        if log.block_number > last {
            last = log.block_number;
        }

        tracing::debug!(
            "From: {}, To: {}, Amount: {}",
            log.from_address,
            log.to_address,
            amount
        );

        let to_user = User::query_builder()
            .evm_address_equals(log.to_address.clone())
            .query()
            .map(User::from)
            .fetch_optional(&mut *tx)
            .await?;

        let from_user = User::query_builder()
            .evm_address_equals(log.from_address.clone())
            .query()
            .map(User::from)
            .fetch_optional(&mut *tx)
            .await?;

        match &to_user {
            Some(user) => tracing::info!(
                "Found recipient user {} for address {}",
                user.id,
                log.to_address
            ),
            None => tracing::warn!("No user found for recipient address {}", log.to_address),
        }

        match &from_user {
            Some(user) => tracing::info!(
                "Found sender user {} for address {}",
                user.id,
                log.from_address
            ),
            None => tracing::debug!("No user found for sender address {}", log.from_address),
        }

        if let Some(event) = dto::events::Event::query_builder()
            .tx_hash_equals(log.tx_hash.clone())
            .log_index_equals(log.log_index)
            .block_number_equals(log.block_number)
            .query()
            .map(dto::events::Event::from)
            .fetch_optional(&mut *tx)
            .await?
        {
            tracing::warn!(
                "Event already exists - ID: {}, tx_hash: {}",
                event.id,
                event.tx_hash
            );
            continue;
        }

        tracing::info!("Inserting new event for token_id: {}", log.token_id);

        let event = match event_repo
            .insert_with_tx(
                &mut *tx,
                log.from_address,
                log.to_address,
                log.tx_hash,
                log.sort_key,
                log.timestamp,
                log.tx_index,
                log.log_index,
                log.block_number,
                log.operator,
                log.token_id,
            )
            .await?
        {
            Some(v) => v,
            None => {
                tracing::error!("Failed to insert event log");
                return Err(Error::Unknown("Failed to insert event log".to_string()));
            }
        };

        if let Some(user) = to_user {
            tracing::info!(
                "Creating transfer record for user {}: +{} of token {}",
                user.id,
                amount,
                log.token_id
            );
            match user_trasnfer_repo
                .insert_with_tx(&mut *tx, user.id, event.id, amount)
                .await
            {
                Ok(_) => tracing::info!("Transfer record created successfully"),
                Err(e) => tracing::error!("Failed to create transfer record: {:?}", e),
            }
        }

        if let Some(user) = from_user {
            tracing::info!(
                "Creating transfer record for user {}: -{} of token {}",
                user.id,
                amount,
                log.token_id
            );
            match user_trasnfer_repo
                .insert_with_tx(&mut *tx, user.id, event.id, -amount)
                .await
            {
                Ok(_) => tracing::debug!("Sender transfer record created"),
                Err(e) => tracing::error!("Failed to create sender transfer record: {:?}", e),
            }
        }
    }

    match tx.commit().await {
        Ok(_) => tracing::info!("Transaction committed successfully"),
        Err(e) => tracing::error!("Failed to commit transaction: {:?}", e),
    }

    Ok(BlockNumberOrTag::Number(last as u64 + 1))
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
