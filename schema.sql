CREATE TABLE IF NOT EXISTS balance_changes  (
    contract FixedString(40),
    owner Nullable(String),
    old_balance Nullable(String),
    new_balance Nullable(String),
    transaction_id Nullable(String),
)
ENGINE = MergeTree()
ORDER BY (contract)
