# Stage 2

__Persistence (models & connection)__

In backend development projects, after the API contract is established, the database design is often the very next task to complete. The database design determines what and how information is imported and stored for repeated usage. While designing the database (what technologies to use, how to model it, etc.), we can very quickly assess if the project is a feasible idea and if we can meet most requirements.

In this project, we will persist Question and Answer records in PostgreSQL. The tables are structured as follows:

### Question

| Name          | Type         | Description                                  |
|---------------|--------------|----------------------------------------------|
| question_uuid | UUID         | Generated identifier unique to each question |
| title         | VARCHAR(255) | Title of the question                        |
| description   | VARCHAR(255) | Description of the question                  |
| created_at    | TIMESTAMP    | Creation timestamp of the question           |

### Answer

| Name          | Type         | Description                                  |
|---------------|--------------|----------------------------------------------|
| answer_uuid   | UUID         | Generated identifier unique to each answer   |
| question_uuid | UUID         | Generated identifier unique to each question |
| content       | VARCHAR(255) | Content of the answer                        |
| created_at    | TIMESTAMP    | Creation timestamp of the answer             |

__NOTE:__ The SQL queries for creating and dropping tables are included in the template.

## Third Party Libraries

A few additional dependencies have been added to help us integrate our code with PostgreSQL.

### SQLx

[SQLx](https://github.com/launchbadge/sqlx) is an async SQL driver featuring compile-time checked queries. We will use this library to execute SQL queries against our Postgres database.

### dotenvy

[dotenvy](https://crates.io/crates/dotenvy) loads environment variables from a .env file, if available, and mashes those with the actual environment variables provided by the operative system. We will use this library to store our Postgres connection URL inside the .env file.

### log & pretty_env_logger

[log](https://crates.io/crates/log) & [pretty_env_logger](https://crates.io/crates/pretty_env_logger) will allow us to log output to the console while our server is running.

## Steps

### Step 0

First we need to download and setup PostgreSQL. There are two ways of doing this... download Postgres directly from the official website or use Docker. Using docker is recommended and the following steps will assume you are using Docker.

__Docker (recommended)__

1. [Install Docker](https://www.docker.com/)
2. Install the [Postgres image](https://hub.docker.com/_/postgres) by running `docker pull postgres` in your terminal
3. Start a Postgres instance by running:
    ```shell
    $ docker run --name stack-overflow-db -e POSTGRES_PASSWORD=postgrespw -p 55008:5432 -d postgres
    ```

__Official website (NOT recommended)__

1. Download Postgres from the [official website](https://www.postgresql.org/download/)
2. Create a Postgres database (follow [this video](https://www.youtube.com/watch?v=fZQI7nBu32M))

__GUI client (optional)__

You may want to use a visual client to see the state of your database. One of the most popular Postgres clients is pgAdmin 4. Follow these steps to install and use it:

1. [Install pgAdmin 4](https://www.pgadmin.org/download/)
2. Register a new server with the following properties:
   - name: stack-overflow-db
   - hostname/address: localhost
   - port: 55008
   - username: postgres
   - password: postgrespw
3. You should now be able to connect to your PostgreSQL server instance

### Step 1

__Persistence (setting up Postgres)__

Now that we got Postgres installed. Let's setup our database!

A `.env` file has been created for you. It contains `DATABASE_URL` which should point to your local Postgres instance. Double-check that the url string is correct (specifically the port number).

A `.cargo/config.toml` file has been created for you. This is a configuration file for Cargo which sets the `RUST_LOG` environment variable to `info`. `RUST_LOG` controls the verbosity levels of the logger. For more info see the [Rust Cookbook page on logging](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html).

A `migrations` folder has also been created for you. This folder stores [migration files](https://www.prisma.io/dataguide/types/relational/what-are-database-migrations) with SQL queries responsible for setting up database tables and cleaning up database tables. Look through these files to understand the SQL structure of the `questions` and `answers` tables.

Let's run these migrations against your local Postgres server instance.

First, install [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli). This is SQLx's associated command-line utility for managing databases, migrations, and more:
```shell
$ cargo install sqlx-cli
```

Now you can execute migrations by running:
```shell
$ sqlx migrate run
```

__NOTE:__ If you ever want to revert the migrations, simply run:
```shell
$ sqlx migrate revert
```

### Step 1.1

__Persistence (connecting to Postgres)__

Now that Postgres is installed and our database tables are setup, let's query our database from `main.rs`!

Do this by completing the TODO items in `main.rs`.

Then run the server using `cargo run` or cargo watch:
```shell
$ cargo watch -q -c -w src/ -x run
```

You should see the following log output in your terminal:
```shell
 INFO  stackoverflow_api > ********* Question Records *********
 INFO  stackoverflow_api > []
```

__NOTE:__ The vector is empty because we haven't created any questions yet.

### Step 2

__Persistence (DAOs)__

Now that we can execute SQL queries from our code, it's time to create the [DAOs](https://en.wikipedia.org/wiki/Data_access_object) for creation, retrieval & deletion of questions & answers.

Note that a few dependencies have been added.

`thiserror` has been added to make it easier to build custom error types.

`async_trait` has been added to support async functions in traits.

Inside `models.rs` a new error type called `DBError` has been defined. We will use this error type inside the DOAs. For simplicity we will only differentiate between invalid UUID errors and any other type of error.

Inside `models.rs` a module called `postgres_error_codes` has also been added. It contains one constant, `FOREIGN_KEY_VIOLATION`, which maps to a Postgres error code. We will use this error code detect UUID errors.  

A new folder called `persistance` has also been created. This folder contains the DAOs... `questions_dao.rs` and `answers_dao.rs`, along with a `tests` module.

Each DAO consists of 2 things... a trait (ex: `QuestionsDao`), and a concrete implementation (ex: `QuestionsDaoImpl`). Concrete implementations store a reference to the Postgres connection pool (`PgPool`).

First, delete the sqlx query and log statements by completing the TODO items in `main.rs`.

Next implement the DAOs by completing the TODO items in `questions_dao.rs` and `answers_dao.rs`.

When you finish all the TODO items running `cargo test` should result in all tests passing. Note that the DAO tests require a connection to your database so make sure your database it up and running. Also DAO tests may occasionally fail if your database is unresponsive. If this happens simple re-run the tests.