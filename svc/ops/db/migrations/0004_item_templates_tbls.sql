CREATE TABLE IF NOT EXISTS item_templates
(
    id                      UUID          PRIMARY KEY,
    name                    VARCHAR(64)   NOT NULL,
    description             VARCHAR(256),
    listing_attr_tmpl_id    UUID          NOT NULL,

    CONSTRAINT item_tmpl_listing_attr_templ_fk  FOREIGN KEY(listing_attr_tmpl_id) REFERENCES attr_templates(id)
);



CREATE TABLE IF NOT EXISTS item_templates_attr_templates_xref
(
    item_tmpl_id             UUID             NOT NULL,
    attr_tmpl_id             UUID             NOT NULL,
    show_index               INT2             NOT NULL     CHECK(show_index > 0),

    PRIMARY KEY (item_tmpl_id, attr_tmpl_id),
    CONSTRAINT item_tmpl_fk   FOREIGN KEY(item_tmpl_id)   REFERENCES item_templates(id) ON DELETE CASCADE,
    CONSTRAINT attr_tmpl_fk   FOREIGN KEY(attr_tmpl_id)   REFERENCES attr_templates(id),
    CONSTRAINT item_tmpl_attr_show_index_uniq UNIQUE (item_tmpl_id, show_index)
);

COMMENT ON COLUMN item_templates_attr_templates_xref.item_tmpl_id 
        IS 'The template id of the item that has the attribute.';
COMMENT ON COLUMN item_templates_attr_templates_xref.attr_tmpl_id
        IS 'The template id of the attribute that the referred item template contains.';



CREATE TABLE IF NOT EXISTS item_template_links
(
    name                          VARCHAR(64)      NOT NULL,
    source_item_tmpl_id           UUID             NOT NULL,
    target_item_tmpl_id           UUID             NOT NULL,
    show_index                    INT2             NOT NULL CHECK (show_index > 0),

    PRIMARY KEY (name, source_item_tmpl_id),
    CONSTRAINT item_template_links_owner_fk        FOREIGN KEY (source_item_tmpl_id) REFERENCES item_templates(id) ON DELETE CASCADE,
    CONSTRAINT item_template_links_target_fk       FOREIGN KEY (target_item_tmpl_id) REFERENCES item_templates(id),
    CONSTRAINT item_template_links_no_self         CHECK (source_item_tmpl_id <> target_item_tmpl_id),
    CONSTRAINT item_template_links_show_index_uniq UNIQUE (source_item_tmpl_id, show_index)
);

CREATE INDEX idx_item_templ_attr_xref_item ON item_templates_attr_templates_xref(item_tmpl_id);
CREATE INDEX idx_item_templ_links_owner ON item_template_links(source_item_tmpl_id);
CREATE INDEX idx_item_templ_links_target ON item_template_links(target_item_tmpl_id);

