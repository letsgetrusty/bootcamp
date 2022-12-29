# Stage 2

__Persistence (models & connection)__

In backend development projects, after the API contract is established, the database design is often the very next task to complete. The database design determines what and how information is imported and stored for repeated usage. While designing the database (what technologies to use, how to model it, etc.), we can very quickly assess if the project is a feasible idea and if we can meet most requirements.

In this project, we will persist Question and Answer records into a SQL database. The primary components are as follows:

Question

| Name          | Type         | Description                                  |
|---------------|--------------|----------------------------------------------|
| question_uuid | UUID         | Generated identifier unique to each question |
| title         | VARCHAR(255) | Title of the question                        |
| description   | VARCHAR(255) | Description of the question                  |
| created_at    | TIMESTAMP    | Creation timestamp of the question           |

Answer

| Name          | Type         | Description                                  |
|---------------|--------------|----------------------------------------------|
| answer_uuid   | UUID         | Generated identifier unique to each answer   |
| question_uuid | UUID         | Generated identifier unique to each question |
| content       | VARCHAR(255) | Content of the answer                        |
| created_at    | TIMESTAMP    | Creation timestamp of the answer             |

Note - The SQL queries for creating and dropping tables are included in the template.

For this stage, here are the todos:
* Create persistence models as described above.
* Establish Postgres database connection.
* Drop, if exists, and create Question, Answer & QuestionAnswers tables.


## Third Party Libraries

TODO


## Project Structure / New Files

TODO?

## Steps

### Step 1

First we need to download and setup PostgreSQL. There are two ways of doing this... download Postgres directly from the official website or use Docker. Using docker is recommended and the steps below will assume you are using Docker.

__Docker (recommended)__

1. [Install Docker](https://www.docker.com/)
2. Install the [Postgres image](https://hub.docker.com/_/postgres) by running `docker pull postgres` in a terminal
3. Start a Postgres instance by running:
    ```text
    docker run --name stack-overflow-db -e POSTGRES_PASSWORD=postgrespw -p 55001:5432 -d postgres
    ```

__Official website (NOT recommended)__

1. Download Postgres from the [official website](https://www.postgresql.org/download/)
2. Create a Postgres database (follow [this video](https://www.youtube.com/watch?v=fZQI7nBu32M))

__GUI client (optional)__

You may want to use a visual client to see the state of your database. One of the most popular Postgres clients is pgAdmin 4. 

1. [Install pgAdmin 4](https://www.pgadmin.org/download/)
2. Register a new server with the following properties:
   - name: stack-overflow-db
   - hostname/address: localhost
   - port: 55008
   - username: postgres
   - password: postgrespw
3. You should now be able to connect to your PostgreSQL server instance

### Step 2

Configure slqx



TODO

### Step 3

## Running

TODO

## Testing

TODO