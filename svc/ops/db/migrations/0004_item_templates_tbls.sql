CREATE TABLE item_templates
(
    id                      BIGINT        PRIMARY KEY,
    name                    VARCHAR(64)   NOT NULL,
    description             VARCHAR(256),
    listing_attr_templ_id   BIGINT        NOT NULL,
    CONSTRAINT item_templ_listing_attr_templ_fk  FOREIGN KEY(listing_attr_templ_id) REFERENCES attr_templates(id)
);

CREATE TABLE item_templates_attr_templates_xref
(
    item_templ_id            BIGINT,
    attr_templ_id            BIGINT,
    show_index               INT2         NOT NULL     CHECK(show_index > 0),
    PRIMARY KEY (item_templ_id, attr_templ_id),
    CONSTRAINT item_templ_fk   FOREIGN KEY(item_templ_id)   REFERENCES item_templates(id) ON DELETE CASCADE,
    CONSTRAINT attr_templ_fk   FOREIGN KEY(attr_templ_id)   REFERENCES attr_templates(id)
);

COMMENT ON COLUMN item_templates_attr_templates_xref.item_templ_id 
        IS 'The template id of the item that has the attribute.';
COMMENT ON COLUMN item_templates_attr_templates_xref.attr_templ_id
        IS 'The template id of the attribute that the referred item template contains.';
