-- ...

-- CLIENTS
CREATE TABLE clients (
  dbentry_id SERIAL PRIMARY KEY,
  client_id INT NOT NULL
);

-- ENGINES
CREATE TABLE engines (
  dbentry_id SERIAL PRIMARY KEY,
  client_id INT NOT NULL,
  engine_id INT NOT NULL
);

-- STATES
CREATE TABLE states (
  dbentry_id SERIAL PRIMARY KEY,
  engine_id INT NOT NULL,
  state_id INT NOT NULL
);

-- SYSTEMS
CREATE TABLE systems (
  dbentry_id SERIAL PRIMARY KEY,
  engine_id INT NOT NULL,
  state_id INT NOT NULL,
  system_id INT NOT NULL,
  system_variant_id INT NOT NULL
);

-- Entities v1
CREATE TABLE entities_v1 (
  dbentry_id SERIAL PRIMARY KEY,
  engine_id INT NOT NULL,
  state_id INT NOT NULL,
  system_id INT NOT NULL,
  entity_id INT NOT NULL,
  -- physical attributes
  mass    FLOAT NOT NULL,
  pos_x   FLOAT NOT NULL,
  pos_y   FLOAT NOT NULL,
  pos_z   FLOAT NOT NULL,
  vel_x   FLOAT NOT NULL,
  vel_y   FLOAT NOT NULL,
  vel_z   FLOAT NOT NULL
);

-- PLANETS
/* CREATE TABLE planets ( */
/*   dbentry_id SERIAL PRIMARY KEY, */
/*   engine_id INT NOT NULL, */
/*   state_id INT NOT NULL, */
/*   system_id INT NOT NULL, */
/*   planet_id INT NOT NULL, */
/*   -- physical attributes */
/*   mass    FLOAT NOT NULL, */
/*   pos_x   FLOAT NOT NULL, */
/*   pos_y   FLOAT NOT NULL, */
/*   pos_z   FLOAT NOT NULL, */
/*   vel_x   FLOAT NOT NULL, */
/*   vel_y   FLOAT NOT NULL, */
/*   vel_z   FLOAT NOT NULL */
/* ); */
