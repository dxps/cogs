CREATE TABLE attr_templates (
    id              BIGINT                   PRIMARY KEY       ,
    name            VARCHAR(128)             NOT NULL          ,
    description     VARCHAR(256)                               ,
    value_type      VARCHAR(16)   NOT NULL                     ,
    default_value   VARCHAR(20)                                ,
    required        BOOLEAN       DEFAULT false                ,
    CONSTRAINT name_desc_unique UNIQUE NULLS NOT DISTINCT (name, description)
);
