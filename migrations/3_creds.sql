create table if not exists credentials (
    id integer primary key autoincrement not null,
    name text not null,
    token text not null
)