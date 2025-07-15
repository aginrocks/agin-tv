use crate::routes::Route;

mod login;
mod start_session;

pub fn routes() -> Vec<Route> {
    [start_session::routes(), login::routes()].concat()
}
