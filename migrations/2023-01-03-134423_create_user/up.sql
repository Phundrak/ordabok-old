-- Your SQL goes here

CREATE TABLE Users (
  id VARCHAR(31) PRIMARY KEY, -- Appwrite User ID
  username VARCHAR(64) NOT NULL
);

CREATE TABLE UserFollows (
  id SERIAL PRIMARY KEY,
  follower VARCHAR(31)
    REFERENCES Users(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL,
  following VARCHAR(31)
    REFERENCES Users(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL
);
