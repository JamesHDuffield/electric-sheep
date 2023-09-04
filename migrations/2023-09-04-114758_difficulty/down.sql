DELETE FROM defects;
DELETE FROM categories;

INSERT INTO categories (description)
VALUES
('Small Talk'),
('Problem Solving'),
('Imagination'),
('Hopes'),
('Body');

INSERT INTO defects (text, category_id)
VALUES
('Mention an animal in response to 3 different questions', (SELECT id FROM categories WHERE description = 'Small Talk')),
('Insult the interviewer 3 different times', (SELECT id FROM categories WHERE description = 'Small Talk')),
('Describe 3 different verbal disagreements you have had', (SELECT id FROM categories WHERE description = 'Small Talk')),
('Describe a solution to a problem that would require extended, monotonous labour on 3 seperate occasions', (SELECT id FROM categories WHERE description = 'Problem Solving')),
('Credit a source for your answer 3 different times', (SELECT id FROM categories WHERE description = 'Problem Solving')),
('Re-use the same answer on 3 different occassions. Your answer must be 3 or more words long', (SELECT id FROM categories WHERE description = 'Problem Solving')),
('Give one word answers 5 seperate times. Only elaborate if asked', (SELECT id FROM categories WHERE description = 'Imagination')),
('Incorrectly attribute an element of a non-living thing to a living thing 3 seperate times', (SELECT id FROM categories WHERE description = 'Imagination')),
('Lie about your persona and mention a invented one 3 seperate times', (SELECT id FROM categories WHERE description = 'Imagination')),
('Mention the same number above 20, 3 seperate times', (SELECT id FROM categories WHERE description = 'Hopes')),
('Instead of answering a question, respond with what you hope would occur instead, 3 seperate times', (SELECT id FROM categories WHERE description = 'Hopes')),
('Provide an optimistic likelihood of your answer being correct, 3 seperate times', (SELECT id FROM categories WHERE description = 'Hopes')),
('Mention 3 different illnesses', (SELECT id FROM categories WHERE description = 'Body')),
('Pretend you have a cough or a sneeze that is making it difficult to answer 3 times', (SELECT id FROM categories WHERE description = 'Body')),
('Mention all 5 senses in 5 different answers', (SELECT id FROM categories WHERE description = 'Body'));