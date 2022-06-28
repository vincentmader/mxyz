CREATE TABLE planets (
  dbentry_id SERIAL PRIMARY KEY,
  step_id INT NOT NULL,
  planet_id INT NOT NULL,
  system_id INT NOT NULL,
  -- physical attributes
  mass    FLOAT NOT NULL,
  pos_x   FLOAT NOT NULL,
  pos_y   FLOAT NOT NULL,
  pos_z   FLOAT NOT NULL,
  vel_x   FLOAT NOT NULL,
  vel_y   FLOAT NOT NULL,
  vel_z   FLOAT NOT NULL
);
CREATE TABLE fluid_cells (
  dbentry_id SERIAL PRIMARY KEY,
  step_id INT NOT NULL,
  cell_id INT NOT NULL,
  system_id INT NOT NULL,
  -- physical attributes
  density FLOAT NOT NULL,
  vel_x   FLOAT NOT NULL,
  vel_y   FLOAT NOT NULL,
  vel_z   FLOAT NOT NULL
);
