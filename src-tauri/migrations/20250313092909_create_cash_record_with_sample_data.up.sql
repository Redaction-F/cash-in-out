-- Add up migration script here
CREATE TABLE cash_record (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    record_date date NOT NULL, 
    category TEXT, 
    title TEXT NOT NULL, 
    amount INTEGER NOT NULL, 
    memo TEXT, 
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO cash_record (id, record_date, category, title, amount, memo) VALUES (1, "2025-2-17", "その他", "テスト1", 200, "Sample");
INSERT INTO cash_record (record_date, category, title, amount) VALUES ("2025-3-5", "その他", "テスト2", 800);
INSERT INTO cash_record (record_date, category, title, amount, memo) VALUES ("2025-3-6", "その他", "テスト3", 400, "SampleSampleSampleSampleSampleSampleSampleSampleSampleSample");
INSERT INTO cash_record (record_date, category, title, amount) VALUES ("2025-3-7", "その他", "テスト4", 1600);