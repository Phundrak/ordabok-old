-- Your SQL goes here
CREATE TYPE PartOfSpeech as ENUM ('ADJ', 'ADP', 'ADV', 'AUX', 'CCONJ', 'DET', 'INTJ', 'NOUN', 'NUM', 'PART', 'PRON', 'PROPN', 'PUNCT', 'SCONJ', 'SYM', 'VERB', 'X');
CREATE TYPE WordRelationship as ENUM('def', 'related');
CREATE TYPE WordLearningStatus as ENUM('learning', 'learned');

CREATE TABLE Words (
  id UUID DEFAULT uuid_generate_v4 () PRIMARY KEY,
  norm VARCHAR(255) NOT NULL, -- normalized word, generally in latin alphabet
  native VARCHAR(255),
  lemma UUID
    REFERENCES Words(id)
    ON UPDATE CASCADE
    ON DELETE SET NULL,
  language UUID
    REFERENCES Languages(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL,
  partofspeech PartOfSpeech NOT NULL,
  audio VARCHAR(511),
  video VARCHAR(511),
  image VARCHAR(511),
  description TEXT, -- Markdown
  etymology TEXT, -- Markdown
  lusage TEXT, -- Markdown
  morphology TEXT -- Markdown
);

CREATE TABLE WordRelation (
  id SERIAL PRIMARY KEY,
  wordsource UUID
    REFERENCES Words(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL,
  wordtarget UUID
    REFERENCES Words(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL,
  relationship WordRelationship NOT NULL
);

CREATE TABLE WordLearning (
  id SERIAL PRIMARY KEY,
  word UUID
    REFERENCES Words(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL,
  userid VARCHAR(31)
    REFERENCES Users(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL,
  status WordLearningStatus DEFAULT 'learning' NOT NULL
);
