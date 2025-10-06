use crate::model::{NewCourse, GetCourseResponse, NewCourseResponse, UpdateCourse, UpdateCourseResponse};
use crate::state::AppState;
use crate::iter6::jwt::{self, get_tutor_id_from_token};
use crate::errors::EzyTutorError;

use actix_web::{web, Error, HttpResponse, Result};
use awc::Client;
use serde_json::json;

pub async fn show_courses_list(
    tmpl: web::Data<tera::Tera>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, Error> {

    let option_cookie = req.cookie("jwt_token");

    if option_cookie.is_none() {
        return Ok(
            HttpResponse::Found()
            .insert_header(("Location", "/signin"))
            .finish()
        );
    }

    let cookie = option_cookie.unwrap();
    // for safety，tutor_id need from jwt
    let tutor_id = get_tutor_id_from_token(cookie.value())
        .map_err(|_| EzyTutorError::JwtError("Invalid token".to_string()))?;

    dbg!(&tutor_id);


    let client = Client::default();
    let responce = client
        .get(format!("http://localhost:3000/courses/{}", tutor_id))
        .send()
        .await
        .unwrap()
        .body()
        .await
        .unwrap();
    let str_list = std::str::from_utf8(&responce.as_ref()).unwrap();
    dbg!(&str_list);
    let courses_list: Vec<GetCourseResponse> = serde_json::from_str(str_list).unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("courses", &courses_list);
    let s = tmpl
        .render("courses.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("TemplateError".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_insert_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    /*web::Path(tutor_id)*/ path: web::Path<i32>,
    params: web::Json<NewCourse>,
) -> Result<HttpResponse, Error> {
    let tutor_id = path.into_inner();
    let new_course = json!({
        "tutor_id": tutor_id,
        "course_name": &params.course_name,
        "course_description": &params.course_description,
        "course_format": &params.course_format,
        "course_structure": &params.course_structure,
        "course_duration": &params.course_duration,
        "course_price": &params.course_price,
        "course_language": &params.course_language,
        "course_level": &params.course_level

    });
    dbg!("handle_insert_course");
    dbg!(&tutor_id);
    dbg!(&new_course);
    let awc_client = awc::Client::default();
    let res = awc_client
        .post("http://localhost:3000/courses/")
        .send_json(&new_course)
        .await
        .unwrap()
        .body()
        .await?;
    println!("Finished call: {:?}", res);
    let course_response: NewCourseResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
    Ok(HttpResponse::Ok().json(course_response))
}

pub async fn handle_update_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    /*web::Path((tutor_id, course_id))*/ path: web::Path<(i32, i32)>,
    params: web::Json<UpdateCourse>,
) -> Result<HttpResponse, Error> {
    let (tutor_id, course_id) = path.into_inner();
    dbg!("handle_update_course");
    dbg!(&tutor_id);
    dbg!(&course_id);

    let update_course = json!({
        "course_name": &params.course_name,
        "course_description": &params.course_description,
        "course_format": &params.course_format,
        "course_duration": &params.course_duration,
       "course_structure": &params.course_structure,
        "course_price": &params.course_price,
        "course_language": &params.course_language,
        "course_level": &params.course_level,

    });
    let awc_client = awc::Client::default();
    let update_url = format!("http://localhost:3000/courses/{}/{}", tutor_id, course_id);
    let res = awc_client
        .put(update_url)
        .send_json(&update_course)
        .await
        .unwrap()
        .body()
        .await?;
    let course_response: UpdateCourseResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;

    Ok(HttpResponse::Ok().json(course_response))
}

// Handler function to delete a course for a tutor_id
pub async fn handle_delete_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    /*web::Path((tutor_id, course_id))*/ path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    let (tutor_id, course_id) = path.into_inner();
    let awc_client = awc::Client::default();
    let delete_url = format!("http://localhost:3000/courses/{}/{}", tutor_id, course_id);
    let _res = awc_client.delete(delete_url).send().await.unwrap();
    Ok(HttpResponse::Ok().body("Course deleted"))
}
