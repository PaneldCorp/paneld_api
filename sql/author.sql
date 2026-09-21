Create table authors
(
    id uuid primary key unique not null,
    japanese_name varchar(255) not null unique,
    name varchar(255) not null unique,
    birthday date not null,
    website varchar(255) null,
    description text null,
    pub_date date not null,
    update_date date not null
);
