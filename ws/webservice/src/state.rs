use std::sync::Mutex;

use crate::models::Course;

pub struct AppState {
    pub health_check_response: String,
    // 使用Mutex来确保线程安全的访问
    // 
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
}