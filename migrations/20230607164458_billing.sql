-- Table Definition ----------------------------------------------

CREATE TABLE billing (
    id integer PRIMARY KEY,
    generated_at timestamp with time zone NOT NULL,
    items text
);
