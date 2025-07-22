CREATE TABLE user_accounts (
    id              CHAR(10)                 PRIMARY KEY                ,
    name            VARCHAR(128)             NOT NULL                   ,
    email           VARCHAR(64)              NOT NULL     UNIQUE        ,
    username        VARCHAR(48)              NOT NULL     UNIQUE        ,
    password        VARCHAR(255)             NOT NULL                   ,
    salt            CHAR(12)                 NOT NULL                   ,
    bio             TEXT                                  DEFAULT ''    ,
    state           CHAR(1)                  NOT NULL     DEFAULT 'A'   ,
    is_anonymous    BOOLEAN                  NOT NULL     DEFAULT FALSE
);
