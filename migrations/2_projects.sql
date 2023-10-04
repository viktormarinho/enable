create table if not exists project (
  id text primary key not null,
  name text not null,

  user_id text not null,
  foreign key(user_id) references user(id)
);

create table if not exists environment (
  id text primary key not null,
  name text not null,

  project_id text not null,
  foreign key(project_id) references project(id)
);

create table if not exists feature (
  id text primary key not null, -- not and uuid yet.

  project_id text not null,
  foreign key(project_id) references project(id)
);

create table if not exists environment_feature (
  id text primary key not null,
  active boolean not null,
  
  feature_id text not null,
  environment_id text not null,

  foreign key(feature_id) references feature(id),
  foreign key(environment_id) references environment(id)
);