CREATE TABLE access_levels
(
    id              UUID               PRIMARY KEY  DEFAULT uuidv7(),
    name            VARCHAR(64)        NOT NULL     UNIQUE,
    description     VARCHAR(256),
    read_only       BOOLEAN            NOT NULL     DEFAULT false
);

INSERT INTO access_levels (name, description, read_only)
VALUES
    ('Public', 'Publicly visible', true),
    ('Private', 'Private access needed', true),
    ('Confidential', 'A more restricted access', true);
