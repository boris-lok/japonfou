use warp::reject::Rejection;

pub type WebResult<T> = Result<T, Rejection>;
