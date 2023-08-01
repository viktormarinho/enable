create table if not exists project (
  id integer primary key autoincrement,
  name text not null,
  user_id integer not null,
  foreign key(user_id) references user(id)
);

create table if not exists feature (
    id text primary key not null,
    active boolean not null,
    project_id integer not null,
    foreign key(project_id) references project(id)
);