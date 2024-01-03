-- Add migration script here

CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');


CREATE TABLE test_struct (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);



CREATE TABLE more_advanced_struct (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    verified BOOLEAN NOT NULL,
    bio TEXT,
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    small_int_field SMALLINT,             -- 2 bytes, -32768 to +32767
    integer_field INT,                    -- 4 bytes, -2147483648 to +2147483647
    big_int_field BIGINT,                 -- 8 bytes, -9223372036854775808 to +9223372036854775807
    float_field FLOAT(24),                -- 4 bytes, single precision floating point number
    double_field DOUBLE PRECISION,        -- 8 bytes, double precision floating point number
    --numeric_field NUMERIC(10, 2),       -- Arbitrary precision number, 10 digits with 2 decimal places
    char_field CHAR(10),                  -- Fixed-length character string
    bytea_field BYTEA,                    -- Binary data
    date_field DATE,                      -- Date without time
    time_field TIME,                      -- Time without date
    timestamp_field TIMESTAMP,            -- Timestamp without time zone
    -- inet_field INET,                      IP address
    uuid_field UUID,                      -- Universally Unique Identifier
    json_field JSON,                       -- JSON data
    jsonb_field JSONB,                    -- Binary JSON data
    int_array_field INT[],                -- Array of integers
    text_array_field TEXT[],              -- Array of text
    -- point_field POINT,                    Geometric point type
    -- money_field MONEY                     Currency amount
    mood_field mood 
);