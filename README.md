# Leviosa

This is a PostgreSQL helper crate built on  top of SQLX and provides an efficient and easy-to-use interface for working with PostgreSQL databases in Rust. It leverages procedural macros to automate common database operations reducing boilerplate.
This is not an ORM but a light helper libary. 

## Features

- **Automatic CRUD Operations**: Generate `create`, `read`, `update`, and `delete` functions for your structs.
- **Advanced Query Building**: Currently `find` .
- **Type-Safe Interface**: Strong typing for database fields to ensure data integrity.

## Getting Started
`git clone https://github.com/tie304/leviosa.git`

To use this crate in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
leviosa = { path = "/PATH_TO_CLONED_CRATE" }
chrono = "0.4.31"
rust_decimal = "1.33.1"
uuid = { version = "1.6.1", features = ["v4"] } 
serde_json = "1.0.108"
sqlx = { version = "0.7.3", features = [ "runtime-tokio", "tls-native-tls", "postgres", "time", "chrono", "bigdecimal", "uuid" ] }
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0.75"
bigdecimal =  { version = "0.3.0", features = ["serde"]}
tokio = { version = "1", features = ["full"] }

```
```rust
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::error::Error;

use leviosa::leviosa;

#[leviosa]
#[derive(Debug, FromRow)]
struct MyStruct { // all tables are snake case: my_struct
    id: Option<i32>, // create by default ignores all optional fields
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:mysecretpassword@localhost:5432/postgres")
        .await?;

    // omits all optional fields
    let mut my_struct = MyStruct::create(&pool, String::from("Harry")).await?;

    // updates in place.
    my_struct.update_name(&pool, &String::from("Ron")).await?;

    //deletes in place.
    my_struct.delete(&pool).await?;

    Ok(())
}
```
### Run sqlx migrations
`sqlx migrate add init`

Add your table to the migration

```sql
CREATE TABLE my_struct (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);
```
`sqlx migrate run`

`cargo run`
## Roadmap

Here's a glimpse of what we plan to roll out in future updates:

- **Support for Additional Types**: Support for more data types to enhance compatibility and flexibility with various PostgreSQL data formats such as `NUMERIC`

- **Batch Operations (`create_many`, `update_many`, ~~`delete_many`~~)**: To improve efficiency and performance, we are working on implementing batch operations. These will allow users to perform create, update, and delete operations on multiple records simultaneously, making bulk data handling much more streamlined.

- **Transactional Support for Batch Operations**:  Upcoming batch operations will be designed to run within database transactions. This ensures that either all operations succeed, or none do, maintaining data consistency and reliability.

- **Enhanced Query Building Capabilities**: We plan to further develop our query builder to support more complex queries, giving users more power and flexibility in data manipulation and retrieval.

- **Performance Optimizations and Refinements**: Continuous performance enhancements and code optimizations are a key part of our roadmap, ensuring that the crate remains efficient and scalable.

Stay tuned for these updates, and feel free to contribute or suggest any features you'd like to see in future releases.
