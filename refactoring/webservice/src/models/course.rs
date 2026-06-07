use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::error::MyError;
use std::convert::TryFrom;

// use crate::models::course::Course;


#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,


}

#[derive(Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}



// impl From<web::Json<Course>> for Course {
//     fn from(course: web::Json<Course>) -> Self {
//         Course {
//             teacher_id: course.teacher_id,
//             id: course.id,
//             //  Rust 不允许直接从解引用的智能指针中移动值，需要使用 clone 方法
//             name: course.name.clone(),
//             time: course.time,
//         }
//     }
// }

// impl From<web::Json<Course>> for CreateCourse {
//     fn from(course: web::Json<Course>) -> Self {
//         CreateCourse {
//             teacher_id: course.teacher_id,
//             name: course.name.clone(),
//             description: course.description.clone(),
//             format: course.format.clone(),
//             structure: course.structure.clone(),
//             duration: course.duration.clone(),
//             price: course.price.clone(),
//             language: course.language.clone(),
//             level: course.level.clone(),
//         }
//     }
// }



impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = MyError;

    fn try_from(course: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            description: course
                .description
                .clone(),
            format: course
                .format
                .clone(),
            structure: course
                .structure
                .clone(),
            duration: course
                .duration
                .clone(),
            price: course.price,
            language: course
                .language
                .clone(),
            level: course.level.clone(),
        })
    }
}



impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: course.name.clone(),
            description: course
                .description
                .clone(),
            format: course
                .format
                .clone(),
            structure: course
                .structure
                .clone(),
            duration: course
                .duration
                .clone(),
            price: course.price,
            language: course
                .language
                .clone(),
            level: course.level.clone(),
        }
    }
}