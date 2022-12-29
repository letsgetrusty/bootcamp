# StackOverflow Clone

## IMPORTANT NOTE

___Please read the project description thoroughly BEFORE getting started, especially the FAQs section.___

___Re-visit the project description multiple times DURING your design and development process, to ensure you're meeting the project requirements.___

## Problem Statement
We will build the backend for StackOverflow.

We will build two primary features in StackOverflow:
1. Question creation, retrieval & deletion
2. Answer creation, retrieval & deletion

## Objective
In this project, we aim to learn and practice the following:
* Designing SQL models
* Hands-on usage of Postgres
* Designing & building APIs
* Medium-sized project structure
* Translating concepts into code
* Navigating and contributing to an existing code base

## FAQs
Here's a list of Frequently Asked Questions:

__Will there a template to build the project on top of?__

Yes. Each step has a partially built Rust project for you to finish. Stages and steps build on top of each other until you have a completed project.

__Should my implementation look exactly like the solution?__

Your code may differ from the solution, as long as your code compiles, tests are passing, and the program works as intended you are in good shape. Also after completing the project feel free to modify the program by changing the architecture, adding features, etc.

__What if I get stuck and have questions?__

If you haven't already, join our Discord server and the exclusive Bootcamp channels as instructed on the Home page of the Bootcamp. Fire away your questions and find project partners over there!

__NOTE:__ `If you don't know how to implement a TODO item, look at the corresponding test to see what is expected.`

## Terminologies

__API / Endpoint__

API and endpoint are often used interchangeably. They are the backend component that acts as the point of contact for frontend clients. Web APIs use the [HTTP protocol](https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview) to exchange messages with frontend clients. The design of the APIs or endpoints determines how the frontend and backend communicate with one another.

__Model__
Models describe how information is organized, transmitted or stored.

__CRUD & DAO__

CRUD stands for actions of creation, read, update & deletion. DAO stands for data access object and is an interface in the application to perform these actions against the database.

__StackOverflow, Question & Answer__

StackOverflow is an industry-standard forum for posting programming related questions and getting crowd-sourced answers. A user posts a question under the user's account, and other users can contribute different answers as they see most appropriate per question. There are additional features on StackOverflow, such as upvoting the question or its corresponding answers. However, question and answer creation, retrieval and deletion are the most primitive.

## Recommendations
Here's a list of recommended action items to do during and after the development, to help you more effectively build the project and learn from the project.

During Development:
* You can either create your own Rust project and copy over the code in each step or clone this repo and finish the steps directly in this repo. 
* Check the project description/requirements to make sure you are building what is asked of you.
* Utilize the included unit tests to help debug your implementation.
* If you get stuck, ask for help in the Discord server or look at the next step for the solution to the current step.
* Refactor as you implement. Keep your code clean and compartmentalized. Doing so makes debugging exponentially easier, as your implementation grows.
* Make your code compiles and all tests are passing before moving on to the next step.

