-- Table Definition ----------------------------------------------

CREATE TABLE billing (
    id integer GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    generated_at timestamp with time zone NOT NULL,
    items text
);
