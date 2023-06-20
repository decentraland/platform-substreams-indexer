mod abi;
mod db;
mod macros;
mod pb;
mod rpc;
mod utils;

use hex_literal::hex;
use pb::dcl;
use substreams::prelude::*;
use substreams::scalar::BigInt;
use substreams::{log, Hex};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

use crate::utils::sanitize_sql_string;

// Polygon's Contracts
const _MARKETPLACE_CONTRACT: [u8; 20] = hex!("02080031b45A3c67d338Dd4A2CC309D28756A160");
// const MARKETPLACEV2_CONTRACT: [u8; 20] = hex!("480a0f4e360E8964e68858Dd231c2922f1df45Ef");
const COLLECTIONS_FACTORY: [u8; 20] = hex!("B549B2442b2BD0a53795BC5cDcBFE0cAF7ACA9f8");
const COLLECTIONS_V3_FACTORY: [u8; 20] = hex!("3195e88aE10704b359764CB38e429D24f1c2f781");
// LANDS GOERLI
const LANDS_CONTRACT: [u8; 20] = hex!("0cfF059845c6Abee7De396d00091016ad72FD324");

substreams_ethereum::init!();

// Reads the collection creation by the `ProxyCreated` event
#[substreams::handlers::map]
pub fn map_collection_created(
    blk: eth::Block,
) -> Result<dcl::Collections, substreams::errors::Error> {
    Ok(dcl::Collections {
        collections: blk
            .events::<abi::collection_factoryv3::events::ProxyCreated>(&[
                &COLLECTIONS_FACTORY,
                &COLLECTIONS_V3_FACTORY,
            ])
            .map(|(event, _log)| {
                substreams::log::info!(
                    "Collection created {:?}",
                    Hex(event.address.clone()).to_string()
                );

                let collection_data = rpc::collection_data_call(event.address.clone()); //@TODO avoid clone?
                dcl::Collection {
                    address: Hex(event.address).to_string(),
                    creator: collection_data.0,
                    is_approved: collection_data.1,
                    name: collection_data.2,
                    symbol: collection_data.3,
                    owner: collection_data.4,
                    is_completed: collection_data.5,
                    created_at: blk.timestamp_seconds(),
                }
            })
            .collect(),
    })
}

/// Store addresses of the collections created by map_collection_created
#[substreams::handlers::store]
pub fn store_collections(collections: dcl::Collections, store: StoreSetString) {
    for collection in collections.collections {
        store.set(0, collection.address, &"".to_string()); // we don't really care about the value, we'll just check if the key is present in the store
    }
}

/// Reads Issue events from the contract
#[substreams::handlers::map]
pub fn map_issues(
    blk: eth::Block,
    collections_store: substreams::store::StoreGetString,
) -> Result<dcl::NfTs, substreams::errors::Error> {
    // let mut addresses_string = vec![];
    // addresses_string.push(String::from("27d33da28f31627c98290e401eb22b6bc6b41edd"));
    // let mut addresses = vec![];
    // match hex::decode("27d33da28f31627c98290e401eb22b6bc6b41edd") {
    //     Ok(decoded) => addresses.push(decoded),
    //     Err(_err) => log::debug!("Err decoding address {}"),
    // }
    // let mut addresses_ref = vec![];
    // for address in &addresses {
    //     substreams::log::info!("address {:?}", Hex(address).to_string());
    //     addresses_ref.push(address.as_slice());
    // }

    let mut nfts = vec![];
    for trx in blk.transactions() {
        for call in trx.calls.iter() {
            let _call_index = call.index;
            if call.state_reverted {
                continue;
            }

            for log in call.logs.iter() {
                let collection_address = &Hex(log.clone().address).to_string();
                let mut collection_exists = false;
                if let Some(_collection) = collections_store.get_last(collection_address) {
                    collection_exists = true
                }

                // if addresses_string.contains(collection_address) {
                //     substreams::log::info!("address exist :) {:?}", collection_address.clone());
                //     collection_exists = true
                // }

                if collection_exists {
                    if let Some(event) = abi::collections_v2::events::Issue::match_and_decode(log) {
                        let timestamp = blk.timestamp_seconds().to_string();
                        let nft = dcl::Nft {
                            beneficiary: Hex(&event.beneficiary).to_string(),
                            issued_id: Some(event.issued_id.into()),
                            item_id: utils::get_item_id(
                                Hex(log.address.clone()).to_string(),
                                event.item_id.to_string(),
                            ),
                            token_id: Some(event.token_id.into()),
                            collection_address: Hex(log.address.clone()).to_string(),
                            created_at: timestamp.clone(),
                            updated_at: timestamp,
                        };
                        nfts.push(nft);
                    }
                }
            }
        }
    }
    Ok(dcl::NfTs { nfts })
}

