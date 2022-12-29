# Stage 1

__API (endpoints & models)__

In any sizable project with a frontend interface, API endpoints are among the first things to be designed. This API design is then used as a contract between frontend engineers and backend engineers during the development cycle.

For this project here are the API endpoints we will implement:

### Questions

**Question creation** 
```
POST /question
```

Sample request

```json
{
    "title": "Newly Created Question",
    "description": "My Description"
}
```

Sample response

```json
{
    "question_uuid": "d347261c-3f0e-42d2-8706-5ef9f1b96725",
    "title": "Newly Created Question",
    "description": "My Description",
    "created_at": "2022-12-31 18:44:08.287442"
}
``` 

**Question retrieval**
```
GET /questions
```

Sample request

** No body for this request **

Sample response

```json
[
    {
        "question_uuid": "d347261c-3f0e-42d2-8706-5ef9f1b96725",
        "title": "Newly Created Question",
        "description": "My Description",
        "created_at": "2022-12-31 18:44:08.287442"
    }
]
``` 

Question deletion
```
DELETE /question
```

Sample request

```json
{ 
    "question_uuid": "8c8e50f0-0302-4e5f-beb5-11bfece813ab"
}
```

Sample response

** No body for this response. A 200 status code should be returned **

### Answers

**Answer creation** 
```
POST /answer
```

Sample request

```json
{ 
    "question_uuid": "b068cd2f-edac-479e-98f1-c5f91008dcbd",
    "content": "test question"
}
```

Sample response

```json
{
    "answer_uuid": "a1a14a9c-ab9e-481b-8120-67f675531ed2",
    "question_uuid": "b068cd2f-edac-479e-98f1-c5f91008dcbd",
    "content": "test question",
    "created_at": "2022-12-31 13:11:59.728682"
}
``` 

**Answer retrieval**
```
GET /answers
```

Sample request

```json
{
    "question_uuid": "b068cd2f-edac-479e-98f1-c5f91008dcbd"
}
```

Sample response

```json
[
    {
        "answer_uuid": "a1a14a9c-ab9e-481b-8120-67f675531ed2",
        "question_uuid": "b068cd2f-edac-479e-98f1-c5f91008dcbd",
        "content": "test question",
        "created_at": "2022-12-31 13:11:59.728682"
    }
]
``` 

Answer deletion
```
DELETE /answer
```

Sample request

```json
{ 
    "answer_uuid": "df818bc0-dcfc-4129-bc38-a4d9c808d09c"
}
```

Sample response

** No body for this response. A 200 status code should be returned **

---

For this stage, we are going to implement the API endpoints described above and return dummy responses.

## Third Party Libraries

Rust has a minimal runtime, which means we will need to use several third-party libraries to implement our project.

### Tokio

[Tokio](https://tokio.rs/) is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing network applications. We discussed Tokio in the advanced section of the Bootcamp. Re-visit that section if you need a refresher.

### Rocket

One of the most popular server frameworks for Rust is [Rocket](https://rocket.rs/). Rocket takes advantage of Rust's macro system to make writing APIs simple, fast, and type safe. Rocket also supports async via Tokio.

Here is a simple Rocket project with one route that prints name and age:

```rust
// #[macro_use] globally imports macros from the rocket crate
// extern crate is needed to import Rocket because it uses proc macros & absolute paths. See: https://users.rust-lang.org/t/why-does-the-rocket-crate-still-require-use-of-extern-crate-re-exporting-crates/72014
#[macro_use] extern crate rocket;

#[get("/hello/<name>/<age>")] // Creates an HTTP GET route
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch] // Creates an async main function and sets up the Rocket framework
fn rocket() -> _ {
    // Builds a Rocket server instance and mounts our routes
    rocket::build().mount("/", routes![hello])
}
```

I'll explain certain concepts in Rocket throughout this project but I highly recommend reading through the [Rocket guide](https://rocket.rs/v0.5-rc/guide/) to get familiar with the framework.

### Serde

[Serde](https://serde.rs/) is a framework for serializing and deserializing Rust data structures efficiently and generically. In this project we will use Serde to serialize and deserialize [JSON](https://en.wikipedia.org/wiki/JSON).

## Project Structure

```text
/[root folder]
    |__ src
        |__ /handlers
            |__ mod.rs
        |__ cors.rs
        |__ main.rs
        |__ models.rs
    |__ Cargo.toml
```

**Cargo.toml**

`serde`, `tokio`, and `rocket` have been included as dependencies.

**main.rs**

This is were we will setup and run our web server. Eventually we will also setup our database connection, data access objects, and logging infrastructure in this file.

**cors.rs**

This file defines some Rocket middleware that handles [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS). It is needed so that we can call our server. You don't need to worry about the specifics of this file, however if you are curious about middleware in Rocket you can read the [documentation on Fairings](https://rocket.rs/v0.5-rc/guide/fairings/#fairings).

**models.rs**

This is where we will define our models.

**handlers/mod.rs**

This is where we will define our API routes.

## Steps

### Step 1

__Implementing the models__

Implement models in `models.rs` by completing the TODO items. 

Note that each model derives the `Serialize` and `Deserialize` traits from `serde`. These traits will allow us to serialize and deserialize the models to and from JSON.

Some models derive `Debug`, `Clone`, and `PartialEq`. `Debug` allows the model to be printed out using debug formatting which is useful for logging. `Clone` allows us to clone the model and `PartialEq` allows us to compare two instances of a model. We will take advantage of these capabilities later on in the project.

### Step 2

__Implementing the route handlers__

Handlers (or route handlers) are functions that handle specific requests. Implement the routes in `handlers/mod.rs` by completing the TODO items.

## Running

After completing steps 1 & 2 you should have a fully functioning server!

Execute `cargo run` to run your server.

Tip: Use [cargo watch](https://github.com/watchexec/cargo-watch) instead of `cargo run` to automatically restart your server when source files change. Example: `cargo watch -q -c -w src/ -x run`.

## Testing

Now that your server it running, test it out by calling the endpoints!

You can call the endpoints using [cURL](https://en.wikipedia.org/wiki/CURL):

Create question

```text
curl --request POST \
  --url http://localhost:8000/question \
  --header 'Accept: application/json' \
  --data '{
	"title": "Title",
	"description": "Description"
}'
```

Get questions

```text
curl --request GET \
  --url http://localhost:8000/questions \
  --header 'Accept: application/json'
```

Delete question

```text
curl --request DELETE \
  --url http://localhost:8000/question \
  --header 'Accept: application/json' \
  --data '{
	"question_uuid": "[UUID of a created question]"
}'
```

Create answer

```text
curl --request POST \
  --url http://localhost:8000/answer \
  --header 'Accept: application/json' \
  --data '{
	"question_uuid": "[UUID of a created question]",
	"content": "Content"
}'
```

Get answers

```text
curl --request GET \
  --url http://localhost:8000/answers \
  --header 'Accept: application/json' \
  --data '{
	"question_uuid": "[UUID of a created question]"
}'
```

Delete answer

```text
curl --request DELETE \
  --url http://localhost:8000/answer \
  --header 'Accept: application/json' \
  --data '{
	"answer_uuid": "[UUID of a created answer]"
}'
```
