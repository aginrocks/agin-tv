use crate::routes::Route;

pub mod movie_id;

pub fn routes() -> Vec<Route> {
    [movie_id::routes()].concat()
}
