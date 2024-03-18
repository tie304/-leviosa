-- Add migration script here

CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');


CREATE TABLE test_struct (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE test_struct_relation (
    id SERIAL PRIMARY KEY,
    my_data TEXT,
    test_struct INT NOT NULL,
    CONSTRAINT fk_test_struct
        FOREIGN KEY (test_struct)
        REFERENCES test_struct (id)
        ON DELETE CASCADE
);

/*
STRICT ONE TO ONE
CREATE TABLE test_struct (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE test_struct_relation (
    id SERIAL PRIMARY KEY,
    my_data TEXT,
    test_struct INT UNIQUE NOT NULL, -- Notice the UNIQUE constraint here
    CONSTRAINT fk_test_struct
        FOREIGN KEY (test_struct)
        REFERENCES test_struct (id)
        ON DELETE CASCADE
);


CREATE TABLE test_struct (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


// MANY TO MANY

CREATE TABLE test_struct_relation (
    id SERIAL PRIMARY KEY,
    my_data TEXT
    -- Removed direct reference to test_struct
);

*/

CREATE TABLE many_to_many_relation_1 (
    id SERIAL PRIMARY KEY,
    my_data VARCHAR(10)
);

CREATE TABLE many_to_many_relation_2 (
    id SERIAL PRIMARY KEY,
    my_data VARCHAR(10)
);

CREATE TABLE many_to_many_association (
    many_to_many_relation_1_id INT NOT NULL,
    many_to_many_relation_2_id INT NOT NULL,
    PRIMARY KEY (many_to_many_relation_1_id, many_to_many_relation_2_id),
    CONSTRAINT fk_many_to_many_relation_1
        FOREIGN KEY (many_to_many_relation_1_id)
        REFERENCES many_to_many_relation_1(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_many_to_many_relation_2
        FOREIGN KEY (many_to_many_relation_2_id)
        REFERENCES many_to_many_relation_2 (id)
        ON DELETE CASCADE
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