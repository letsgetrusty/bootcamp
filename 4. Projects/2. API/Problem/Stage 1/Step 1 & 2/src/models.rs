use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Question {
    // TODO: add a public `title` field of type String
    // TODO: add a public `description` field of type String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    // TODO: add a public `question_uuid` field of type String
    // TODO: add a public `title` field of type String
    // TODO: add a public `description` field of type String
    // TODO: add a public `created_at` field of type String
}

// TODO: create a struct called `QuestionId`
//       derive the following traits: Serialize, Deserialize
//       add a public `question_uuid` field of type String

// ----------

// TODO: create a struct called `Answer`
//       derive the following traits: Serialize, Deserialize
//       add a public `question_uuid` field of type String
//       add a public `content` field of type String

// TODO: create a struct called `AnswerDetail`
//       derive the following traits: Serialize, Deserialize, Debug, Clone, PartialEq
//       add a public `answer_uuid` field of type String
//       add a public `question_uuid` field of type String
//       add a public `content` field of type String
//       add a public `created_at` field of type String

// TODO: create a struct called `AnswerId`
//       derive the following traits: Serialize, Deserialize
//       add a public `answer_uuid` field of type String
