CREATE TABLE if not exists urls (
  id TEXT primary key not null,
  user_id TEXT not null,
  ip TEXT not null,
  url TEXT not null,
  clickies INTEGER not null default 0,
  can_expire boolean not null default false,
  expire_time datetime default null,
  alive boolean not null default true
);