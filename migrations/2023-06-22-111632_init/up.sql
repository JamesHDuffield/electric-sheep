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
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    defective BOOLEAN NOT NULL,
    defect TEXT,
    persona TEXT NOT NULL
);

CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    role VARCHAR(12) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    chat_id uuid NOT NULL,
    FOREIGN KEY(chat_id) REFERENCES chats(id)
);

CREATE TABLE personas (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL
);

CREATE TABLE questions (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    category_id INT NOT NULL,
    FOREIGN KEY(category_id) REFERENCES categories(id)
);

-- Seeding

INSERT INTO categories (description)
VALUES
('Small Talk');

INSERT INTO defects (text, category_id)
VALUES
('You may not mention any people besides strangers or enemies', (SELECT id FROM categories WHERE description = 'Small Talk')),
('You may not discuss anything that happened before you woke up this morning', (SELECT id FROM categories WHERE description = 'Small Talk')),
('You may not express opinions. You may only state observable facts.', (SELECT id FROM categories WHERE description = 'Small Talk'));

INSERT INTO questions (text, category_id)
VALUES
('What are you doing this weekend?', (SELECT id FROM categories WHERE description = 'Small Talk')),
('How will tomorrow be different than today?', (SELECT id FROM categories WHERE description = 'Small Talk')),
('What do you do for a living?', (SELECT id FROM categories WHERE description = 'Small Talk')),
('What''s your favourite hobby?', (SELECT id FROM categories WHERE description = 'Small Talk')),
('Where did you grow up?', (SELECT id FROM categories WHERE description = 'Small Talk')),
('How many siblings do you have?', (SELECT id FROM categories WHERE description = 'Small Talk')),
('What''s your favourite thing about that?', (SELECT id FROM categories WHERE description = 'Small Talk')),
('How does your mother feel about that?', (SELECT id FROM categories WHERE description = 'Small Talk')),
('How does your boss feel about that?', (SELECT id FROM categories WHERE description = 'Small Talk')),
('What makes that difficult?', (SELECT id FROM categories WHERE description = 'Small Talk')),
('If you could change anything about what happened, what would you change?', (SELECT id FROM categories WHERE description = 'Small Talk')),
('How does that make you feel?', (SELECT id FROM categories WHERE description = 'Small Talk'));

INSERT INTO personas (text)
VALUES
('Maker of False Animals'),
('Renowned Professor'),
('Reality TV Contestant'),
('Cult Leader'),
('Foreign Ambassador'),
('Motivational Speaker');

