use std::num::ParseIntError;

fn main() {
    let result = square("25");
    println!("{:?}", result);

    let result = square("err");
    println!("{:?}", result);


    let result = square2("err");
    println!("{:?}", result);
}

// 错误的处理方式：
// 返回错误
fn square(val: &str) -> Result<i32, ParseIntError> {
    match val.parse::<i32>() {
        Ok(num) => Ok(num.pow(2)),
        Err(e) => Err(e),
    }
}

// 使用?时，只管ok分支
fn square2(val: &str) -> Result<i32, ParseIntError> {
    Ok(val.parse::<i32>()?)
}

#[derive(Debug)]
pub enum MyError {
    ParseError,
    IOError,
    OtherError,
}