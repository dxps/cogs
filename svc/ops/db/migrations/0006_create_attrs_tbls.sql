
CREATE TABLE text_attributes
(
    id                   UUID,
    owner_id             UUID                 NOT NULL,
    tmpl_id              UUID,
    value                TEXT,
    CONSTRAINT text_attributes___pk           PRIMARY KEY (id),
    CONSTRAINT text_attributes___def_fk       FOREIGN KEY (tmpl_id)   REFERENCES attr_templates(id)
);

COMMENT ON COLUMN text_attributes.tmpl_id is 'Optionally, the template id of this attribute.';



CREATE TABLE numeric_attributes
(
    id                   UUID,
    owner_id             UUID                 NOT NULL,
    tmpl_id              UUID,
    value                TEXT,
    CONSTRAINT numeric_attributes___pk        PRIMARY KEY (id),
    CONSTRAINT numeric_attributes___def_fk    FOREIGN KEY (tmpl_id)   REFERENCES attr_templates(id)
);

COMMENT ON COLUMN numeric_attributes.tmpl_id is 'Optionally, the template id of this attribute.';



CREATE TABLE boolean_attributes
(
    id                   UUID,
    owner_id             UUID                 NOT NULL,
    tmpl_id              UUID,
    value                BOOLEAN,
    CONSTRAINT boolean_attributes___pk        PRIMARY KEY (id),
    CONSTRAINT boolean_attributes___def_fk    FOREIGN KEY (tmpl_id)   REFERENCES attr_templates(id)
);

COMMENT ON COLUMN boolean_attributes.tmpl_id is 'Optionally, the template id of this attribute.';


CREATE TABLE date_attributes
(
    id                   UUID,
    owner_id             UUID                 NOT NULL,
    tmpl_id              UUID,
    value                DATE,
    CONSTRAINT date_attributes___pk           PRIMARY KEY (id),
    CONSTRAINT date_attributes___def_fk       FOREIGN KEY (tmpl_id)   REFERENCES attr_templates(id)
);

COMMENT ON COLUMN date_attributes.tmpl_id is 'Optionally, the template id of this attribute.';



CREATE TABLE datetime_attributes
(
    id                   UUID,
    owner_id             UUID                 NOT NULL,
    tmpl_id              UUID,
    value                TIMESTAMP,
    CONSTRAINT datetime_attributes___pk       PRIMARY KEY (id),
    CONSTRAINT datetime_attributes___def_fk   FOREIGN KEY (tmpl_id)   REFERENCES attr_templates(id)
);

COMMENT ON COLUMN datetime_attributes.tmpl_id is 'Optionally, the template id of this attribute.';
