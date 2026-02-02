CREATE TABLE IF NOT EXISTS books (
    id SERIAL PRIMARY KEY,
    title VARCHAR(225) NOT NULL,
    author VARCHAR(50),
    published_date DATE,
    stock INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW()
)