-- Table for balance changes --
CREATE TABLE IF NOT EXISTS balance_changes  (
    "id"            String,
    timestamp       DateTime64(3, 'UTC'),
    contract        FixedString(40),
    owner           FixedString(40),
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
ORDER BY (contract, owner,block_num)
POPULATE
AS SELECT * FROM balance_changes;

-- MV for owner --
CREATE MATERIALIZED VIEW balance_changes_account_historical_mv
ENGINE = MergeTree()
ORDER BY (owner, contract,block_num)
POPULATE
AS SELECT * FROM balance_changes;


CREATE TABLE IF NOT EXISTS token_holders
(
    account              FixedString(40),
    contract             String,
    amount               UInt256,
    block_num            UInt32(),
    timestamp            DateTime64(3, 'UTC'),
    tx_id                FixedString(64)
)
    ENGINE = ReplacingMergeTree(block_num)
        PRIMARY KEY (contract,account)
        ORDER BY (contract, account);

CREATE MATERIALIZED VIEW token_holders_mv
    TO token_holders
AS
SELECT owner as account,
       contract,
       new_balance as amount,
       block_num,
       timestamp,
       transaction_id as tx_id
FROM balance_changes;

CREATE TABLE IF NOT EXISTS account_balances
(
    account              FixedString(40),
    contract             String,
    amount               UInt256,
    block_num            UInt32(),
    timestamp            DateTime64(3, 'UTC'),
    tx_id                FixedString(64)
)
    ENGINE = ReplacingMergeTree(block_num)
        PRIMARY KEY (account,contract)
        ORDER BY (account,contract);

CREATE MATERIALIZED VIEW account_balances_mv
    TO account_balances
AS
SELECT owner as account,
       contract,
       amount,
       block_num, 
       timestamp,
       transaction_id as tx_id
FROM balance_changes;