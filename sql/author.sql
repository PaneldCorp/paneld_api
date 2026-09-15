Create table authors
(
    id uuid primary key unique not null,
    japanese_name varchar(255) not null unique,
    name varchar(255) not null unique,
    website varchar(255) null,
    description text null
);
