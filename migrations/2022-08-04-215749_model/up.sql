
CREATE TABLE admin (
        id SERIAL PRIMARY KEY,
        username VARCHAR(255) NOT NULL,
        password VARCHAR(255) NOT NULL,
        accessToken VARCHAR(255) NOT NULL
    );

CREATE TABLE host (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    accessToken VARCHAR(255) NOT NULL,
    lastMeetingAt VARCHAR(255) NOT NULL
);

CREATE TABLE privacy (
    id SERIAL PRIMARY KEY,
    body VARCHAR(255) NOT NULL
);