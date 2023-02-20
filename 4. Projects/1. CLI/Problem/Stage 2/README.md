# Stage 2

__UI (pages and prompts)__

Now that we have the database working we will implement the user interface of our application. The UI is made up of two objects, pages and prompts.

Pages represent the navigable pages of our application. There are 3 pages.

__Pages__

Home - The home page displays a list of epics.

```
----------------------------- EPICS -----------------------------
     id     |               name               |      status
1           | Epic - Project 1                 | IN PROGRESS
4           | Epic - Project 2                 | OPEN


[q] quit | [c] create epic | [:id:] navigate to epic
```

Epic Detail - The epic detail page displays a single epic's details and a list of stories connected to that epic.

```
------------------------------ EPIC ------------------------------
  id  |     name     |         description         |    status
1     | Epic - Pr... | This is Project 1 for th... | IN PROGRESS

---------------------------- STORIES ----------------------------
     id     |               name               |      status
2           | Story - Project 1 Solution       | CLOSED
3           | Story - Project 1 README         | RESOLVED


[p] previous | [u] update epic | [d] delete epic | [c] create story | [:id:] navigate to story
```

Story Detail - The story detail page display a single story's details.

```
------------------------------ STORY ------------------------------
  id  |     name     |         description         |    status
2     | Story - P... | Please provide full impl... | CLOSED


[p] previous | [u] update story | [d] delete story
```

Pages will have access to the database so they can query it and render the data in a nicely formatted way.

Pages have two methods, `draw_page()` and `handle_input()`. 

`draw_page()` is responsible for rendering the page to standard out. This method will also render a list of actions users can take (ex: navigate to epic detail page, create story, delete epic, etc.)

`handle_input()` is responsible for handling user input and potentially producing an action. The return type is `Result<Option<Action>>` because this function call can fail. If it doesn't fail then it can optionally return an action. Returning `None` means that the user input was invalid.

__Actions__ 

User actions are represented by the `Action` Enum in `models.rs`. 

__Prompts__

Prompts are used when more complicated user input is needed. For example, when creating a new epic the user is asked to enter a name and description. 

## Steps

### Step 1

__Pages and Page Helpers__

Pages are defined inside the UI module. Complete this step by finishing the TODOs in `pages/mod.rs`, `page_helpers.rs`, and `models.rs`. Make sure all the tests are passing.

__NOTE 1:__ Status needs to implement the `Display` trait so it can be printed to the screen. The implementation should result in the following Enum variant to string mapping:

* OPEN -> "OPEN"
* InProgress -> "IN PROGRESS"
* Resolved -> "RESOLVED"
* Closed -> "Closed"

__NOTE 2:__ `page_helpers.rs` contains a function called `get_column_string()` which is used to confine text to a specific character width. A dependency called `ellipse` was added to help accomplish this. Use the `truncate_ellipse()` function from the `ellipse` crate to implement `get_column_string()`.

__NOTE 3:__ Use `get_column_string()` inside the `draw_page()` methods.

__NOTE 4:__ The `iterools` dependency has also been added. This allows you to sort an iterator by calling `sorted()` on it.

### Step 2

__Prompts__

Complete this step by finishing the TODOs in `prompts.rs`. The `Prompts` Struct has been added as a level of indirection which will allow us to mock the prompt functions during testing later on in this project.

Here is how each prompt should look:

Create Epic
```
----------------------------
Epic Name:
test
Epic Description:
test
```

Create Story
```
----------------------------
Story Name:
test
Story Description:
test
```

Delete Epic
```
----------------------------
Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]:
Y
```

Delete Story
```
----------------------------
Are you sure you want to delete this story? [Y/n]: Y
```

Update Status
```
----------------------------
New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED):
3
```

__NOTE:__ a new file called `io_utils.rs` has been added. This file contains a function called `get_user_input()`. Use this function inside the prompt functions for getting user input.
