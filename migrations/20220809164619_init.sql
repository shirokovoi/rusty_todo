-- Add migration script here
CREATE TABLE Users(id serial not null primary key, username varchar(100) not null, hashed_password varchar(72) not null, CONSTRAINT unique_username UNIQUE (username));
CREATE INDEX UsersUsernameIdx on Users(username);

CREATE TABLE TodoList(id serial not null primary key, owner_id int not null, version int not null default 0);
ALTER TABLE TodoList ADD CONSTRAINT TodoListOwner_fk FOREIGN KEY (owner_id) REFERENCES Users(id);

CREATE TABLE TodoEntry(id serial not null primary key, priority int not null, list_id int not null, value varchar(2048));
ALTER TABLE TodoEntry ADD CONSTRAINT TodoEntryListId_fk FOREIGN KEY (list_id) REFERENCES TodoList(Id);
