DROP TABLE posts CASCADE;
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(128) NOT NULL,
    content TEXT NOT NULL,
    author INTEGER NOT NULL,
    flags bigint NOT NULL DEFAULT 0,
    created TIMESTAMP WITH TIME ZONE DEFAULT now()
);
