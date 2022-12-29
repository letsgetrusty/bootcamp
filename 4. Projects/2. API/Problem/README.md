# StackOverflow Clone

## IMPORTANT NOTE

___Please read the project description thoroughly BEFORE getting started, especially the FAQs section.___

___Re-visit the project description multiple times DURING your design and development process, to ensure you're meeting the project requirements.___

## Problem Statement
We will build an API for a StackOverflow-like app.

We will build two primary features in StackOverflow:
1. Question creation, retrieval & deletion
2. Answer creation, retrieval & deletion

![api-gif](./api.gif)

## Objective
In this project, we aim to learn and practice the following:
* Designing & building APIs
* Using a backend framework (Rocket)
* Designing SQL models
* Hands-on usage of Postgres
* Writing testable code
* Organizing code using modules
* Navigating and contributing to an existing code base

## Terminologies

__StackOverflow, Question & Answer__

StackOverflow is an industry-standard forum for posting programming related questions and getting crowd-sourced answers. A user posts a question under the user's account, and other users can contribute different answers as they see most appropriate per question. There are additional features on StackOverflow, such as upvoting the question or its corresponding answers. However, question and answer creation, retrieval and deletion are the most primitive.

__API / Endpoint__

API and endpoint are often used interchangeably. They are the backend component that acts as the point of contact for frontend clients. Web APIs use the [HTTP protocol](https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview) to exchange messages with frontend clients. The design of the APIs or endpoints determines how the frontend and backend communicate with one another.

__Model__
Models describe how information is organized, transmitted or stored.

__Database / Storage / Persistence__

Database, storage and persistence are often used interchangeably. They represent the component we use to store and access information for the application.

__CRUD & DAO__

CRUD stands for actions of creation, read, update & deletion. DAO stands for data access object and is an interface in the application to perform these actions against the database.

## Recommendations
Here's a list of recommended action items to do during and after the development, to help you more effectively build the project and learn from the project.

During Development:
* You can either create your own Rust project and copy over the code in each step or clone this repo and finish the steps directly in this repo. 
* Check the project description/requirements to make sure you are building what is asked of you.
* Utilize the included unit tests to help debug your implementation.
* If you get stuck, ask for help in the Discord server or look at the next step for the solution to the current step.
* Refactor as you implement. Keep your code clean and compartmentalized. Doing so makes debugging exponentially easier, as your implementation grows.
* Make sure your code compiles and all tests are passing (if applicable) before moving on to the next step.

After Development:
* Run through the provided manual test cases (included in the Stage 3 README), and fix any bugs! You are almost done, so finish the project strong!
* Post your completed project on GitHub. You're a Rust developer now!
* Showcase your project to your friends and family (at the very least, to others in the Let's Get Rusty community)!
* After completing the project feel free to modify the program by changing the architecture, adding features, etc. This will help you make the project your own and better internalize the lessons you've learned.

## FAQs

__Will there a template to build the project on top of?__

Yes. Each step has a partially built Rust project for you to finish. Stages and steps build on top of each other until you have a completed project.

__Should my implementation look exactly like the solution?__

Your code may differ from the solution, as long as your code compiles, tests are passing, and the program works as intended you are in good shape. Also after completing the project feel free to modify the program by changing the architecture, adding features, etc.

__What if I get stuck and have questions?__

If you haven't already, join our Discord server and the exclusive Bootcamp channels as instructed on the Home page of the Bootcamp. Fire away your questions and find project partners over there!

__NOTE:__ `If you don't know how to implement a TODO item, look at the corresponding test to see what is expected.`

## Stages Overview
The project is split into multiple stages. Please keep in mind, some implementation choices are made to minimize the scope of the project, so we can focus on the learning and implementing Rust related concepts. Here's an overview of the stages:

### Stage 1

__API (endpoints & models)__

In this stage we will design our API and implement stub endpoints.

### Stage 2

__Persistence (connection & DAOs)__

In this stage we will setup PostgreSQL, create our database schema, and connect to our database from Rust code. We will also create DAOs for questions and answers.

### Stage 3

__Connecting endpoints with DAOs__

In this stage we will hook up our DAOs with our endpoint handlers. After this step you will have a fully functioning API!

## Get Started!

Get started by navigating to Stage 1 and reading the README!