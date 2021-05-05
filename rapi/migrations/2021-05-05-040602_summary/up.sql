-- Your SQL goes here
CREATE TABLE treatment_summary (
  id SERIAL PRIMARY KEY,
  treatment_id INTEGER NOT NULL,
  date DATE NOT NULL,
  max_point INTEGER DEFAULT NULL,
  mode_point INTEGER DEFAULT NULL,
  FOREIGN KEY (treatment_id) references treatment(id)
)
;