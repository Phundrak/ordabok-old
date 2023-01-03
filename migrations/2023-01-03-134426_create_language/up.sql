-- Your SQL goes here
CREATE TYPE Release as ENUM ('PUBLIC', 'NONCOMMERCIAL', 'RESEARCH', 'PRIVATE');
CREATE TYPE DictGenre as ENUM ('gen', 'lrn', 'ety', 'spe', 'his', 'ort', 'trm');
CREATE TYPE AgentLanguageRelation as ENUM ('publisher', 'author');

CREATE TABLE Languages (
  name VARCHAR(255) PRIMARY KEY,
  native VARCHAR(255),
  release Release NOT NULL,
  targetLanguage TEXT[] NOT NULL,
  genre DictGenre[] NOT NULL,
  abstract TEXT,
  created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  description TEXT,
  rights TEXT,
  license TEXT,
  owner VARCHAR(31)
    REFERENCES Users(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL
);

CREATE TABLE LangAndAgents (
  id SERIAL PRIMARY KEY,
  agent VARCHAR(31)
    REFERENCES Users(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL,
  language VARCHAR(255)
    REFERENCES Languages(name)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL,
  relationship AgentLanguageRelation NOT NULL
);
