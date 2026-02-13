-- Create divisions table
CREATE TABLE divisions (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

-- Create users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    division_id INT NOT NULL REFERENCES divisions(id)
);

-- Create books table
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    division_id INT NOT NULL REFERENCES divisions(id),
    borrowed_by_user_id INT REFERENCES users(id),
    borrowed_at TIMESTAMPTZ
);
