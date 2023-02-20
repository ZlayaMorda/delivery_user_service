-- Your SQL goes here

CREATE EXTENSION "uuid-ossp";

CREATE TABLE users (
    uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
    first_name VARCHAR(64) NOT NULL,
    address VARCHAR(128),
    phone_number VARCHAR(16) NOT NULL,
    email VARCHAR(256) NOT NULL,
    password VARCHAR(100) NOT NULL,
    role VARCHAR(8) NOT NULL DEFAULT ('user'),
    is_blocked BOOL NOT NULL DEFAULT (FALSE),
    is_deleted BOOL NOT NULL DEFAULT (FALSE),
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (uuid),
    UNIQUE (phone_number, email)
);

CREATE TABLE couriers (
    uuid UUID NOT NULL,
    is_free BOOL DEFAULT (FALSE),
    rating DOUBLE PRECISION,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    FOREIGN KEY (uuid) REFERENCES users (uuid) ON DELETE CASCADE,
    PRIMARY KEY (uuid)
)