CREATE TABLE IF NOT EXISTS bots (
    id INTEGER UNIQUE PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT UNIQUE NOT NULL,
    token TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT 'A bot',
    prefix TEXT NOT NULL DEFAULT '!'
);

CREATE TABLE IF NOT EXISTS code_pieces (
    id INTEGER UNIQUE PRIMARY KEY AUTOINCREMENT NOT NULL,
    code TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS commands (
    id INTEGER UNIQUE PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT UNIQUE NOT NULL,
    bot_id INTEGER REFERENCES bots(id) NOT NULL,
    code_id INTEGER REFERENCES code_pieces(id) DEFAULT NULL,
    description TEXT NOT NULL DEFAULT 'A command'
);

CREATE TABLE IF NOT EXISTS settings (
    current_bot INTEGER REFERENCES bots(id) DEFAULT NULL
);
INSERT INTO settings DEFAULT VALUES;