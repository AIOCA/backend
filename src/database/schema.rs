table! {
    admins (user_id) {
        user_id -> Nullable<Integer>,
    }
}

table! {
    bookings (booking_id) {
        booking_id -> Integer,
        user_id -> Nullable<Integer>,
    }
}

table! {
    users (user_id) {
        user_id -> Nullable<Integer>,
        user_name -> Varchar,
        password -> Varchar,
        email -> Varchar,
        display_name -> Varchar,
        address -> Text,
    }
}

joinable!(admins -> users (user_id));
joinable!(bookings -> users (user_id));

allow_tables_to_appear_in_same_query!(admins, bookings, users,);
