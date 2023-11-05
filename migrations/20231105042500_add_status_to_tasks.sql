-- Add migration script here
ALTER TABLE tasks ADD COLUMN status INT NOT NULL DEFAULT 0;
