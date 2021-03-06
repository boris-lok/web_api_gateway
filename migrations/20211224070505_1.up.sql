-- Add up migration script here

create table users
(
    id         uuid          not null primary key,
    name       varchar(64)   not null unique,
    password   varchar(1024) not null,
    role       smallint      not null,
    created_at timestamptz,
    updated_at timestamptz
);

create index idx_users_name on users using btree (name);