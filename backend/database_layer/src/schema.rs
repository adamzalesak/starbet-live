table! {
    bet (id) {
        id -> Int4,
        game_match_id -> Int4,
        ticket_id -> Int4,
        team_id -> Int4,
        bet_ratio -> Text,
        created_at -> Text,
    }
}

table! {
    game (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        logo -> Text,
    }
}

table! {
    game_match (id) {
        id -> Int4,
        game_id -> Int4,
        game_name -> Text,
        team_one_id -> Int4,
        team_one_ratio -> Text,
        team_one_name -> Text,
        team_two_id -> Int4,
        team_two_ratio -> Text,
        team_two_name -> Text,
        supposed_start_at -> Text,
        state -> Text,
    }
}

table! {
    game_match_event (id) {
        id -> Int4,
        game_match_id -> Int4,
        event_type -> Text,
        created_at -> Text,
        event_value -> Nullable<Text>,
    }
}

table! {
    submitted_bet (id) {
        id -> Int4,
        game_match_id -> Int4,
        submitted_ticket_id -> Nullable<Int4>,
        team_id -> Int4,
        bet_ratio -> Text,
        placed_at -> Text,
        submitted_at -> Text,
        won -> Nullable<Bool>,
    }
}

table! {
    submitted_ticket (id) {
        id -> Int4,
        user_id -> Int4,
        submitted_at -> Text,
        price_paid -> Text,
        total_ratio -> Text,
        winnable_price -> Text,
        won -> Nullable<Bool>,
    }
}

table! {
    team (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        logo -> Text,
    }
}

table! {
    team_plays_game (id) {
        id -> Int4,
        team_id -> Int4,
        game_id -> Int4,
    }
}

table! {
    ticket (id) {
        id -> Int4,
        user_id -> Int4,
        created_at -> Text,
        valid_until -> Text,
    }
}

table! {
    user (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        user_password -> Text,
        user_password_salt -> Text,
        civil_id_number -> Text,
        date_of_birth -> Text,
        email -> Text,
        phone_number -> Text,
        created_at -> Text,
        balance -> Text,
        photo -> Nullable<Text>,
    }
}

table! {
    user_address (id) {
        id -> Int4,
        user_id -> Int4,
        street_name -> Text,
        street_number -> Text,
        city -> Text,
        area -> Nullable<Text>,
        postal_code -> Text,
        country -> Text,
        valid_from -> Text,
    }
}

joinable!(bet -> game_match (game_match_id));
joinable!(bet -> team (team_id));
joinable!(bet -> ticket (ticket_id));
joinable!(game_match -> game (game_id));
joinable!(game_match_event -> game_match (game_match_id));
joinable!(submitted_bet -> game_match (game_match_id));
joinable!(submitted_bet -> submitted_ticket (submitted_ticket_id));
joinable!(submitted_bet -> team (team_id));
joinable!(submitted_ticket -> user (user_id));
joinable!(team_plays_game -> game (game_id));
joinable!(team_plays_game -> team (team_id));
joinable!(ticket -> user (user_id));
joinable!(user_address -> user (user_id));

allow_tables_to_appear_in_same_query!(
    bet,
    game,
    game_match,
    game_match_event,
    submitted_bet,
    submitted_ticket,
    team,
    team_plays_game,
    ticket,
    user,
    user_address,
);
