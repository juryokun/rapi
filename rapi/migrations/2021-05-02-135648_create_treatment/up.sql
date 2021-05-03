CREATE TABLE treatment (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
)
;

CREATE TABLE action (
  id SERIAL PRIMARY KEY,
  treatment_id INTEGER NOT NULL,
  name VARCHAR NOT NULL,
  FOREIGN KEY (treatment_id) references treatment(id)
)
;

CREATE TABLE command (
  id SERIAL PRIMARY KEY,
  action_id INTEGER NOT NULL,
  name VARCHAR NOT NULL,
  option INTEGER DEFAULT NULL,
  FOREIGN KEY (action_id) references action(id)
)
;

CREATE TABLE timing (
  id SERIAL PRIMARY KEY,
  action_id INTEGER NOT NULL,
  name VARCHAR NOT NULL,
  FOREIGN KEY (action_id) references action(id)
)
;

CREATE TABLE treatment_history (
  id SERIAL PRIMARY KEY,
  action_id INTEGER NOT NULL REFERENCES action(id),
  command_id INTEGER NOT NULL REFERENCES command(id),
  timing_id INTEGER NOT NULL REFERENCES timing(id),
  date DATE NOT NULL,
  option INTEGER DEFAULT NULL
)
;