-- Your SQL goes here
CREATE TYPE PartOfSpeech as ENUM ('ADJ', 'ADP', 'ADV', 'AUX', 'CCONJ', 'DET', 'INTJ', 'NOUN', 'NUM', 'PART', 'PRON', 'PROPN', 'PUNCT', 'SCONJ', 'SYM', 'VERB', 'X');
CREATE TYPE WordRelationship as ENUM('def', 'related');

CREATE TABLE Words (
  norm VARCHAR(255) PRIMARY KEY, -- normalized word
  native VARCHAR(255),
  lemma VARCHAR(255)
    REFERENCES Words(norm)
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
  wordsource VARCHAR(255)
    REFERENCES Words(norm)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL,
  wordtarget VARCHAR(255)
    REFERENCES Words(norm)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    NOT NULL,
  relationship WordRelationship NOT NULL
);
