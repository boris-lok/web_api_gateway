-- Add up migration script here

create table users
(
    id         uuid         not null primary key,
    name       varchar(64)  not null,
    password   varchar(256) not null,
    role       smallint     not null,
    created_at timestamptz,
    updated_at timestamptz
);

create index idx_users_name on users using btree (name);

create table tokens
(
    user_id    uuid         not null primary key,
    token      varchar(512) not null,
    expired_at timestamptz  not null
);