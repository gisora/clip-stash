use crate::data::AppDatabase;
use crate::service;
use crate::service::action;
use crate::web::{ctx, form, renderer::Renderer, PageError};
use crate::{ServiceError, ShortCode};
use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::content::Html;
use rocket::response::{status, Redirect};
use rocket::{uri, State};

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> Html<String> {
    let ctx = ctx::Home::default();
    Html(renderer.render(ctx, &[]))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home]
}

pub mod catcher {
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        eprintln!("General error: {:?}", req);
        "something went wrong..."
    }

    #[catch(500)]
    fn internal_error(req: &Request) -> &'static str {
        eprintln!("Internal error: {:?}", req);
        "internal server error"
    }

    #[catch(404)]
    fn not_found(req: &Request) -> &'static str {
        "404"
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![default, internal_error, not_found]
    }
}