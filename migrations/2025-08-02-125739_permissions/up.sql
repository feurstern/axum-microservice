-- Your SQL goes here
CREATE TABLE permissions(
    id serial PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL,
    --view_report, edit_invetory,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL
);