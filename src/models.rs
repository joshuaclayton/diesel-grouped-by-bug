use crate::schema::*;

#[derive(Debug, Identifiable, Queryable)]
pub struct Merchant {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Identifiable, Queryable)]
pub struct Account {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Associations, Clone, Debug, Queryable)]
#[belongs_to(Category)]
#[belongs_to(Merchant)]
#[belongs_to(Account)]
pub struct Transaction {
    pub id: i32,
    pub account_id: i32,
    pub merchant_id: i32,
    pub category_id: i32,
    pub amount: i32,
}

#[derive(Insertable)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub id: i32,
    pub name: &'a str,
}

#[derive(Insertable)]
#[table_name = "merchants"]
pub struct NewMerchant<'a> {
    pub id: i32,
    pub name: &'a str,
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub id: i32,
    pub name: &'a str,
}

#[derive(Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub account_id: i32,
    pub category_id: i32,
    pub merchant_id: i32,
    pub amount: i32,
}
