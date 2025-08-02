-- Your SQL goes here
CREATE TABLE users{
    id SERIAL primary KEY,
    email varchar not null,
    first_name varchar not null,
    last_name varchar not null,
    role_id integer not null,
    passsword varchar not null,
    is_verified boolean default false,
    created_at timestamp
    updated_at timestamp null,
    deleted_at timestamp null,
}