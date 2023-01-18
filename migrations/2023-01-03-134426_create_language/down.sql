-- This file should undo anything in `up.sql`
DROP TABLE UserFollowLanguage;
DROP TABLE LangAndAgents;
DROP TABLE LangTranslatesTo;
DROP TABLE Languages;
DROP TYPE Release;
DROP TYPE DictGenre;
DROP TYPE AgentLanguageRelation;
DROP EXTENSION "uuid-ossp";
