use crate::routes::Route;

mod movie_id;

pub fn routes() -> Vec<Route> {
    vec![movie_id::routes()].concat()
}
