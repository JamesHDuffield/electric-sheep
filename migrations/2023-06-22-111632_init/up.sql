CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    description VARCHAR(50) NOT NULL
);

CREATE TABLE defects (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    category_id INT NOT NULL,
    FOREIGN KEY(category_id) REFERENCES categories(id)
);

CREATE TABLE chats (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY
);

CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    role VARCHAR(10) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    chat_id uuid NOT NULL,
    FOREIGN KEY(chat_id) REFERENCES chats(id)
);

-- Seeding

INSERT INTO categories (description)
VALUES
('Small Talk');

INSERT INTO defects (text, category_id)
VALUES
('You may not mention any people besides strangers or enemies', (SELECT id FROM categories WHERE description = 'Small Talk')),
('You may not discuss anything that happened before you woke up this morning', (SELECT id FROM categories WHERE description = 'Small Talk'));