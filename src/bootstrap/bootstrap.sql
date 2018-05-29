CREATE TABLE images
(
  id INT PRIMARY KEY,
  filename TEXT,
  created TEXT
);

CREATE TABLE schema_versions
(
  id INT
);

INSERT INTO schema_versions (id) VALUES (1);
