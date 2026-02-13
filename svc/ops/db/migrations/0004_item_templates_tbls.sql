CREATE TABLE IF NOT EXISTS item_templates
(
    id                      UUID          PRIMARY KEY,
    name                    VARCHAR(64)   NOT NULL,
    description             VARCHAR(256),
    listing_attr_templ_id   UUID          NOT NULL,
    CONSTRAINT item_templ_listing_attr_templ_fk  FOREIGN KEY(listing_attr_templ_id) REFERENCES attr_templates(id)
);

CREATE TABLE IF NOT EXISTS item_templates_attr_templates_xref
(
    item_templ_id            UUID,
    attr_templ_id            UUID,
    show_index               INT2         NOT NULL     CHECK(show_index > 0),
    PRIMARY KEY (item_templ_id, attr_templ_id),
    CONSTRAINT item_templ_fk   FOREIGN KEY(item_templ_id)   REFERENCES item_templates(id) ON DELETE CASCADE,
    CONSTRAINT attr_templ_fk   FOREIGN KEY(attr_templ_id)   REFERENCES attr_templates(id),
    CONSTRAINT item_templ_attr_show_index_uniq UNIQUE (item_templ_id, show_index)
);

COMMENT ON COLUMN item_templates_attr_templates_xref.item_templ_id 
        IS 'The template id of the item that has the attribute.';
COMMENT ON COLUMN item_templates_attr_templates_xref.attr_templ_id
        IS 'The template id of the attribute that the referred item template contains.';

CREATE TABLE IF NOT EXISTS item_template_links
(
    item_templ_id         UUID         NOT NULL,
    link_name             VARCHAR(64)  NOT NULL,
    target_item_templ_id  UUID         NOT NULL,
    show_index            INT2         NOT NULL CHECK (show_index > 0),
    PRIMARY KEY (item_templ_id, link_name),
    CONSTRAINT item_template_links_owner_fk  FOREIGN KEY (item_templ_id) REFERENCES item_templates(id) ON DELETE CASCADE,
    CONSTRAINT item_template_links_target_fk FOREIGN KEY (target_item_templ_id) REFERENCES item_templates(id),
    CONSTRAINT item_template_links_no_self CHECK (item_templ_id <> target_item_templ_id),
    CONSTRAINT item_template_links_show_index_uniq UNIQUE (item_templ_id, show_index)
);

CREATE INDEX idx_item_templ_attr_xref_item ON item_templates_attr_templates_xref(item_templ_id);
CREATE INDEX idx_item_templ_links_owner ON item_template_links(item_templ_id);
CREATE INDEX idx_item_templ_links_target ON item_template_links(target_item_templ_id);

