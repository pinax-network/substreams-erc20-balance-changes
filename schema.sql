-- Table for balance changes --
CREATE TABLE IF NOT EXISTS BalanceChange  (
    chain           LowCardinality(String),
    block_number    UInt32(),
    timestamp       DateTime64(3, 'UTC'),
    contract        FixedString(40),
    owner           FixedString(40),
    transferValue   UInt256,
    oldBalance      UInt256,
    newBalance      UInt256,
    transaction     FixedString(64),
)
ENGINE = MergeTree()
ORDER BY (timestamp, block_number, chain);

-- Indexes for block_number and chain --
ALTER TABLE BalanceChange ADD INDEX balance_changes_block_number_index block_number TYPE minmax;
ALTER TABLE BalanceChange ADD INDEX balance_changes_chain_index chain TYPE minmax;

-- MV for contract --
CREATE MATERIALIZED VIEW mv_balance_changes_contract
ENGINE = MergeTree()
ORDER BY (contract, owner)
POPULATE
AS SELECT * FROM BalanceChange;

-- MV for owner --
CREATE MATERIALIZED VIEW mv_balance_changes_owner
ENGINE = MergeTree()
ORDER BY (owner, contract)
POPULATE
AS SELECT * FROM BalanceChange;
