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
    persona TEXT NOT NULL,
    name TEXT NOT NULL
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

CREATE TABLE first_names (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
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

INSERT INTO personas (text)
VALUES
('Maker of False Animals'),
('Renowned Professor'),
('Reality TV Contestant'),
('Foreign Ambassador'),
('Motivational Speaker'),
('Bounty Hunter'),
('Used Van Dealer'),
('War Veteran'),
('Robot Rights Activist'),
('Street Artist'),
('Underground Surgeon'),
('Nightclub DJ'),
('Noise Musician'),
('Digital Dream Interpreter'),
('Augmented Reality Tour Guide'),
('Street Philosopher'),
('Mood Designer'),
('Virtual Idol Manager'),
('Wildlife Tracker'),
('Digital Art Restorer'),
('Meditation Guru'),
('Deep Net Archivist'),
('Quantum Poet'),
('Flavour Chemist'),
('Combat Dummy'),
('Nanny'),
('Chef'),
('Crypto Barista'),
('Digital Doppelganger Creator'),
('Neural Network Stylist'),
('Holo-sign Installer'),
('Sewage Technician'),
('Pest Exterminator'),
('Spa Therapist'),
('Personal Shopper'),
('Sommelier'),
('Masseuse'),
('Dive Bar Bartender'),
('Nutrition Bar Vendor'),
('Vacation Planner'),
('Interpreter');