After Development:
* Run through the provided manual test cases (included in the Stage 3 README), and fix any bugs! You are almost done, so finish the project strong!
* Post your completed project on GitHub. You're a Rust developer now!
* Showcase your project to your friends and family (at the very least, to others in the Let's Get Rusty community)!
* After completing the project feel free to modify the program by changing the architecture, adding features, etc. This will help you make the project your own and better internalize the lessons you've learned.

## Stages
The project is split into multiple stages. Please keep in mind, some implementation choices are made to minimize the scope of the project, so we can focus on the learning and implementing Go related concepts. Also, you will likely need to go back and refactor code from previous stages, as you add capabilities and encounter more different user scenarios.

### Stage 1

__API (endpoints & models)__

In this stage we will design our API and implement stub endpoints.


### Stage 2

__Persistence (models & connection)__

In this stage we will setup PostgreSQL, create our database schema, and connect to our database from our Rust code. 

For this stage, here are the todos:
* Create persistence models as described above.
* Establish Postgres database connection.
* Drop, if exists, and create Question, Answer & QuestionAnswers tables.

### Stage 3
__Persistence (DAOs)__

Now that we have the models and connection created, it's time to create the DAOs for creation, retrieval & deletion for Question & Answer.

Note - The SQL queries are included in the template.

### Stage 4
__Connecting endpoints with DAOs__

Now that both endpoints and DAOs are created, it's time to connect these two major components.

Remember endpoints are the entry point for receive frontend client requests, we need to query the database based on the details provided in the requests to perform creation, retrieval & deletion. These operations will take place on top of the DAOs created.

### Stage 5
__Bug Bash__

This is the stage to more thoroughly test out your implementation and fix any additionally discovered bugs.

Make sure your implementation follows these requirements:
* When you create a question/answer, a random UUID is assigned and persisted with that question/answer.
* When you create a question/answer, a timestamp for when the question/answer is created is assigned and persisted with that question/answer.
* When you create/delete a question/answer, it is indeed create and a retrieval reflects of said question/answer reflects that database record change.
* When you view questions via `GET /questions`, all questions created are listed.
* When you view answers of a question via `GET /answers`, all answers created under that question are listed.
* When you delete a question, all answers tied to that question are also deleted.
* User can terminate the application at any point, and previous creations & deletions persist.
* Test your application against the provided Test Cases, and the resulting database records match 100% with expectations.
* Fix any bugs!

#### Sample Requests
Create question

```text

curl --request POST \
  --url http://localhost/question \
  --header 'Accept: application/json' \
  --data '{
	"title": "Title",
	"description": "Description"
}'
```

Get questions

```text
curl --request GET \
  --url http://localhost/questions \
  --header 'Accept: application/json'
```

Delete question

```text
curl --request DELETE \
  --url http://localhost/question \
  --header 'Accept: application/json' \
  --data '{
	"question_uuid": "[UUID of a created question]"
}'
```

Create answer

```text
curl --request POST \
  --url http://localhost/answer \
  --header 'Accept: application/json' \
  --data '{
	"question_uuid": [UUID of a created question],
	"content": "Content"
}'
```

Get answers

```text
curl --request GET \
  --url http://localhost/answers \
  --header 'Accept: application/json' \
  --data '{
	"question_uuid": [UUID of a created question]
}'
```

Delete answer

```text
curl --request DELETE \
  --url http://localhost/answer \
  --header 'Accept: application/json' \
  --data '{
	"answer_uuid": [UUID of a created answer]
}'
```

#### Test Cases

__Get empty questions__
* `cd` into the root folder of project
    ```text
    /[root folder]    <--here
      |__ /handlers
      |__ /main
      |__ /managers
      |__ /models
      |__ /persistence
      |__ ...
    ```
* Run `cd main`
* Run `go run main.go`
* Get questions, which should be empty.

__Create question__
* `cd` into the root folder of project
    ```text
    /[root folder]    <--here
      |__ /handlers
      |__ /main
      |__ /managers
      |__ /models
      |__ /persistence
      |__ ...
    ```
* Run `cd main`
* Run `go run main.go`
* Get questions, which should be empty.
* Create one question, which should return question created.
* Get questions, which should return one question created.
* Create another question, which should return question created.
* Get questions, which should return two questions created.

__Delete question__
* `cd` into the root folder of project
    ```text
    /[root folder]    <--here
      |__ /handlers
      |__ /main
      |__ /managers
      |__ /models
      |__ /persistence
      |__ ...
    ```
* Run `cd main`
* Run `go run main.go`
* Get questions, which should be empty.
* Create one question, which should return question created.
* Get questions, which should return one question created.
* Delete non-existing question, which shouldn't do anything.
* Create one question, which should return question created.
* Delete existing question.
* Get questions, which should be empty.

__Get empty answers__
* `cd` into the root folder of project
    ```text
    /[root folder]    <--here
      |__ /handlers
      |__ /main
      |__ /managers
      |__ /models
      |__ /persistence
      |__ ...
    ```
* Run `cd main`
* Run `go run main.go`
* Get answers with non-existing question, which should be empty
* Create one question, which should return question created.
* Get answers with existing question, which should be empty.

__Create answer__
* `cd` into the root folder of project
    ```text
    /[root folder]    <--here
      |__ /handlers
      |__ /main
      |__ /managers
      |__ /models
      |__ /persistence
      |__ ...
    ```
* Run `cd main`
* Run `go run main.go`
* Create one question, which should return question created.
* Get answers with non-existing question, which should be empty.
* Get answers with existing question with no answers, which should be empty.
* Create answer with non-existing question, which shouldn't do anything.
* Get answers with non-existing question, which should be empty.
* Create answer with existing question, which should return answer created.
* Get answers with existing question, which should return answer created.

__Delete answer__
* `cd` into the root folder of project
    ```text
    /[root folder]    <--here
      |__ /handlers
      |__ /main
      |__ /managers
      |__ /models
      |__ /persistence
      |__ ...
    ```
* Run `cd main`
* Run `go run main.go`
* Create one question, which should return question created.
* Get answers with existing question, which should be empty.
* Create one answer with existing question, which should return answer created.
* Get answers with existing question, which should return one answer created.
* Create another answer with existing question, which should return answer created.
* Get answers with existing question, which should return two answers created.
* Delete non-existing answer, which shouldn't do anything.
* Get answers with existing question, which should return two answers created.
* Delete one existing answer under existing question.
* Get answers with existing question, which should return one answers created.
* Delete another existing answer under existing question.
* Get answers with existing question, which should be empty.

## Get Started!

Get started by navigating to Stage 1 and reading the README!