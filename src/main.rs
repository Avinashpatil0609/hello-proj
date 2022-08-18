use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::body::json;
use warp::http::StatusCode;
use warp::reply::Json;
use warp::{http, Filter, Reply};

// type Students = HashMap<String, String>;
// let Stud: Vec<String> = Vec::new();

#[derive(Debug, Deserialize, Clone)]
pub struct Student {
    id: String,
    name: String,
}

#[derive(Clone)]
struct Store {
    student_list: Arc<RwLock<Vec<Student>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            student_list: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let add_student = warp::post()
        .and(warp::path("student"))
        .and(warp::path::end())
        .and(json())
        .and(store_filter.clone())
        .and_then(add_student_to_list);
    let get_student = warp::get()
        .and(warp::path("student"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_student_from_list);

    let routes = hello.or(add_student).or(get_student);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn add_student_to_list(
    student: Student,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .student_list
        .write()
        .await
        .insert(student.id.parse().unwrap(), student);
    Ok(warp::reply::with_status(
        "Added student to the list",
        http::StatusCode::CREATED,
    ))
}

async fn get_student_from_list(
    id: String,
    store: Store,
) -> Result<impl warp::reply::Reply, warp::Rejection> {
    // let mut result = HashMap::new();

    let r = store.student_list.read().await;
    // let student = r.get(&id);
    let maybe_student = r.iter().find(|&(key,value)|key.eq(&id)).map(|kv|kv.clone());

    match maybe_student{
        Some(student) => Ok(warp::reply::json(&student).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response())
    }



    // for (key, value) in r.iter() {
    //     if key == &id {
    //         result.insert(key, value);
    //         return Ok(warp::reply::json(&result));
    //     }
    // }
    // Ok(warp::reply::with_status(
    //     Default::default(),
    //     StatusCode::NOT_FOUND,
    // ))
}
