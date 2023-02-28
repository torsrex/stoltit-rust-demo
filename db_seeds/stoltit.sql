-- 
-- Schema and types for stoltit
--


CREATE TABLE IF NOT EXISTS public."stoltit"
(
    name text not null,
    year bigint,
    value double precision
);

INSERT INTO stoltit(name, year, value) VALUES('person1', 2022, 10);
INSERT INTO stoltit(name, year, value) VALUES('person2', 2023, 15);
INSERT INTO stoltit(name, year, value) VALUES('person3', 2019, 5);