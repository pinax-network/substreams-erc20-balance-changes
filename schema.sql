CREATE TABLE IF NOT EXISTS balance_changes  (
    contract FixedString(40),
    owner String,
    old_balance String,
    new_balance String,
    transaction_id String,
)
ENGINE = MergeTree()
ORDER BY (transaction_id)
