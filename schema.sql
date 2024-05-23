-- Table for balance changes --
CREATE TABLE IF NOT EXISTS balance_changes  (
    "id"            String,
    block_number    UInt32(),
    timestamp       DateTime64(3, 'UTC'),
    contract        FixedString(40),
    owner           FixedString(40),
    transferValue   UInt256,
    oldBalance      UInt256,
    newBalance      UInt256,
    transaction     FixedString(64),
)
ENGINE = MergeTree PRIMARY KEY ("id")
ORDER BY (id,timestamp, block_number);

-- Indexes for block_number --
ALTER TABLE balance_changes ADD INDEX balance_changes_block_number_index block_number TYPE minmax;

-- MV for contract --
CREATE MATERIALIZED VIEW mv_balance_changes_contract
ENGINE = MergeTree()
ORDER BY (contract, owner)
POPULATE
AS SELECT * FROM balance_changes;

-- MV for owner --
CREATE MATERIALIZED VIEW mv_balance_changes_owner
ENGINE = MergeTree()
ORDER BY (owner, contract)
POPULATE
AS SELECT * FROM balance_changes;
