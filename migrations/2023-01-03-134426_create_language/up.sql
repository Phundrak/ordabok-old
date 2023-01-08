-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE Release as ENUM ('PUBLIC', 'NONCOMMERCIAL', 'RESEARCH', 'PRIVATE');
CREATE TYPE DictGenre as ENUM ('gen', 'lrn', 'ety', 'spe', 'his', 'ort', 'trm');
CREATE TYPE AgentLanguageRelation as ENUM ('publisher', 'author');

CREATE TABLE Languages (
  id UUID DEFAULT uuid_generate_v4 () PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  native VARCHAR(255),
  release Release NOT NULL,
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

CREATE TABLE LangTranslatesTo (
  id SERIAL PRIMARY KEY,
  langfrom UUID
    REFERENCES Languages(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL,
  langto UUID
    REFERENCES Languages(id)
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
  language UUID
    REFERENCES Languages(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL,
  relationship AgentLanguageRelation NOT NULL
);
