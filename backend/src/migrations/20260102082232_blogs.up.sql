-- Add migration script here
CREATE TABLE IF NOT EXISTS blogs (
	id serial NOT NULL,
	title VARCHAR(30) NOT NULL,
	
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	update_at TIMESTAMP,
	delete_at TIMESTAMP,
	
	PRIMARY KEY (id)
);