// @generated automatically by Diesel CLI.

diesel::table! {
    couriers (user_uuid) {
        user_uuid -> Uuid,
        is_free -> Nullable<Bool>,
        rating -> Nullable<Float8>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (user_uuid) {
        user_uuid -> Uuid,
        first_name -> Varchar,
        address -> Nullable<Varchar>,
        phone_number -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Varchar,
        is_blocked -> Bool,
        is_deleted -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(couriers -> users (user_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    couriers,
    users,
);
