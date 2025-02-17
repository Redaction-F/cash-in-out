-- Add up migration script here
CREATE TABLE cash_record (
    id INTEGER PRIMARY KEY,
    record_date date NOT NULL, 
    category TEXT, 
    title TEXT NOT NULL, 
    amount INTEGER NOT NULL, 
    memo TEXT, 
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);