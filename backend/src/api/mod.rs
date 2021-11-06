use rocket::Route;

use self::{
    admin_users::admin_routes, auth::auth_routes, invite::invite_routes, tasks::tasks_routes,
    user::user_routes,
};

mod admin_users;
mod api_result;
mod auth;
mod catchers;
mod guards;
mod invite;
mod tasks;
mod user;

pub use self::catchers::api_catchers;

pub fn api_routes() -> Vec<Route> {
    let mut api_routes = Vec::new();

    api_routes.extend(add_base("/auth", auth_routes()));
    api_routes.extend(add_base("/invites", invite_routes()));
    api_routes.extend(add_base("/user", user_routes()));
    api_routes.extend(add_base("/admin", admin_routes()));
    api_routes.extend(add_base("/tasks", tasks_routes()));

    api_routes
}

fn add_base(base: &str, routes: Vec<Route>) -> Vec<Route> {
    routes
        .into_iter()
        .map(|route| {
            route
                .map_base(|origin| format!("{}{}", base, origin))
                .expect("Route coudn't be created!")
        })
        .collect()
}
