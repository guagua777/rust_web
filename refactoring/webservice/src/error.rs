use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

// 自定义错误类型

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    // #[allow(dead_code)]
    NotFound(String),
}

// 定义错误响应
// 自定义错误类型要转化为自定义错误响应
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            MyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}


// 实现trait
impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(_msg) | MyError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        println!("error_response........");
        HttpResponse::build(self.status_code())
        // Set a JSON body and build the `HttpResponse`.
        .json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for MyError {
    // f是传入的参数，里面包含要输出到的地方，
    // Formatter 获取上下文信息的结构体
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        println!("my error format........");
        // 这是什么意思
        // 难道这里不无限递归吗？ 会无限递归
        write!(f, "no recursive? : {}", self)
    }
}

// 将actix_web的错误类型转换为自定义错误类型
impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

// 将sqlx的错误类型转换为自定义错误类型
impl From<SQLxError> for MyError {
    fn from(err: SQLxError) -> Self {
        MyError::DBError(err.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let err = MyError::DBError("test error".to_string());
        assert_eq!(err.to_string(), "no recursive? : test error");
    }

}
