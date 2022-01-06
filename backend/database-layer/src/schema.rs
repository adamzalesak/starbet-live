table! {
    Bet (id) {
        id -> Int4,
        match_id -> Nullable<Int4>,
        ticket_id -> Nullable<Int4>,
        team_id -> Nullable<Int4>,
        bet_ratio -> Text,
        bet_price -> Text,
        created_at -> Text,
    }
}

table! {
    Game (id) {
        id -> Int4,
        name -> Text,
        logo -> Text,
    }
}

table! {
    Match (id) {
        id -> Int4,
        game_id -> Nullable<Int4>,
        team_one_id -> Nullable<Int4>,
        team_two_id -> Nullable<Int4>,
        team_one_ratio -> Text,
        team_two_ration -> Text,
        supposed_start_at -> Text,
        state -> Text,
    }
}

table! {
    MatchEvent (id) {
        id -> Int4,
        match_id -> Nullable<Int4>,
        event_type -> Text,
        created_at -> Text,
    }
}

table! {
    Team (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        logo -> Text,
    }
}

table! {
    TeamPlaysGame (id) {
        id -> Int4,
        team_id -> Nullable<Int4>,
        game_id -> Nullable<Int4>,
    }
}

table! {
    Ticket (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        created_at -> Text,
        paid_at -> Nullable<Text>,
    }
}

table! {
    User (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        civil_id_number -> Text,
        email -> Text,
        phone_number -> Text,
        photo -> Nullable<Text>,
    }
}

table! {
    UserAddress (id) {
        id -> Int4,
        user_id -> Int4,
        street_name -> Text,
        city -> Text,
        area -> Text,
        postal_code -> Text,
        country -> Text,
        valid_from -> Text,
    }
}

joinable!(Bet -> Match (match_id));
joinable!(Bet -> Team (team_id));
joinable!(Bet -> Ticket (ticket_id));
joinable!(Match -> Game (game_id));
joinable!(MatchEvent -> Match (match_id));
joinable!(TeamPlaysGame -> Game (game_id));
joinable!(TeamPlaysGame -> Team (team_id));
joinable!(Ticket -> User (user_id));
joinable!(UserAddress -> User (user_id));

allow_tables_to_appear_in_same_query!(
    Bet,
    Game,
    Match,
    MatchEvent,
    Team,
    TeamPlaysGame,
    Ticket,
    User,
    UserAddress,
);
