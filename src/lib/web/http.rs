use crate::data::AppDatabase;
use crate::service;
use crate::service::action;
use crate::web::{ctx, form, renderer::Renderer, PageError};
use crate::{ServiceError, ShortCode};
use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::content::{self, Html};
use rocket::response::{status, Redirect};
use rocket::{uri, State};
use sqlx::database;

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> Html<String> {
    let ctx = ctx::Home::default();
    Html(renderer.render(ctx, &[]))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home, get_clip]
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

#[rocket::get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
) -> Result<status::Custom<Html<String>>, PageError> {
    fn render_with_status<T: ctx::PageContext + serde::Serialize + std::fmt::Debug>(
        status: Status,
        context: T,
        renderer: &Renderer,
    ) -> Result<status::Custom<Html<String>>, PageError> {
        Ok(status::Custom(status, Html(renderer.render(context, &[]))))
    }

    match action::get_clip(shortcode.clone().into(), database.get_pool()).await {
        Ok(clip) => {
            let context = ctx::ViewClip::new(clip);
            render_with_status(Status::Ok, context, renderer)
        }
        Err(e) => match e {
            ServiceError::PermissionError(_) => {
                let context = ctx::Passwordrequired::new(shortcode);
                render_with_status(Status::Unauthorized, context, renderer)
            }
            ServiceError::NotFound => Err(PageError::NotFound("Clip not found".to_owned())),
            _ => Err(PageError::Internal("server error".to_owned())),
        },
    }
}
