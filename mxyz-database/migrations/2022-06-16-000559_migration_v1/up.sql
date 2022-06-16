CREATE TABLE planets (
  planet_id SERIAL PRIMARY KEY,
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
  cell_id SERIAL PRIMARY KEY,
  system_id INT NOT NULL,
  -- physical attributes
  density FLOAT NOT NULL,
  vel_x   FLOAT NOT NULL,
  vel_y   FLOAT NOT NULL,
  vel_z   FLOAT NOT NULL
);
