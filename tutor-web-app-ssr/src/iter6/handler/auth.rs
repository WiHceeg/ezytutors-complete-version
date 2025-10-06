use crate::dbaccess::{get_user_record, post_new_user};
use crate::errors::EzyTutorError;
use crate::iter6::jwt::{self, get_username_from_token};
use crate::iter6::state::AppState;
use crate::model::{TutorRegisterForm, TutorResponse, TutorSigninForm, User};
use actix_web::{web, Error, HttpResponse, Result};
use argon2::{self, Config};
use serde_json::json;

pub async fn home(
    tmpl: web::Data<tera::Tera>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, Error> {
    dbg!("home");

    let option_cookie = req.cookie("jwt_token");

    if option_cookie.is_none() {
        return Ok(HttpResponse::Found()
            .insert_header(("Location", "/signin"))
            .finish());
    }

    let cookie = option_cookie.unwrap();
    let user_name = get_username_from_token(cookie.value())
        .map_err(|_| EzyTutorError::JwtError("Invalid token".to_string()))?;
    let mut ctx = tera::Context::new();
    ctx.insert("name", &user_name);
    let s = tmpl
        .render("home.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    dbg!("show_register_form");

    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_username", "");
    ctx.insert("current_password", "");
    ctx.insert("current_confirmation", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");
    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    dbg!("handle_register");
    dbg!(&params);
    let username = params.username.clone();
    let user = get_user_record(&app_state.db, username.to_string()).await;
    let user_not_found: bool = user.is_err();
    // If user is not found in database, proceed to verification of passwords
    if user_not_found {
        if params.password != params.confirmation {
            ctx.insert("error", "Passwords do not match");
            ctx.insert("current_username", &params.username);
            ctx.insert("current_password", "");
            ctx.insert("current_confirmation", "");
            ctx.insert("current_name", &params.name);
            ctx.insert("current_imageurl", &params.imageurl);
            ctx.insert("current_profile", &params.profile);
            s = tmpl
                .render("register.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
        } else {
            let new_tutor = json!({
                "tutor_name": &params.name,
                "tutor_pic_url": &params.imageurl,
                "tutor_profile": &params.profile
            });
            let awc_client = awc::Client::default();
            let res = awc_client
                .post("http://localhost:3000/tutors/")
                .send_json(&new_tutor)
                .await
                .unwrap()
                .body()
                .await?;
            let tutor_response: TutorResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
            s = format!("Congratulations. You have been successfully registered with EzyTutor and your tutor id is: {}. To start using EzyTutor, please login with your credentials.",tutor_response.tutor_id);
            // Hash the password
            let salt = b"somerandomsalt";
            let config = Config::default();
            let hash =
                argon2::hash_encoded(params.password.clone().as_bytes(), salt, &config).unwrap();
            let user = User {
                username,
                tutor_id: Some(tutor_response.tutor_id),
                user_password: hash,
            };
            let _tutor_created = post_new_user(&app_state.db, user).await?;
        }
    } else {
        ctx.insert("error", "User Id already exists");
        ctx.insert("current_username", &params.username);
        ctx.insert("current_password", "");
        ctx.insert("current_confirmation", "");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn show_signin_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    dbg!("show_signin_form");
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("current_password", "");
    let s = tmpl
        .render("signin.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("TemplateError".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_signin(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorSigninForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    dbg!("handle_signin");
    dbg!(&params);
    let username = params.username.clone();
    let user = get_user_record(&app_state.db, username.to_string()).await;
    if let Ok(user) = user {
        let does_password_match = argon2::verify_encoded(
            &user.user_password.trim(),
            params.password.clone().as_bytes(),
        )
        .unwrap();
        if !does_password_match {
            ctx.insert("error", "Invalid login");
            ctx.insert("current_name", &params.username);
            ctx.insert("current_password", &params.password);
            s = tmpl
                .render("signin.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
        } else {
            let tutor_id = user.tutor_id.unwrap_or(0);
            let tutor_id_cookie =
                actix_web::cookie::Cookie::build("tutor_id", tutor_id.to_string())
                    .path("/")
                    .secure(false)
                    .http_only(false)
                    .finish();
            dbg!(tutor_id);
            let token = jwt::generate_jwt(&username, tutor_id)
                .map_err(|_| EzyTutorError::JwtError("Failed to generate token".to_string()))?;
            let jwt_cookie = actix_web::cookie::Cookie::build("jwt_token", token)
                .path("/")
                .secure(false)
                .http_only(true)
                .finish();
            dbg!(&jwt_cookie);
            return Ok(HttpResponse::SeeOther()
                .cookie(jwt_cookie)
                .cookie(tutor_id_cookie)
                .append_header(("Location", "/"))
                .finish());
        }
    } else {
        ctx.insert("error", "User id not found");
        ctx.insert("current_name", &params.username);
        ctx.insert("current_password", &params.password);
        s = tmpl
            .render("signin.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
