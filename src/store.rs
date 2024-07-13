use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt};
use crate::pb::erc20::types::v1::{BalanceChanges};
use substreams::store::StoreNew;

#[substreams::handlers::store]
pub fn store_valid_balance_changes(balance_changes: BalanceChanges, store: StoreAddBigInt) {
    let mut type0_counter : u64 = 0;
    let mut type1_counter : u64 = 0;
    let mut type2_counter : u64 = 0;
    let mut total_counter : u64 = 0;

    for change in balance_changes.balance_changes {
        match change.change_type {
            1 => {
                type1_counter += 1;
                total_counter += 1;
            },
            2 => {
                type2_counter += 1;
                total_counter += 1;
            },
            _ => {
                type0_counter += 1;
                total_counter += 1;
            },
        }
    }

    store.add(0, "type1", BigInt::from(type1_counter));
    store.add(0, "type2", BigInt::from(type2_counter));
    store.add(0, "type0", BigInt::from(type0_counter));
    store.add(0, "total", BigInt::from(total_counter));
}