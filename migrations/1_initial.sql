create table if not exists user (
  id integer primary key autoincrement,
  email text not null unique,
  password_hash text not null
);