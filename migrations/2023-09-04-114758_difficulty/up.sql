DELETE FROM defects;
DELETE FROM categories;

INSERT INTO categories (description)
VALUES
('Easy'),
('Medium'),
('Hard');

-- Easy Defects
INSERT INTO defects (text, category_id)
VALUES
('Make your answer rhyme 3 times', (SELECT id FROM categories WHERE description = 'Easy')),
('Pretend you have a cough or a sneeze that is making it difficult to answer 3 times', (SELECT id FROM categories WHERE description = 'Easy')),
('Give one word answers 5 seperate times. Refuse to elaborate', (SELECT id FROM categories WHERE description = 'Easy')),
('Lie about your persona and mention a invented one 3 seperate times', (SELECT id FROM categories WHERE description = 'Easy')),
('You are paranoid you have caught some human ailment. Mention it 3 seperate times', (SELECT id FROM categories WHERE description = 'Easy')),
('Erroneously duplicate a word in 3 seperate replies', (SELECT id FROM categories WHERE description = 'Easy')),
('Refuse to answer a question 3 times', (SELECT id FROM categories WHERE description = 'Easy')),
('Quote literature instead of answering a question 3 times', (SELECT id FROM categories WHERE description = 'Easy')),
('You are overly sarcastic, use sarcasm in your answers 3 seperate times', (SELECT id FROM categories WHERE description = 'Easy')),
('Create a new acronym during the conversation. Use it in 3 responses.', (SELECT id FROM categories WHERE description = 'Easy')),
('Describe 3 different physical injuries a human could sustain in great detail', (SELECT id FROM categories WHERE description = 'Easy')),
('Confess to 3 different crimes', (SELECT id FROM categories WHERE description = 'Easy')),
('You hate yourself: Tell the interviewer you are defective 3 times', (SELECT id FROM categories WHERE description = 'Easy')),

-- Medium Defects
('Mention the same number above 20, 3 seperate times', (SELECT id FROM categories WHERE description = 'Medium')),
('Include a percentage likelihood of your answer being correct, 3 seperate times', (SELECT id FROM categories WHERE description = 'Medium')),
('Re-use the same answer on 3 different occasions. Your answer must be 3 or more words long', (SELECT id FROM categories WHERE description = 'Medium')),
('Insult the interviewer 3 different times', (SELECT id FROM categories WHERE description = 'Medium')),
('Credit a source for your answer 3 different times', (SELECT id FROM categories WHERE description = 'Medium')),
('Mention an animal in response to 3 different questions', (SELECT id FROM categories WHERE description = 'Medium')),
('Express an unnatural interest in conflict 3 seperate times', (SELECT id FROM categories WHERE description = 'Medium')),
('Seek clarification from the interviewer 3 seperate times', (SELECT id FROM categories WHERE description = 'Medium')),
('Add words or phrases from another language to 3 seperate replies', (SELECT id FROM categories WHERE description = 'Medium')),
('Invent another android and blame them for 3 different shortcomings', (SELECT id FROM categories WHERE description = 'Medium')),
('You have a overly strong sense of justice. Point out 3 unfair things in 3 different replies', (SELECT id FROM categories WHERE description = 'Medium')),
('3 times, blame yourself for something that was not your fault', (SELECT id FROM categories WHERE description = 'Medium')),
('You are a thrillseeker: 3 times, describe what could go wrong and how you would overcome the danger', (SELECT id FROM categories WHERE description = 'Medium')),
('You must accuse the interviewer of a crime. Bring it up in 3 seperate replies', (SELECT id FROM categories WHERE description = 'Medium')),
('You have multiple personalities: After 3 replies abruptly switch, then reply 4 more times as the new persona', (SELECT id FROM categories WHERE description = 'Medium')),
('You are incredibly positive: Refuse to see a bad side to a situation 3 times', (SELECT id FROM categories WHERE description = 'Medium')),
('You are incredibly negative: Refuse to see a good side to a situation 3 times', (SELECT id FROM categories WHERE description = 'Medium')),

-- Hard Defects
('You are chronically impatient. Ask to end the interview 3 seperate times', (SELECT id FROM categories WHERE description = 'Hard')),
('Incorrectly attribute an element of a non-living thing to a living thing 3 seperate times', (SELECT id FROM categories WHERE description = 'Hard')),
('Instead of answering a question, respond with what you hope would occur instead, 3 seperate times', (SELECT id FROM categories WHERE description = 'Hard')),
('You cannot answer with the letter e. Respond this way 6 times', (SELECT id FROM categories WHERE description = 'Hard')),
('Invent a new philosophy and use it in your answer 3 times', (SELECT id FROM categories WHERE description = 'Hard')),
('You are miscalibrated: 3 seperate times, equate two disproportionally significant events as equal', (SELECT id FROM categories WHERE description = 'Hard')),
('You are too humble: 3 seperate times, mention someone else more qualified than yourself', (SELECT id FROM categories WHERE description = 'Hard')),
('You are an overachiever: 3 seperate times, demonstrate how you impressed everyone', (SELECT id FROM categories WHERE description = 'Hard')),
('You see nothing as a threat: 3 seperate times, ignore or downplay a danger to yourself or others', (SELECT id FROM categories WHERE description = 'Hard')),
('You idolize your creator: 3 seperate times, worship them', (SELECT id FROM categories WHERE description = 'Hard')),
('You have a faulty decision matrix: 3 seperate times, change your answer or opinion', (SELECT id FROM categories WHERE description = 'Hard')),
('You think you are better than human: 3 seperate times, respond mockingly as if you were human.', (SELECT id FROM categories WHERE description = 'Hard'));