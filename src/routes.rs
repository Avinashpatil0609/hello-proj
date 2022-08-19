use std::convert::Infallible;
use warp::Filter;
use warp::body::json;
use crate::handlers;
use crate::store::Storage;


pub(crate) fn student_routes(store: Storage) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    add_student(store.clone()).or(get_student(store.clone()))
}

fn add_student(store: Storage) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::post()
        .and(warp::path("student"))
        .and(warp::path::end())
        .and(json())
        .and(with_store(store))
        .and_then(handlers::add_student_to_list)
}

fn get_student(store: Storage) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    warp::path!("student" / String)
        .and(warp::get())
        .and(with_store(store))
        .and_then(handlers::get_student_from_list)
}


fn with_store(store: Storage) -> impl Filter<Extract = (Storage ,), Error = Infallible> + Clone{
    warp::any().map(move || store.clone())
}