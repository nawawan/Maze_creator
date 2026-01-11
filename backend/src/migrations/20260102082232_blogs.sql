-- Add migration script here
create table blogs (
	id serial not null,
	title VARCHAR(30) not null,
	
	created_at timestamp ,
	update_at timestamp,
	delete_at timestamp,
	
	primary key (id)
);