use diesel::prelude::*;
use diesel_belongs_to_bug::{models::*, schema::*};
use dotenv::dotenv;
use std::env;

fn find_merchant_by_id(connection: &SqliteConnection, id: i32) -> QueryResult<Merchant> {
    merchants::table
        .filter(merchants::id.eq(id))
        .first(connection)
}

fn find_account_by_id(connection: &SqliteConnection, id: i32) -> QueryResult<Account> {
    accounts::table
        .filter(accounts::id.eq(id))
        .first(connection)
}

fn find_category_by_id(connection: &SqliteConnection, id: i32) -> QueryResult<Category> {
    categories::table
        .filter(categories::id.eq(id))
        .first(connection)
}

fn main() {
    let connection = establish_connection();

    create_account(&connection, 1, "Checking");
    create_account(&connection, 2, "Savings");
    create_account(&connection, 3, "Visa");

    create_category(&connection, 1, "Groceries");
    create_category(&connection, 2, "Utilities");

    create_merchant(&connection, 1, "Whole Foods");
    create_merchant(&connection, 2, "Trader Joes");
    create_merchant(&connection, 3, "Town Water");
    create_merchant(&connection, 4, "Gas Provider");
    create_merchant(&connection, 5, "Electricity Provider");

    // checking, groceries, Whole Foods and Trader Joes
    create_transaction(&connection, 1, 1, 1, 500);
    create_transaction(&connection, 1, 1, 2, 1500);

    // Visa, utilities, water/gas/electricity
    create_transaction(&connection, 3, 2, 3, 500);
    create_transaction(&connection, 3, 2, 4, 2500);
    create_transaction(&connection, 3, 2, 5, 1500);

    let merchants: Vec<Merchant> = merchants::table.load(&connection).unwrap();

    let transactions = Transaction::belonging_to(&merchants)
        .load::<Transaction>(&connection)
        .unwrap()
        .grouped_by(&merchants);

    let data = merchants.into_iter().zip(transactions).collect::<Vec<_>>();

    for (merchant, merchant_transactions) in data {
        println!(
            "{} {} {}",
            merchant.id,
            merchant.name,
            merchant_transactions.len(),
        );
    }

    assert_merchant_and_transaction_count(&connection, 1, 1);
    assert_merchant_and_transaction_count(&connection, 2, 1);
    assert_merchant_and_transaction_count(&connection, 3, 1);
    assert_merchant_and_transaction_count(&connection, 4, 1);
    assert_merchant_and_transaction_count(&connection, 5, 1);

    assert_account_and_transaction_count(&connection, 1, 2);
    assert_account_and_transaction_count(&connection, 2, 0);
    assert_account_and_transaction_count(&connection, 3, 3);

    assert_category_and_transaction_count(&connection, 1, 2);
    assert_category_and_transaction_count(&connection, 2, 3);
}

fn assert_merchant_and_transaction_count(
    connection: &SqliteConnection,
    id: i32,
    transaction_count: usize,
) {
    let single_merchant = find_merchant_by_id(&connection, id).unwrap();
    let single_merchant_transactions = Transaction::belonging_to(&single_merchant)
        .load::<Transaction>(connection)
        .unwrap()
        .grouped_by(&[single_merchant]);
    assert_eq!(1, single_merchant_transactions.len());
    assert_eq!(transaction_count, single_merchant_transactions[0].len());
}

fn assert_account_and_transaction_count(
    connection: &SqliteConnection,
    id: i32,
    transaction_count: usize,
) {
    let single_account = find_account_by_id(&connection, id).unwrap();
    let single_account_transactions = Transaction::belonging_to(&single_account)
        .load::<Transaction>(connection)
        .unwrap()
        .grouped_by(&[single_account]);
    assert_eq!(1, single_account_transactions.len());
    assert_eq!(transaction_count, single_account_transactions[0].len());
}

fn assert_category_and_transaction_count(
    connection: &SqliteConnection,
    id: i32,
    transaction_count: usize,
) {
    let single_category = find_category_by_id(&connection, id).unwrap();
    let single_category_transactions = Transaction::belonging_to(&single_category)
        .load::<Transaction>(connection)
        .unwrap()
        .grouped_by(&[single_category]);
    assert_eq!(1, single_category_transactions.len());
    assert_eq!(transaction_count, single_category_transactions[0].len());
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_category(conn: &SqliteConnection, id: i32, name: &str) -> QueryResult<usize> {
    diesel::insert_into(categories::table)
        .values(NewCategory { id, name })
        .execute(conn)
}

fn create_merchant(conn: &SqliteConnection, id: i32, name: &str) -> QueryResult<usize> {
    diesel::insert_into(merchants::table)
        .values(NewMerchant { id, name })
        .execute(conn)
}

fn create_account(conn: &SqliteConnection, id: i32, name: &str) -> QueryResult<usize> {
    diesel::insert_into(accounts::table)
        .values(NewAccount { id, name })
        .execute(conn)
}

fn create_transaction(
    conn: &SqliteConnection,
    account_id: i32,
    category_id: i32,
    merchant_id: i32,
    amount: i32,
) -> QueryResult<usize> {
    diesel::insert_into(transactions::table)
        .values(NewTransaction {
            account_id,
            category_id,
            merchant_id,
            amount,
        })
        .execute(conn)
}
