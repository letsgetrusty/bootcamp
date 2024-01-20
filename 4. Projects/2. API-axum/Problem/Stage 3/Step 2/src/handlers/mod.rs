use axum::{extract::State, response::IntoResponse, Json};

use crate::{models::*, AppState};

mod handlers_inner;

// ---- CRUD for Questions ----

pub async fn create_question(
    // Example of how to add state to a route. Note that we are using ".." to ignore the other fields in AppState.
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    Json(QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_owned(),
    })
}

pub async fn read_questions(// TODO: add questions_dao from app state as an argument
) -> impl IntoResponse {
    Json(vec![QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

pub async fn delete_question(
    // TODO: add questions_dao from app state as an argument
    Json(question_uuid): Json<QuestionId>,
) {
    // ...
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    // Example of how to add state to a route
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {
    Json(AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: "question_uuid".to_owned(),
        content: "content".to_owned(),
        created_at: "created_at".to_owned(),
    })
}

pub async fn read_answers(
    // TODO: add answers_dao from app state as an argument
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    Json(vec![AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: "question_uuid".to_owned(),
        content: "content".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

pub async fn delete_answer(
    // TODO: add answers_dao from app state as an argument
    Json(answer_uuid): Json<AnswerId>,
) {
    // ...
}
