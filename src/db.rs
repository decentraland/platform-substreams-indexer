use crate::dcl_hex;
use crate::pb::dcl;
use crate::utils::sanitize_sql_string;
use substreams::prelude::BigInt;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};

pub fn transform_nfts_database_changes(changes: &mut DatabaseChanges, nfts: dcl::NfTs) {
    for nft in nfts.nfts {
        changes
            .push_change(
                String::from("nfts"),
                dcl_hex!(format!(
                    "{}-{}",
                    nft.collection_address,
                    nft.token_id.clone().unwrap().value
                )),
                0,
                table_change::Operation::Create,
            )
            .change(
                "issued_id",
                (
                    None,
                    BigInt::from(nft.issued_id.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change("item_id", (None, nft.item_id))
            .change("collection_id", (None, dcl_hex!(nft.collection_address)))
            .change(
                "token_id",
                (
                    None,
                    BigInt::from(nft.token_id.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change("created_at", (None, nft.created_at))
            .change("updated_at", (None, nft.updated_at))
            .change("owner", (None, dcl_hex!(nft.beneficiary)));
    }
}

pub fn transform_collection_database_changes(
    changes: &mut DatabaseChanges,
    collections: dcl::Collections,
) {
    for collection in collections.collections {
        changes
            .push_change(
                String::from("collections"),
                dcl_hex!(collection.address.clone()),
                0,
                table_change::Operation::Create,
            )
            .change("creator", (None, dcl_hex!(collection.creator)))
            .change("is_approved", (None, collection.is_approved))
            .change("is_completed", (None, collection.is_completed))
            .change("name", (None, sanitize_sql_string(collection.name)))
            .change("symbol", (None, sanitize_sql_string(collection.symbol)))
            .change("owner", (None, dcl_hex!(collection.owner)))
            .change("created_at", (None, collection.created_at));
    }
}

pub fn transform_item_database_changes(changes: &mut DatabaseChanges, items: dcl::Items) {
    for item in items.items {
        changes
            .push_change(
                "items".to_string(),
                item.id.clone(),
                0,
                table_change::Operation::Create,
            )
            .change(
                "blockchain_item_id",
                (
                    None,
                    BigInt::from(item.blockchain_item_id.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change("item_type", (None, item.item_type))
            .change(
                "price",
                (
                    None,
                    BigInt::from(item.price.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change(
                "total_supply",
                (
                    None,
                    BigInt::from(item.total_supply.clone().unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change(
                "available",
                (
                    None,
                    BigInt::from(item.max_supply.clone().unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change(
                "max_supply",
                (
                    None,
                    BigInt::from(item.max_supply.unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change("rarity", (None, item.rarity))
            .change("beneficiary", (None, item.beneficiary))
            .change("name", (None, item.name))
            .change("description", (None, item.description))
            .change("category", (None, item.category))
            .change("body_shapes", (None, item.body_shapes))
            .change("created_at", (None, item.created_at))
            .change("collection_id", (None, dcl_hex!(item.collection_id)));
    }
}

pub fn transform_transfers_database_changes(
    changes: &mut DatabaseChanges,
    transfers: dcl::Transfers,
) {
    for transfer in transfers.transfers {
        changes
            .push_change(
                String::from("transfers"),
                dcl_hex!(format!(
                    "{}-{}",
                    transfer.tx_hash.clone(),
                    transfer.log_index
                )),
                0,
                table_change::Operation::Create,
            )
            .change(
                "collection_id",
                (None, dcl_hex!(transfer.collection_id.clone())),
            )
            .change(
                "token_id",
                (
                    None,
                    BigInt::from(transfer.token_id.clone().unwrap_or(dcl::BigInt {
                        value: String::from("0"),
                    })),
                ),
            )
            .change("created_at", (None, transfer.created_at))
            .change("from_address", (None, dcl_hex!(transfer.from)))
            .change("to_address", (None, dcl_hex!(transfer.to)));

        changes
            .push_change(
                String::from("nfts"),
                dcl_hex!(format!(
                    "{}-{}",
                    transfer.collection_id,
                    transfer.token_id.unwrap().value
                )),
                0,
                table_change::Operation::Update,
            )
            .change("owner", (None, dcl_hex!(transfer.to.clone())))
            .change("updated_at", (None, transfer.created_at));
    }
}

pub fn push_block_database_changes(
    changes: &mut DatabaseChanges,
    block_number: u64,
    timestamp: u64,
) {
    changes
        .push_change(
            "blocks",
            &block_number.to_string(),
            0,
            table_change::Operation::Create,
        )
        .change("block_timestamp", (None, timestamp));
}
