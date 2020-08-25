CREATE TABLE categories (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE merchants (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE accounts (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE UNIQUE INDEX index_categories_name ON categories (name);
CREATE TABLE transactions (
    id INTEGER NOT NULL PRIMARY KEY,
    category_id INTEGER NOT NULL,
    merchant_id INTEGER NOT NULL,
    account_id INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    CONSTRAINT fk_categories FOREIGN KEY (category_id) REFERENCES categories(id),
    CONSTRAINT fk_merchants FOREIGN KEY (merchant_id) REFERENCES merchants(id),
    CONSTRAINT fk_accounts FOREIGN KEY (account_id) REFERENCES accounts(id)
);

CREATE INDEX index_transactions_category_id ON transactions(category_id);
CREATE INDEX index_transactions_merchant_id ON transactions(merchant_id);
CREATE INDEX index_transactions_account_id ON transactions(account_id);
