CREATE TABLE user_permissions
(
    user_id         BIGINT        NOT NULL,
    permission      VARCHAR(256)  NOT NULL,
    UNIQUE (user_id, permission),
    CONSTRAINT user_fk FOREIGN KEY(user_id) REFERENCES user_accounts(id)
);

