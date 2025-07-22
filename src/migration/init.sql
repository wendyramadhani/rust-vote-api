DROP TABLE IF EXISTS items;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS items_categories;

CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);
CREATE TABLE IF NOT EXISTS items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    votes INTEGER DEFAULT 0,
    category_id INTEGER NOT NULL,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL
);

INSERT INTO items (name, votes, category_id) VALUES ('Item 1', 0,1), ('Item 2', 0,1), ('Item 3', 0,1);
INSERT INTO categories (name) VALUES ('General'), ('Feedback'), ('Suggestions');