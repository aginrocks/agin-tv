use crate::_routes::Route;

mod login;
mod start_session;
mod user;

pub fn routes() -> Vec<Route> {
    [start_session::routes(), login::routes(), user::routes()].concat()
}
