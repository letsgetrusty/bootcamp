use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{models::*, AppState};

mod handlers_inner;

impl IntoResponse for handlers_inner::HandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            handlers_inner::HandlerError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            handlers_inner::HandlerError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

// ---- CRUD for Questions ----

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Replace the fake data below with a call to `handlers_inner::create_question`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    Json(QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_owned(),
    })
}

pub async fn read_questions(
    State(AppState { questions_dao, .. }): State<AppState>,
) -> impl IntoResponse {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Replace the fake data below with a call to `handlers_inner::read_questions`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    Json(vec![QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Make a call to `handlers_inner::delete_question`.
    // Return a unit type in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Replace the fake data below with a call to `handlers_inner::create_answer`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    Json(AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: "question_uuid".to_owned(),
        content: "content".to_owned(),
        created_at: "created_at".to_owned(),
    })
}

pub async fn read_answers(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Replace the fake data below with a call to `handlers_inner::read_answers`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    Json(vec![AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: "question_uuid".to_owned(),
        content: "content".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

pub async fn delete_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Make a call to `handlers_inner::delete_answer`.
    // Return a unit type in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
}
