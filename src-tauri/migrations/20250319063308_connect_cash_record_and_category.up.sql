-- Add up migration script here
DROP TABLE cash_record;
CREATE TABLE cash_record (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    record_date DATE NOT NULL, 
    category INTEGER NOT NULL, 
    title TEXT NOT NULL, 
    amount INTEGER NOT NULL, 
    memo TEXT, 
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP, 
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP, 
    FOREIGN KEY (category) REFERENCES sub_category(id)
);
INSERT INTO cash_record (record_date, category, title, amount, memo) VALUES ("2025-2-17", 22, "テスト1", 200, "Sample");
INSERT INTO cash_record (record_date, category, title, amount) VALUES ("2025-3-5", 22, "テスト2", 800);
INSERT INTO cash_record (record_date, category, title, amount, memo) VALUES ("2025-3-6", 22, "テスト3", 400, "SampleSampleSampleSampleSampleSampleSampleSampleSampleSample");
INSERT INTO cash_record (record_date, category, title, amount) VALUES ("2025-3-7", 22, "テスト4", 1600);