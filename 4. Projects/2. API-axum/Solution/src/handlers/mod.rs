use axum::{
    extract::State as AxumState, http::StatusCode, response::IntoResponse, Json as JsonAxum,
};

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
    AxumState(AppState { questions_dao, .. }): AxumState<AppState>,
    JsonAxum(question): JsonAxum<Question>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::create_question(question, questions_dao.as_ref())
        .await
        .map(JsonAxum)
}

pub async fn read_questions(
    AxumState(AppState { questions_dao, .. }): AxumState<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::read_questions(questions_dao.as_ref())
        .await
        .map(JsonAxum)
}

pub async fn delete_question(
    AxumState(AppState { questions_dao, .. }): AxumState<AppState>,
    JsonAxum(question_uuid): JsonAxum<QuestionId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::delete_question(question_uuid, questions_dao.as_ref()).await
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    AxumState(AppState { answers_dao, .. }): AxumState<AppState>,
    JsonAxum(answer): JsonAxum<Answer>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::create_answer(answer, answers_dao.as_ref())
        .await
        .map(JsonAxum)
}

pub async fn read_answers(
    AxumState(AppState { answers_dao, .. }): AxumState<AppState>,
    JsonAxum(question_uuid): JsonAxum<QuestionId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::read_answers(question_uuid, answers_dao.as_ref())
        .await
        .map(JsonAxum)
}

pub async fn delete_answer(
    AxumState(AppState { answers_dao, .. }): AxumState<AppState>,
    JsonAxum(answer_uuid): JsonAxum<AnswerId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    handlers_inner::delete_answer(answer_uuid, answers_dao.as_ref()).await
}
