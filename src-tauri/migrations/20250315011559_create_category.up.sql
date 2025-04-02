-- Add up migration script here
CREATE TABLE main_category (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE, 
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP, 
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
INSERT INTO main_category (name) VALUES 
    ("食費"), 
    ("日用品費"), 
    ("交際費"), 
    ("趣味費"), 
    ("交通費"), 
    ("医療費"), 
    ("教育費"), 
    ("雑費");
CREATE TABLE sub_category (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL, 
    super_category INTEGER NOT NULL, 
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP, 
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP, 
    FOREIGN KEY (super_category) REFERENCES main_category(id)
);
ALTER TABLE sub_category ADD UNIQUE (super_category, name);
INSERT INTO sub_category (name, super_category) VALUES 
    ("その他", 1), 
    ("食料品", 1), 
    ("外食", 1), 
    ("コンビニ", 1), 
    ("その他", 2), 
    ("台所用品等", 2), 
    ("文具", 2), 
    ("その他", 3), 
    ("交遊", 3), 
    ("贈り物", 3), 
    ("その他", 4), 
    ("ポケカ", 4), 
    ("本", 4), 
    ("その他", 5), 
    ("Suica", 5), 
    ("駐輪", 5), 
    ("その他", 6), 
    ("内科", 6), 
    ("歯科", 6), 
    ("その他", 7), 
    ("大学関連", 7), 
    ("その他", 8);