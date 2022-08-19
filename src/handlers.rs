use warp::{http, Reply};
use warp::http::StatusCode;
use crate::store::{Storage, Student};

pub(crate) async fn add_student_to_list(
    student: Student,
    store: Storage,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.write().await.insert(student.id.parse().unwrap(), student);
    Ok(warp::reply::with_status(
        "Added student to the list",
        http::StatusCode::CREATED,
    ))
}

pub(crate) async fn get_student_from_list(
    id: String,
    store: Storage,
) -> Result<impl warp::reply::Reply, warp::Rejection> {

    let r = store.read().await;
    let maybe_student = r.iter().find(|&Student| id.eq(&Student.id)).map(|kv| kv.clone());
    match maybe_student {
        Some(student) => Ok(warp::reply::json(&student).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response())
    }
}
