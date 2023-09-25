create table if not exists user (
  id text primary key not null,
  email text not null unique,
  password_hash text not null
);