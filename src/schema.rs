table! {
    accounts (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    categories (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    merchants (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    transactions (id) {
        id -> Integer,
        category_id -> Integer,
        merchant_id -> Integer,
        account_id -> Integer,
        amount -> Integer,
    }
}

joinable!(transactions -> accounts (account_id));
joinable!(transactions -> categories (category_id));
joinable!(transactions -> merchants (merchant_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    categories,
    merchants,
    transactions,
);
