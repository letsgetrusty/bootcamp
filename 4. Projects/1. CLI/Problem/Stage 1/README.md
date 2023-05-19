# Stage 1

__Database and Models__

In backend development projects, the database design is often the very first task to complete. The database design determines what and how information is imported and stored for repeated usages. While designing the database (what technologies to use, how to model it, etc.), we can very quickly assess if the project is feasible and if we can meet the requirements.

In this project, we will persist Epic and Story records in a JSON file to keep things as simple as possible. The JSON model contains the following components:
* `last_item_id` - A global integer ID counter for both Epics and Stories. Each newly created Epic/Story will increment the counter.
* `epics` - A mapping between Epic IDs and the actual Epics. An Epic will consist a list of Stories in the form of Story IDs.
* `stories` - A mapping between Story IDs and the actual Stories.
* `epic` and `story` both have `name`, `description` and `status`.
* `status` can be `Open`, `InProgress`, `Resolved` or `Closed`. 

Here's an example for how the JSON file will look like:
```json
{
  "last_item_id": 3,
  "epics": {
    "1": {
      "name": "Epic - Project 1",
      "description": "This is Project 1 for the Bootcamp",
      "status": "InProgress",
      "stories": [
        2,
        3
      ]
    }
  },
  "stories": {
    "2": {
      "name": "Story - Project 1 Solution",
      "description": "Please provide full implement for Project 1",
      "status": "Closed"
    },
    "3": {
      "name": "Story - Project 1 README",
      "description": "Please create README file for Project 1",
      "status": "InProgress"
    }
  }
}
```

This file will be stored in `data/db.json`.

## Steps

### Step 1

__Modeling the JSON representation in Rust__

Take the JSON representation and translate it to Rust Structs and Enums. Do this by completing the TODO items in `models.rs`.

### Step 2

__Reading and writing to the JSON file__

A new file called `db.rs` has been added. This is where we will store the logic which handles reading and writing the JSON file. This file contains two items, `Database` and `JSONFileDatabase`. `Database` is a trait with two methods, `read_db` and `write_db`. For simplicity we will read/write the entire state of the database. `JSONFileDatabase` is a Struct that implements the `Database` trait. 

Note that a few dependencies have also been added.

`anyhow` has been added for error handling.

`serde` and `serde_json` have been added for serializing/de-serializing JSON. We haven't discussed these crates in the bootcamp but they are very straight forward. Please check out the documentation for both on [crates.io](https://crates.io/) if you are not familiar with them. You will see serde used in MANY Rust projects.

Lastly `tempfile` has been added as a dev dependency to help with testing.

Complete this step by finishing the TODO items in `db.rs` and `models.rs`.

### Step 3

__Add CRUD operations for Epics/Stories__

Another Struct called `JiraDatabase` has been added to `db.rs`. This Struct will contain CRUD methods for Epics and Stories. 

Complete this step by finishing the TODO items in `db.rs`. 

__NOTE 1:__ Use the `anyhow!()` macro for error handling.

__NOTE 2:__ Take a look at the `test_utils` module. Because `JiraDatabase` stores a trait object which can be any type that implements `Database`, we can create a mock database for testing (`MockDB`). Also note that `MockDB` uses the `RefCell` smart pointer. This is because `write_db()` takes an immutable reference to `self` and we need some way to work around this restriction.
