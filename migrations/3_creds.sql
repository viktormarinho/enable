create table if not exists credentials (
    id text primary key not null,
    name text not null,
    token text not null
)