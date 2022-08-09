-- Add migration script here
CREATE TABLE Users(Id int not null primary key, Username int not null , HashedPassword int not null);
CREATE INDEX UsersUsernameIdx on Users(Username);

CREATE TABLE TodoList(Id int not null primary key, OwnerId int not null);
ALTER TABLE TodoListOwner_fk ADD FOREIGN KEY (OwnerId) REFERENCES Users(Id);

CREATE TABLE TodoEntry(Id int not null primary key, Priority int not null, ListId int not null, Value varchar(2048));
ALTER TABLE TodoEntryListId_fk ADD FOREIGN KEY (ListId) REFERENCES TodoList(Id);