/// Reads item creationg by the `AddItem` event
#[substreams::handlers::map]
pub fn map_add_items(
    blk: eth::Block,
    collections: dcl::Collections,
) -> Result<dcl::Items, substreams::errors::Error> {
    let mut addresses = vec![];
    for collection in collections.collections {
        match hex::decode(&collection.address) {
            Ok(decoded) => addresses.push(decoded),
            Err(_err) => log::debug!("Err decoding the collection address {}", collection.address),
        }
    }
    let mut addresses_ref = vec![];
    for address in &addresses {
        addresses_ref.push(address.as_slice());
    }

    Ok(dcl::Items {
        items: blk
            .events::<abi::collections_v2_fixed::events::AddItem>(&addresses_ref)
            .map(|(add_item_event, log)| {
                let item = add_item_event.item;
                //@TODO missing fields:
                // creation_fee => grab from oracle
                // creator? => it can be part of the collection
                let abi::collections_v2_fixed::events::Erc721BaseCollectionV2Item {
                    beneficiary,
                    rarity,
                    max_supply,
                    total_supply,
                    price,
                    metadata,
                    content_hash,
                } = item;
                sanitize_sql_string(metadata.clone());
                dcl::Item {
                    id: utils::get_item_id(
                        Hex(log.address()).to_string(),
                        add_item_event.item_id.to_string(),
                    ),
                    rarity,
                    max_supply: Some(max_supply.into()),
                    total_supply: Some(total_supply.into()),
                    price: Some(price.into()),
                    beneficiary: Hex(beneficiary).to_string(),
                    metadata: metadata.clone(),
                    content_hash,
                    blockchain_item_id: Some(add_item_event.item_id.into()),
                    collection_id: Hex(log.address()).to_string(),
                    created_at: blk.timestamp_seconds(),
                    item_type: utils::items::build_item_metadata(metadata).item_type,
                }
            })
            .collect(),
    })
}

// Reads the `Complete` event from the Collection contract and the items from it
#[substreams::handlers::map]
pub fn map_collection_complete(
    blk: eth::Block,
    collections: dcl::Collections,
) -> Result<dcl::Items, substreams::errors::Error> {
    let mut addresses = vec![];
    for collection in collections.collections {
        match hex::decode(&collection.address) {
            Ok(decoded) => addresses.push(decoded),
            Err(_err) => log::debug!("Err decoding address {}", collection.address),
        }
    }

    let mut addresses_ref = vec![];
    for address in &addresses {
        addresses_ref.push(address.as_slice());
    }

    Ok(dcl::Items {
        items: blk
            .events::<abi::collections_v2::events::Complete>(&addresses_ref)
            .flat_map(|(_complete_event, log)| {
                let collection_item_count = rpc::get_collection_item_count(log.address().to_vec());
                let mut items = vec![];
                let item_amount =
                    BigInt::to_u64(&collection_item_count.unwrap_or_else(BigInt::zero));
                for n in 0..item_amount {
                    match rpc::get_collection_item(log.address().to_vec(), n) {
                        Some(item) => items.push(dcl::Item {
                            id: utils::get_item_id(Hex(log.address()).to_string(), n.to_string()),
                            rarity: item.0,
                            max_supply: Some(item.1.into()),
                            total_supply: Some(dcl::BigInt {
                                value: item.2.to_string(),
                            }),
                            price: Some(dcl::BigInt {
                                value: item.3.to_string(),
                            }),
                            beneficiary: Hex(item.4).to_string(),
                            metadata: item.5.clone(),
                            content_hash: item.6,
                            blockchain_item_id: Some(dcl::BigInt {
                                value: n.to_string(),
                            }),
                            collection_id: Hex(log.address()).to_string(),
                            created_at: blk.timestamp_seconds(),
                            item_type: utils::items::build_item_metadata(item.5).item_type,
                        }),
                        None => continue,
                    }
                }
                items
            })
            .collect::<Vec<dcl::Item>>(),
    })
}

