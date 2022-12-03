-- Add migration script here
INSERT INTO users (user_id, username, password_hash) 
VALUES (
    '131d37f3-6ddc-406c-911d-bc0d71bd14b6',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$n6K5p/EkAISDwH+jb/MbSg$Bnf31m8JensY2YK9VXzwnaK+W7RfRiK7LeJZY8kZl5g'
);