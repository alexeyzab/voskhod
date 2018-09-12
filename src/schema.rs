table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        referral_code -> Varchar,
        referrer_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
