-- Your SQL goes here

INSERT INTO "user"(id, first_name, last_name, user_password, civil_id_number, date_of_birth, email, phone_number, created_at, balance, photo)
VALUES (0, 'admin', '-', 'admin', 'ER000000', '1970-01-01UTC', 'admin@starbet.live','+420000000000', '1970-01-01UTC', '0.0', NULL);


INSERT INTO "user_address"(user_id, street_name, street_number, city, area, postal_code, country, valid_from)
VALUES (0, 'Botanicka', '68a', 'Brno', NULL, '60200', 'Czech republic', '1970-01-01UTC');