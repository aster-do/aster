create type billable_operation as enum('Add','Subtract', 'Multiply', 'Divide');

create table billable_rule (
    id serial primary key,
    name varchar(255) unique not null,
    operation billable_operation not null,
    number int not null
);