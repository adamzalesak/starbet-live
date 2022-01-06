-- User
CREATE TABLE "User" (
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    civil_id_number TEXT NOT NULL,
    email TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    photo TEXT
);

-- User's address
CREATE TABLE "UserAddress" (
    id SERIAL PRIMARY KEY,
    "user_id" INTEGER REFERENCES "User" NOT NULL,
    street_name TEXT NOT NULL,
    city TEXT NOT NULL,
    area TEXT NOT NULL,
    postal_code TEXT NOT NULL,
    country TEXT NOT NULL,
    valid_from TEXT NOT NULL
);

-- Game -> Like a category of matches
CREATE TABLE "Game" (
    id SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL,
    logo TEXT NOT NULL
);

-- Team which plays the game
CREATE TABLE "Team" (
    id SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    logo TEXT NOT NULL
);

-- teams can play multiple games, games have multiple teams that play them
CREATE TABLE "TeamPlaysGame" (
    id SERIAL PRIMARY KEY,
    team_id INTEGER REFERENCES "Team",
    game_id INTEGER REFERENCES "Game"
);

-- Match that is played
CREATE TABLE "Match" (
    id SERIAL PRIMARY KEY,
    game_id INTEGER REFERENCES "Game",
    team_one_id INTEGER REFERENCES "Team",
    team_two_id INTEGER REFERENCES "Team",
    team_one_ratio TEXT NOT NULL,
    team_two_ration TEXT NOT NULL,
    supposed_start_at TEXT NOT NULL,
    "state" TEXT NOT NULL
);

-- Events that can happen during the match
CREATE TYPE MATCH_EVENTS AS ENUM (
    'MATCH_UPCOMING',
    'MATCH_LIVE',
    'MATCH_END',
    'MATCH_OVERTIME',
    'MATCH_CANCELLED'
);

-- match has events which can happen
CREATE TABLE "MatchEvent" (
    id SERIAL PRIMARY KEY,
    match_id INTEGER REFERENCES "Match",
    event_type MATCH_EVENTS NOT NULL,
    created_at TEXT NOT NULL
);

-- Ticket containing multiple bets
CREATE TABLE "Ticket" (
    id SERIAL PRIMARY KEY,
    "user_id" INTEGER REFERENCES "User",
    created_at TEXT NOT NULL,
    paid_at TEXT
);

-- Bet containing the 
CREATE TABLE "Bet" (
    id SERIAL PRIMARY KEY,
    match_id INTEGER REFERENCES "Match",
    ticket_id INTEGER REFERENCES "Ticket",
    team_id INTEGER REFERENCES "Team",
    bet_ratio TEXT NOT NULL,
    bet_price TEXT NOT NULL,
    created_at TEXT NOT NULL
);
