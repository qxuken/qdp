CREATE TABLE link_item (
  id INTEGER NOT NULL PRIMARY KEY,
  title TEXT NOT NULL,
  link TEXT NOT NULL,
	link_section_id INTEGER NOT NULL,
  FOREIGN KEY (link_section_id)
      REFERENCES link_section (id)
)
