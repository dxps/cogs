CREATE TABLE items
(
    id                                     UUID                  PRIMARY KEY,
    tmpl_id                                UUID,
    listing_attr_tmpl_id                   UUID                  NOT NULL,
    listing_attr_name                      VARCHAR(64)           NOT NULL,
    listing_attr_value                     VARCHAR(64),

    CONSTRAINT items_tmpl_fk               FOREIGN KEY(tmpl_id)              REFERENCES item_templates(id),
    CONSTRAINT items_listing_attr_tmpl_fk  FOREIGN KEY(listing_attr_tmpl_id) REFERENCES attr_templates(id)
);

COMMENT ON COLUMN items.tmpl_id is 'Optionally, the template id of this item.';

