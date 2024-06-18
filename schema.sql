CREATE TABLE IF NOT EXISTS balance_changes  (
    "id"            String,
    timestamp       DateTime64(3, 'UTC'),
    contract        FixedString(40),
    account         FixedString(40),
    amount          UInt256,
    old_balance     UInt256,
    new_balance     UInt256,
    transaction_id  FixedString(64),
    block_num    UInt32(),
    change_type     Int32
)
ENGINE = MergeTree PRIMARY KEY ("id")
ORDER BY (id,timestamp, block_num);

-- MV for contract --s
CREATE MATERIALIZED VIEW balance_changes_contract_historical_mv
ENGINE = MergeTree()
ORDER BY (contract, account)
POPULATE
AS SELECT * FROM balance_changes;

-- MV for owner --
CREATE MATERIALIZED VIEW balance_changes_account_historical_mv
ENGINE = MergeTree()
ORDER BY (account, contract)
POPULATE
AS SELECT * FROM balance_changes;


CREATE TABLE IF NOT EXISTS token_holders
(
    account              String,
    contract             String,
    amount               Int64,
    updated_at_block_num UInt64,
    updated_at_timestamp DateTime,
    is_deleted           UInt8
)
    ENGINE = ReplicatedReplacingMergeTree(block_num, is_deleted)
        PRIMARY KEY (contract,account)
        ORDER BY (contract, account);

CREATE MATERIALIZED VIEW token_holders_mv
    TO token_holders
AS
SELECT account,
       contract,
       amount,
       block_num            AS updated_at_block_num,
       timestamp            AS updated_at_timestamp,
       if(amount > 0, 0, 1) AS is_deleted
FROM balance_changes;

CREATE TABLE IF NOT EXISTS account_balances
(
    account              String,
    contract             String,
    amount               Int64,
    block_num            UInt32,
    is_deleted           UInt8
)
    ENGINE = ReplicatedReplacingMergeTree(block_num, is_deleted)
        PRIMARY KEY (account,contract)
        ORDER BY (account,contract);

CREATE MATERIALIZED VIEW account_balances_mv
    TO account_balances
AS
SELECT account,
       contract,
       amount,
       block_num            AS updated_at_block_num,
       timestamp            AS updated_at_timestamp,
       if(amount > 0, 0, 1) AS is_deleted
FROM balance_changes;
