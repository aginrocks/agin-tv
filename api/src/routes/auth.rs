use crate::routes::Route;

mod start_session;

pub fn routes() -> Vec<Route> {
    [start_session::routes()].concat()
}
