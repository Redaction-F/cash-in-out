-- Add up migration script here
DROP TABLE cash_record;
CREATE TABLE cash_record (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    record_date DATE NOT NULL, 
    category INTEGER NOT NULL, 
    title TEXT NOT NULL, 
    amount INTEGER NOT NULL, 
    memo TEXT NOT NULL, 
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP, 
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP, 
    FOREIGN KEY (category) REFERENCES sub_category(id)
);