CREATE TABLE species (
    id SERIAL PRIMARY KEY,
    planet VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    population FLOAT NOT NULL,
    traits TEXT[] NOT NULL
);
