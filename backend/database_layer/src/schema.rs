table! {
    bet (id) {
        id -> Int4,
        game_match_id -> Int4,
        ticket_id -> Int4,
        team_id -> Int4,
        bet_ratio -> Text,
        bet_state -> Text,
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
        team_one_id -> Int4,
        team_two_id -> Int4,
        team_one_ratio -> Text,
        team_two_ratio -> Text,
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
        until -> Nullable<Text>,
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
        price -> Text,
        paid_at -> Nullable<Text>,
    }
}

table! {
    user (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        civil_id_number -> Text,
        date_of_birth -> Text,
        email -> Text,
        phone_number -> Text,
        created_at -> Text,
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
joinable!(team_plays_game -> game (game_id));
joinable!(team_plays_game -> team (team_id));
joinable!(ticket -> user (user_id));
joinable!(user_address -> user (user_id));

allow_tables_to_appear_in_same_query!(
    bet,
    game,
    game_match,
    game_match_event,
    team,
    team_plays_game,
    ticket,
    user,
    user_address,
);
