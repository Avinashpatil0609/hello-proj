use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::body::json;
use warp::{http, Filter};

type Students = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Student {
    id: String,
    name: String,
}

#[derive(Clone)]
struct Store {
    student_list: Arc<RwLock<Students>>,
}

impl Store {
    fn new() -> Self {
        Store {
            student_list: Arc::new(RwLock::new(HashMap::new())),
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

    let routes = hello.or(add_student);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn add_student_to_list(
    student: Student,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.student_list.write().await.insert(student.id, student.name);

    Ok(warp::reply::with_status(
        "Added student to the list",
        http::StatusCode::CREATED,
    ))
}
