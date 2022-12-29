mod answers_tests {
    use sqlx::PgPool;

    use crate::{
        models::{Answer, DBError, Question},
        persistance::{
            answers_dao::{AnswersDao, AnswersDaoImpl},
            questions_dao::{QuestionsDao, QuestionsDaoImpl},
        },
    };

    #[sqlx::test]
    async fn create_answer_should_fail_with_malformed_uuid(pool: PgPool) -> Result<(), String> {
        let answer_doa = AnswersDaoImpl::new(pool);

        let result = answer_doa
            .create_answer(Answer {
                question_uuid: "malformed".to_owned(),
                content: "test content".to_owned(),
            })
            .await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::InvalidUUID(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an invalid UUID error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[sqlx::test]
    async fn create_answer_should_fail_with_non_existent_uuid(pool: PgPool) -> Result<(), String> {
        let answer_doa = AnswersDaoImpl::new(pool);

        let result = answer_doa
            .create_answer(Answer {
                question_uuid: "a22abcd2-22ab-2222-a22b-2abc2a2b22cc".to_owned(),
                content: "test content".to_owned(),
            })
            .await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::InvalidUUID(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an invalid UUID error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[sqlx::test]
    async fn create_answer_should_fail_if_database_error_occurs(
        pool: PgPool,
    ) -> Result<(), String> {
        let answer_doa = AnswersDaoImpl::new(pool.clone());

        pool.close().await;

        let result = answer_doa
            .create_answer(Answer {
                question_uuid: "a22abcd2-22ab-2222-a22b-2abc2a2b22cc".to_owned(),
                content: "test content".to_owned(),
            })
            .await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::Other(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an Other error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[sqlx::test]
    async fn create_answer_should_succeed(pool: PgPool) -> Result<(), String> {
        let question_doa = QuestionsDaoImpl::new(pool.clone());
        let answer_doa = AnswersDaoImpl::new(pool);

        let result = question_doa
            .create_question(Question {
                title: "test title".to_owned(),
                description: "test description".to_owned(),
            })
            .await
            .map_err(|e| format!("{:?}", e))?;

        let result = answer_doa
            .create_answer(Answer {
                question_uuid: result.question_uuid,
                content: "test content".to_owned(),
            })
            .await
            .map_err(|e| format!("{:?}", e))?;

        if result.content != "test content".to_owned() {
            return Err("Incorrect answer content".to_owned());
        }

        Ok(())
    }

    #[sqlx::test]
    async fn delete_answer_should_fail_with_malformed_uuid(pool: PgPool) -> Result<(), String> {
        let answer_doa = AnswersDaoImpl::new(pool);

        let result = answer_doa.delete_answer("malformed".to_owned()).await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::InvalidUUID(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an invalid UUID error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[sqlx::test]
    async fn delete_answer_should_fail_if_database_error_occurs(
        pool: PgPool,
    ) -> Result<(), String> {
        let answer_doa = AnswersDaoImpl::new(pool.clone());

        pool.close().await;

        let result = answer_doa
            .delete_answer("a22abcd2-22ab-2222-a22b-2abc2a2b22cc".to_owned())
            .await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::Other(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an Other error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[sqlx::test]
    async fn delete_answer_should_succeed(pool: PgPool) -> Result<(), String> {
        let question_doa = QuestionsDaoImpl::new(pool.clone());
        let answer_doa = AnswersDaoImpl::new(pool);

        let question = question_doa
            .create_question(Question {
                title: "test title".to_owned(),
                description: "test description".to_owned(),
            })
            .await
            .map_err(|e| format!("{:?}", e))?;

        let result = answer_doa
            .create_answer(Answer {
                question_uuid: question.question_uuid.clone(),
                content: "test content".to_owned(),
            })
            .await
            .map_err(|e| format!("{:?}", e))?;

        answer_doa
            .delete_answer(result.answer_uuid)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let results = answer_doa
            .get_answers(question.question_uuid.clone())
            .await
            .map_err(|e| format!("{:?}", e))?;

        if results.len() != 0 {
            return Err("Answer was not deleted".to_owned());
        }

        Ok(())
    }

    #[sqlx::test]
    async fn get_answers_should_fail_with_malformed_uuid(pool: PgPool) -> Result<(), String> {
        let answer_doa = AnswersDaoImpl::new(pool);

        let result = answer_doa.get_answers("malformed".to_owned()).await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::InvalidUUID(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an invalid UUID error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[sqlx::test]
    async fn get_answers_should_fail_if_database_error_occurs(pool: PgPool) -> Result<(), String> {
        let answer_doa = AnswersDaoImpl::new(pool.clone());

        pool.close().await;

        let result = answer_doa
            .get_answers("a22abcd2-22ab-2222-a22b-2abc2a2b22cc".to_owned())
            .await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::Other(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an Other error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[sqlx::test]
    async fn get_answers_should_succeed(pool: PgPool) -> Result<(), String> {
        let question_doa = QuestionsDaoImpl::new(pool.clone());
        let answer_doa = AnswersDaoImpl::new(pool);

        let question = question_doa
            .create_question(Question {
                title: "test title".to_owned(),
                description: "test description".to_owned(),
            })
            .await
            .map_err(|e| format!("{:?}", e))?;

        let result = answer_doa
            .create_answer(Answer {
                question_uuid: question.question_uuid.clone(),
                content: "test content".to_owned(),
            })
            .await
            .map_err(|e| format!("{:?}", e))?;

        let results = answer_doa
            .get_answers(question.question_uuid.clone())
            .await
            .map_err(|e| format!("{:?}", e))?;

        if results.len() != 1 {
            return Err("Incorrect number of results returned.".to_owned());
        }

        if results.get(0).unwrap().answer_uuid != result.answer_uuid {
            return Err("Incorrect answer returned.".to_owned());
        }

        Ok(())
    }
}

mod questions_tests {
    use sqlx::PgPool;

    use crate::{
        models::{DBError, Question},
        persistance::questions_dao::{QuestionsDao, QuestionsDaoImpl},
    };

    #[sqlx::test]
    async fn create_question_should_fail_if_database_error_occurs(
        pool: PgPool,
    ) -> Result<(), String> {
        let doa = QuestionsDaoImpl::new(pool.clone());

        pool.close().await;

        let result = doa
            .create_question(Question {
                title: "test title".to_owned(),
                description: "test description".to_owned(),
            })
            .await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::Other(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an Other error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[sqlx::test]
    async fn create_question_should_succeed(pool: PgPool) -> Result<(), String> {
        let doa = QuestionsDaoImpl::new(pool);

        let result = doa
            .create_question(Question {
                title: "test title".to_owned(),
                description: "test description".to_owned(),
            })
            .await
            .map_err(|e| format!("{:?}", e))?;

        if result.title != "test title".to_owned()
            || result.description != "test description".to_owned()
        {
            return Err("Incorrect title or description".to_owned());
        }

        Ok(())
    }

    #[sqlx::test]
    async fn delete_question_should_fail_with_malformed_uuid(pool: PgPool) -> Result<(), String> {
        let doa = QuestionsDaoImpl::new(pool);

        let result = doa.delete_question("malformed".to_owned()).await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::InvalidUUID(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an invalid UUID error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[sqlx::test]
    async fn delete_question_should_fail_if_database_error_occurs(
        pool: PgPool,
    ) -> Result<(), String> {
        let doa = QuestionsDaoImpl::new(pool.clone());

        pool.close().await;

        let result = doa
            .delete_question("a22abcd2-22ab-2222-a22b-2abc2a2b22cc".to_owned())
            .await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::Other(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an Other error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[sqlx::test]
    async fn delete_question_should_succeed(pool: PgPool) -> Result<(), String> {
        let doa = QuestionsDaoImpl::new(pool);

        let result = doa
            .create_question(Question {
                title: "test title".to_owned(),
                description: "test description".to_owned(),
            })
            .await
            .map_err(|e| format!("{:?}", e))?;

        doa.delete_question(result.question_uuid)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let results = doa.get_questions().await.map_err(|e| format!("{:?}", e))?;

        if results.len() != 0 {
            return Err("Question was not deleted".to_owned());
        }

        Ok(())
    }

    #[sqlx::test]
    async fn get_questions_should_fail_if_database_error_occurs(
        pool: PgPool,
    ) -> Result<(), String> {
        let doa = QuestionsDaoImpl::new(pool.clone());

        pool.close().await;

        let result = doa.get_questions().await;

        if result.is_ok() {
            return Err(format!(
                "Expected an error but got the following result: {:?}",
                result.unwrap()
            ));
        }

        if let Err(DBError::Other(_)) = result {
            Ok(())
        } else {
            Err(format!(
                "Expected an Other error but got the following error: {:?}",
                result.err()
            ))
        }
    }

    #[sqlx::test]
    async fn get_questions_should_succeed(pool: PgPool) -> Result<(), String> {
        let doa = QuestionsDaoImpl::new(pool);

        let result = doa
            .create_question(Question {
                title: "test title".to_owned(),
                description: "test description".to_owned(),
            })
            .await
            .map_err(|e| format!("{:?}", e))?;

        let results = doa.get_questions().await.map_err(|e| format!("{:?}", e))?;

        if results.len() != 1 {
            return Err("Incorrect number of results returned.".to_owned());
        }

        if results.get(0).unwrap().question_uuid != result.question_uuid {
            return Err("Incorrect question returned.".to_owned());
        }

        Ok(())
    }
}
