create table urls (
    id          text primary key not null,
    user_id     text             not null,
    ip          text             not null,
    url         text             not null,
    clickies    integer          not null default 0,
    can_expire  boolean          not null default false,
    expire_time timestamptz               default null,
    alive       boolean          not null default true
);
