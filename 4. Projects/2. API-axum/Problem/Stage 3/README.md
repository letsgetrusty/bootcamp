# Stage 3

__Connecting endpoints with DAOs__

Now that both endpoints and DAOs are created, it's time to connect these two major components so that we can return real data from our API!

## Steps

### Step 1

__Creating inner handlers__

A new file(and module) called `handlers_inner.rs` has been created in the `handlers` folder. This is where our framework agnostic handlers are defined. These handlers will do the "real work" of interacting with the DAOs. The outer handlers will eventually just call the inner handlers, passing in the appropriate inputs. The benefit of having framework agnostic handlers is that we can more easily change the web framework we are using!

Inside `handlers_inner.rs` you will notice a custom error type called `HandlerError`. For simplicity the inner handlers will only differentiate between two error types, a bad request error which means the user gave us improper inputs, or an internal error, which encapsulates every other error. A function called `default_internal_error` is also defined for convenience.

Complete this step by finishing the TODO items in `handlers_inner.rs`.

### Step 2

__Connection DAOs to Axum state__

Now that the inner handlers are complete, it's time to setup our DAO instances and hook them up to Axum. This will allow Axum to inject our DAO instances into the outer handlers.

Complete this step by finishing the TODO items in `main.rs` and `handlers/mod.rs`.

### Step 3

__Updating outer handlers to use inner handlers__

The final step is to update the outer handlers to use the inner handlers instead of returning fake data. We will also add error handling to the outer handlers.

Inside `handlers/mod.rs` the `IntoResponse` trait has been implemented for `HandlerError`. This will allow us to return `HandlerError` from the Axum route handlers.

Complete this step by finishing the TODO items in `handlers/mod.rs`.

## Building & Testing

Now you can run your server using `cargo run` or cargo watch
```shell
 $ cargo watch -q -c -w src/ -x run
```

And test the endpoints using [cURL](https://en.wikipedia.org/wiki/CURL):

Create question

```text
curl --location 'localhost:8000/question' \
--header 'Content-Type: application/json' \
--data '{
    "title": "Newly Created Question",
    "description": "My Description"
}'
```

Get questions

```text
curl --location 'localhost:8000/questions'
```

Delete question

```text
curl --location --request DELETE 'localhost:8000/question' \
--header 'Content-Type: application/json' \
--data '{ 
    "question_uuid": "[UUID of a created question]"
}'
```

Create answer

```text
curl --location 'localhost:8000/answer' \
--header 'Content-Type: application/json' \
--data '{ 
    "question_uuid": "[UUID of a created question]",
    "content": "test question"
}'
```

Get answers

```text
curl --location --request GET 'localhost:8000/answers' \
--header 'Content-Type: application/json' \
--data '{
    "question_uuid": "[UUID of a created question]"
}'
```

Delete answer

```text
curl --location --request DELETE 'localhost:8000/answer' \
--header 'Content-Type: application/json' \
--data '{ 
    "answer_uuid": "[UUID of a created answer]"
}'
```

__NOTE:__ If things are not working as expected, compare your code against the `Solution` folder.

## Final Note

Congratulations! You have built fully functioning API!

You should be proud of your progress if you've gotten this far.

Showcase your implementation and struggles you've faced along the way to others in the Let's Get Rusty community.

More importantly, teaching is the best way to learn. Any questions posted by others in the Discord channels are opportunities for you to answer and truly internalize your knowledge.