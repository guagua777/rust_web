use super::handlers::*;
use actix_web::web;


pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            // to的参数为什么是一个异步函数？
            // 为什么一个异步函数是一个Handler? blanket implementation
            .route("/", web::post().to(new_course))
            .route("/{user_id}", web::get().to(get_courses_for_teacher))
            .route("/{user_id}/{course_id}", web::get().to(get_course_detail))
    );
}