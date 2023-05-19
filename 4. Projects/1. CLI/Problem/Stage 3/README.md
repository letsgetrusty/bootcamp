# Stage 3

__Navigation and Program Loop__

Now that the database and UI are complete, it's time to build the bridge between them. In this step we will be building the navigator and program loop. 

The navigator will handle navigating between pages, responding to actions returned from `handle_input()` on Page objects, and displaying prompts.

The program loop is responsible for running our application and continuously asking for user input until the user exits the program.

## Steps

### Step 1

__Implementing The Navigator__

A new file called `navigator.rs` has been added. Inside this file a `Navigator` Struct is defined, which contains 3 fields. 

`pages` is a vector of `Page` objects. This vector is used for navigation. The user starts off on the home page and if they navigate to the Epic details page (for example) a new instance of `EpicDetail` will be created and pushed onto the pages vector. To navigate back to the home page we will simply pop the `EpicDetail` page off the pages vector (which acts like a stack). Note that the current page is always the last element in the `pages` vector.

`prompts` is an instance of the `Prompts` Struct. Look at the navigator tests to see how we can mock colures in the prompts Struct.

`db` is a `JiraDatabase` instance wrapped in a reference counting smart pointer so we can share ownership.

The `Navigator` Struct contains 2 primary functions. `get_current_page()` which does exactly what the name suggests and `handle_action()` which responds to user actions.

To complete this step finish all the TODO items in `navigator.rs`.

__NOTE 1:__ Use `with_context()` and the `anyhow!` macro form the anyhow crate inside `handle_action()` for error handling.

__NOTE 2:__ A method called `as_any()` has been added to all page objects. This was done to support down-casting, which is used in the navigator tests. For more info check out [this StackOverflow post](https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object).

### Step 2

__The Program Loop__

The program loop will be defined inside `main.rs`. First we will instantiate the navigator. Then inside the program loop we will get the current page, render it, wait for user input and then handle the input. 

To complete this task finish the TODO items in `main.rs`.

__NOTE 1:__ A new dependency called `clearscreen` has been added. At the top of the program loop we call `clearscreen::clear()`. This will clear the screen which is what we want to do before rendering the new content. Think about it like refreshing a web page... everything is wiped away and reloaded.

__NOTE 2:__ A function called `wait_for_key_press()` has been added to `io_utils.rs`. Use this method when displaying errors. For example:
```rust
if let Err(error) = page.draw_page() {
    println!("Error rendering page: {}\nPress any key to continue...", error);
    wait_for_key_press();
};
```

## Manual Tests

Run these manual tests (___from top to bottom in order___) to see if your program works as expected.

__NOTE:__ Before running these tests, reset the database by updating `data/db.json` to this:
```json
{
    "last_item_id": 0,
    "epics": {},
    "stories": {}
}
```

__Create Epic__

Steps:
* `cd` into the root folder of the project
* Run `cargo run`
* Input `c` to create a new Epic
* Input `"New Epic name"` as Epic name
* Input `"New Epic description"` as Epic description
* Check if `db.json` matches with the following:
     ```json
    {"last_item_id":1,"epics":{"1":{"name":"New Epic name","description":"New Epic description","status":"Open","stories":[]}},"stories":{}}
     ```

__Create Story__

Steps:
* Input `1` to select the created Epic
* Input `c` to create a new Story in the selected Epic
* Input `"New Story name"` as Story name
* Input `"New Story description"` as Story description
* Check if `db.json` matches with the following:
    ```json
    {"last_item_id":2,"epics":{"1":{"name":"New Epic name","description":"New Epic description","status":"Open","stories":[2]}},"stories":{"2":{"name":"New Story name","description":"New Story description","status":"Open"}}}
    ```

__Update Epic__

Steps:
* Input `u` to update the selected Epic
* Input `2` to select `IN-PROGRESS` as the updated status
* Check if `db.json` matches with the following:
    ```json
    {"last_item_id":2,"epics":{"1":{"name":"New Epic name","description":"New Epic description","status":"InProgress","stories":[2]}},"stories":{"2":{"name":"New Story name","description":"New Story description","status":"Open"}}}
    ```

__Update Story__

Steps:
* Input `2` to select the created Story
* Input `u` to update the selected Story
* Input `3` to select `RESOLVED` as the updated status
* Check if `db.json` matches with the following:
    ```json
    {"last_item_id":2,"epics":{"1":{"name":"New Epic name","description":"New Epic description","status":"InProgress","stories":[2]}},"stories":{"2":{"name":"New Story name","description":"New Story description","status":"Resolved"}}}
    ```

__Remove Story__

Steps:
* Input `d` to delete the selected Story
* Input `Y` to confirm removal
* Check if `db.json` matches with the following:
    ```json
    {"last_item_id":2,"epics":{"1":{"name":"New Epic name","description":"New Epic description","status":"InProgress","stories":[]}},"stories":{}}
    ```

__Remove Epic__

Steps:
* Input `c` to create a new Story in the selected Epic
* Input `"New Story name"` as Story name
* Input `"New Story description"` as Story description
* Input `d` to delete the selected Epic
* Input `Y` to confirm removal
* Check if storage.json matches with the following:
    ```json
    {"last_item_id":3,"epics":{},"stories":{}}
    ```

__NOTE:__ You can also check your work against the `Solution` folder.

## Final Note

Check this -- you're a Rust developer now!

This is a pretty elaborate first project. You should be proud of your progress if you've gotten this far.

Showcase your implementation and struggles you've faced along the way to others in the Let's Get Rusty community.
More importantly, teaching is the best way to learn. Any questions posted by others in the Discord channels are opportunities for you to answer and truly internalize your knowledge.

Congrats! And let's get started with the next modules and corresponding projects!
