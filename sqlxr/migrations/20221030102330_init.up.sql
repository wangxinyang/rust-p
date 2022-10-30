CREATE SCHEMA tr;

CREATE TABLE tr.course (
  id UUID NOT NULL DEFAULT gen_random_uuid(),
  teacher_id int4 NOT NULL,
  name varchar(255) NOT NULL,
  time date DEFAULT now(),
  timespan TSTZRANGE NOT NULL,

  CONSTRAINT course_pkey PRIMARY KEY (id)
);