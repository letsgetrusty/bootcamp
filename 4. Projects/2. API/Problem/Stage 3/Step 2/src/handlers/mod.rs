use rocket::{serde::json::Json, State};

use crate::models::*;

mod handlers_inner;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    // Example of how to add state to a route
    // TODO: fix compile time error importing QuestionsDao
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Json<QuestionDetail> {
    Json (
        QuestionDetail {
            question_uuid: "question_uuid".to_owned(),
            title: "title".to_owned(),
            description: "description".to_owned(),
            created_at: "created_at".to_owned()
        }
    )
}

#[get("/questions")]
pub async fn read_questions(
    questions_dao: todo!(), // add the appropriate type annotation
) -> Json<Vec<QuestionDetail>> {
    Json (
        vec![QuestionDetail {
            question_uuid: "question_uuid".to_owned(),
            title: "title".to_owned(),
            description: "description".to_owned(),
            created_at: "created_at".to_owned()
        }]
    )
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: todo!(), // add the appropriate type annotation
) {
    // ...
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    // Example of how to add state to a route
    // TODO: fix compile time error importing AnswersDao
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Json<AnswerDetail> {
    Json (
        AnswerDetail {
            answer_uuid: "answer_uuid".to_owned(),
            question_uuid: "question_uuid".to_owned(),
            content: "content".to_owned(),
            created_at: "created_at".to_owned()
        }
    )
}

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(
    question_uuid: Json<QuestionId>,
    answers_dao: todo!(), // add the appropriate type annotation
) -> Json<Vec<AnswerDetail>> {
    Json (
        vec![AnswerDetail {
            answer_uuid: "answer_uuid".to_owned(),
            question_uuid: "question_uuid".to_owned(),
            content: "content".to_owned(),
            created_at: "created_at".to_owned()
        }]
    )
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>,
    answers_dao: todo!(), // add the appropriate type annotation
) {
    // ...
}
