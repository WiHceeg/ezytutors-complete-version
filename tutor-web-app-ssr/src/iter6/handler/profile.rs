use crate::errors::EzyTutorError;
use actix_web::{web, Error, HttpResponse, Result};

pub async fn show_tutor_profile(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    dbg!("show_tutor_profile");
    let mut ctx = tera::Context::new();

    let s = tmpl
        .render("profile.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("TemplateError".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}