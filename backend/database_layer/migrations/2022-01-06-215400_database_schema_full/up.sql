-- User
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    user_password TEXT NOT NULL,
    user_password_salt TEXT NOT NULL,
    civil_id_number TEXT NOT NULL,
    date_of_birth TEXT NOT NULL,
    email TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    created_at TEXT NOT NULL,
    balance TEXT NOT NULL,
    photo TEXT
);

-- User's address
CREATE TABLE "user_address" (
    id SERIAL PRIMARY KEY,
    "user_id" INTEGER REFERENCES "user" NOT NULL,
    street_name TEXT NOT NULL,
    street_number TEXT NOT NULL,
    city TEXT NOT NULL,
    area TEXT,
    postal_code TEXT NOT NULL,
    country TEXT NOT NULL,
    valid_from TEXT NOT NULL
);

-- Game -> Like a category of matches
CREATE TABLE "game" (
    id SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    logo TEXT NOT NULL
);

-- Team which plays the game
CREATE TABLE "team" (
    id SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    logo TEXT NOT NULL
);

-- teams can play multiple games, games have multiple teams that play them
CREATE TABLE "team_plays_game" (
    id SERIAL PRIMARY KEY,
    team_id INTEGER REFERENCES "team" NOT NULL,
    game_id INTEGER REFERENCES "game" NOT NULL
);

-- Match that is played
CREATE TABLE "game_match" (
    id SERIAL PRIMARY KEY,
    game_id INTEGER REFERENCES "game" NOT NULL,
    game_name TEXT NOT NULL,
    team_one_id INTEGER REFERENCES "team" NOT NULL,
    team_one_ratio TEXT NOT NULL,
    team_one_name TEXT NOT NULL,
    team_two_id INTEGER REFERENCES "team" NOT NULL,
    team_two_ratio TEXT NOT NULL,
    team_two_name TEXT NOT NULL,
    supposed_start_at TEXT NOT NULL,
    "state" TEXT NOT NULL
);

-- match has events which can happen
CREATE TABLE "game_match_event" (
    id SERIAL PRIMARY KEY,
    game_match_id INTEGER REFERENCES "game_match" NOT NULL,
    event_type TEXT NOT NULL,
    created_at TEXT NOT NULL,
    event_value TEXT
);

-- Ticket containing multiple bets
CREATE TABLE "ticket" (
    id SERIAL PRIMARY KEY,
    "user_id" INTEGER REFERENCES "user" NOT NULL,
    created_at TEXT NOT NULL,
    valid_until TEXT NOT NULL
);

-- Tickets that have been submitted
CREATE TABLE "submitted_ticket" (
    id SERIAL PRIMARY KEY,
    "user_id" INTEGER REFERENCES "user" NOT NULL,
    submitted_at TEXT NOT NULL,
    price_paid TEXT NOT NULL,
    total_ratio TEXT NOT NULL,
    winnable_price TEXT NOT NULL,
    won BOOLEAN
);

-- bets that have been submitted
CREATE TABLE "submitted_bet" (
    id SERIAL PRIMARY KEY,
    game_match_id INTEGER REFERENCES "game_match" NOT NULL,
    submitted_ticket_id INTEGER REFERENCES "submitted_ticket",
    team_id INTEGER REFERENCES "team" NOT NULL,
    bet_ratio TEXT NOT NULL,
    placed_at TEXT NOT NULL,
    submitted_at TEXT NOT NULL,
    won BOOLEAN
);

-- Bet containing the ratio and the team id 
CREATE TABLE "bet" (
    id SERIAL PRIMARY KEY,
    game_match_id INTEGER REFERENCES "game_match" NOT NULL,
    ticket_id INTEGER REFERENCES "ticket" NOT NULL,
    team_id INTEGER REFERENCES "team" NOT NULL,
    bet_ratio TEXT NOT NULL,
    created_at TEXT NOT NULL
);
