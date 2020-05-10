drop table if exists list_item;
drop table if exists list_list;

create table list_list (
    id serial primary key,
    title varchar(150) not null
);

create table list_item (
    id serial primary key,
    title varchar(150) not null,
    list_id integer not null,
    foreign key (list_id) references list_list(id)
);

insert into list_list(title)
values ('List 1'), ('List 2');

insert into list_item(title, list_id)
values ('Item 1', 1), ('Item 2', 1), ('Item 1', 2);