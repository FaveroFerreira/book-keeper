use std::sync::Arc;

use sqlx::query_as;
use uuid::Uuid;

use crate::config::Ctx;
use crate::student::model::{Student, StudentDTO};

pub async fn select_student_by_id(ctx: &Arc<Ctx>, student_id: Uuid) -> Option<Student> {
    const SELECT_STUDENT_BY_ID_QUERY: &str =
        "SELECT * FROM student WHERE id = $1 AND deleted = false";

    query_as::<_, Student>(SELECT_STUDENT_BY_ID_QUERY)
        .bind(student_id)
        .fetch_optional(&ctx.db_pool)
        .await
        .unwrap()
}

pub async fn select_all_students(ctx: &Arc<Ctx>) -> Vec<Student> {
    const SELECT_STUDENTS_QUERY: &str = "SELECT * FROM student WHERE deleted = false";

    query_as::<_, Student>(SELECT_STUDENTS_QUERY)
        .fetch_all(&ctx.db_pool)
        .await
        .unwrap()
}

pub async fn insert_new_student(ctx: &Arc<Ctx>, student: &StudentDTO) -> Student {
    const INSERT_NEW_STUDENT_QUERY: &str = "
    INSERT INTO student(id, name, class, ra, deleted)
    VALUES($1, $2, $3, $4, $5)
    RETURNING id, name, class, ra, deleted";

    query_as::<_, Student>(INSERT_NEW_STUDENT_QUERY)
        .bind(Uuid::new_v4())
        .bind(&student.name)
        .bind(&student.class)
        .bind(&student.ra)
        .bind(false)
        .fetch_one(&ctx.db_pool)
        .await
        .unwrap()
}
