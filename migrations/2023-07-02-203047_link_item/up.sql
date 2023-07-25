CREATE TABLE link_item (
  id INTEGER NOT NULL PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  link TEXT NOT NULL,
  order_number INTEGER NOT NULL,
	link_section_id INTEGER NOT NULL,
  FOREIGN KEY (link_section_id)
      REFERENCES link_section (id)
)
