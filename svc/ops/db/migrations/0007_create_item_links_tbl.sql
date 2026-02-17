CREATE TABLE item_links
(
    id                              UUID             PRIMARY KEY,
    name                            VARCHAR(64)      NOT NULL,
    source_item_tmpl_id             UUID,
    source_item_id                  UUID             NOT NULL,
    target_item_id                  UUID             NOT NULL,

    CONSTRAINT item_links_tmpl_links_tmpl_fk  FOREIGN KEY(name, source_item_tmpl_id)   REFERENCES item_template_links(name, source_item_tmpl_id),
    CONSTRAINT item_links_source_item_fk      FOREIGN KEY(source_item_id)              REFERENCES items(id),
    CONSTRAINT item_links_target_item_fk      FOREIGN KEY(target_item_id)              REFERENCES items(id)
);

COMMENT ON COLUMN item_links.source_item_tmpl_id is 'Optionally, the template id of this item link.';

