mod routes;
mod store;
mod handlers;
// type Students = HashMap<String, String>;
// let Stud: Vec<String> = Vec::new();



// #[derive(Clone)]
// struct Store {
//     student_list: Arc<RwLock<Vec<Student>>>,
// }
//
// impl Store {
//     fn new() -> Self {
//         Store {
//             student_list: Arc::new(RwLock::new(Vec::new())),
//         }
//     }
// }

#[tokio::main]
async fn main() {
     let store = store::init_store();
    // let store_filter = warp::any().map(move || store.clone());

    // let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let student_routes = routes::student_routes(store);

    warp::serve(student_routes).run(([127, 0, 0, 1], 8080)).await;
}