//  Reads Transfer events from the contract
#[substreams::handlers::map]
pub fn map_transfers(
    blk: eth::Block,
    collections_store: substreams::store::StoreGetString,
) -> Result<dcl::Transfers, substreams::errors::Error> {
    // let mut addresses_string = vec![];
    // addresses_string.push(String::from("27d33da28f31627c98290e401eb22b6bc6b41edd"));
    // let mut addresses = vec![];
    // match hex::decode("27d33da28f31627c98290e401eb22b6bc6b41edd") {
    //     Ok(decoded) => addresses.push(decoded),
    //     Err(_err) => log::debug!("Err decoding address {}"),
    // }

    // let mut addresses_ref = vec![];
    // for address in &addresses {
    //     substreams::log::info!("address {:?}", Hex(address).to_string());
    //     addresses_ref.push(address.as_slice());
    // }

    let mut transfers = vec![];
    for trx in blk.transactions() {
        for call in trx.calls.iter() {
            let _call_index = call.index;
            if call.state_reverted {
                continue;
            }
            for log in call.logs.iter() {
                let collection_address = &Hex(log.clone().address).to_string();
                let mut collection_exists = false;
                if let Some(_collection) = collections_store.get_last(collection_address) {
                    collection_exists = true
                }

                // if addresses_string.contains(collection_address) {
                //     collection_exists = true
                // }

                if collection_exists {
                    if let Some(event) =
                        abi::collections_v2::events::Transfer::match_and_decode(log)
                    {
                        let transfer = dcl::Transfer {
                            from: Hex(event.from).to_string(),
                            to: Hex(event.to).to_string(),
                            token_id: Some(event.token_id.into()),
                            collection_id: collection_address.clone(),
                            block_timestamp: blk.timestamp_seconds(),
                            tx_hash: Hex(trx.hash.clone()).to_string(),
                            log_index: log.index,
                        };
                        substreams::log::info!("Transfer: {:?}", transfer);
                        transfers.push(transfer);
                    }
                }
            }
        }
    }
    Ok(dcl::Transfers { transfers })
}

// Reads the collection creation by the `ProxyCreated` event
#[substreams::handlers::map]
pub fn map_land_transfers(
    blk: eth::Block,
) -> Result<dcl::LandTransfers, substreams::errors::Error> {
    let mut addresses_string = vec![];
    addresses_string.push(String::from("0cfF059845c6Abee7De396d00091016ad72FD324"));
    addresses_string.push(String::from("25b6B4bac4aDB582a0ABd475439dA6730777Fbf7"));
    let mut addresses = vec![];
    match hex::decode("0cfF059845c6Abee7De396d00091016ad72FD324") {
        Ok(decoded) => addresses.push(decoded),
        Err(_err) => log::debug!("Err decoding address {}"),
    }
    match hex::decode("25b6B4bac4aDB582a0ABd475439dA6730777Fbf7") {
        Ok(decoded) => addresses.push(decoded),
        Err(_err) => log::debug!("Err decoding address {}"),
    }

    let mut addresses_ref = vec![];
    for address in &addresses {
        // substreams::log::info!("address {:?}", Hex(address).to_string());
        addresses_ref.push(address.as_slice());
    }
    for trx in blk.transactions() {
        for call in trx.calls.iter() {
            let _call_index = call.index;
            if call.state_reverted {
                continue;
            }
            for log in call.logs.iter() {
                let address = &Hex(log.clone().address).to_string();
                if addresses_string.contains(address) {
                    substreams::log::info!("exist!");
                } else {
                    substreams::log::info!("address: {:?}", address);
                }
            }
        }
    }

    Ok(dcl::LandTransfers {
        land_transfers: blk
            .events::<abi::lands::events::Transfer1>(&[&LANDS_CONTRACT])
            .map(|(event, _log)| dcl::LandTransfer {
                from: Hex(event.from).to_string(),
                to: Hex(event.to).to_string(),
                asset_id: Some(event.asset_id.into()),
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn db_out(
    nfts: dcl::NfTs,
    transfers: dcl::Transfers,
    collections: dcl::Collections,
    items: dcl::Items,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut database_changes: DatabaseChanges = Default::default();
    // log::info!("In db out nfts {:?}", nfts);
    db::transform_nfts_database_changes(&mut database_changes, nfts);
    // log::info!("In db out transfers {:?}", transfers);
    db::transform_transfers_database_changes(&mut database_changes, transfers);
    // log::info!("In db out collections {:?}", collections);
    db::transform_collection_database_changes(&mut database_changes, collections);
    // log::info!("In db out items {:?}", items);
    db::transform_item_database_changes(&mut database_changes, items);
    Ok(database_changes)
}
