-- Add down migration script here

drop index if exists idx_users_name;
drop table if exists users;
drop table if exists tokens;