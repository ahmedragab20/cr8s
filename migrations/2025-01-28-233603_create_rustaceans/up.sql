CREATE TABLE rustaceans
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR                 NOT NULL UNIQUE,
    email      VARCHAR                 NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
)