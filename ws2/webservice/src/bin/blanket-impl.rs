// 定义一个基础 trait
trait Speak {
    fn say_hi(&self) -> &str;
}

// 定义一个扩展 trait
trait Greet {
    fn greet(&self);
}

// ✨ Blanket Implementation：
// 为 所有实现了 Speak 的类型，自动实现 Greet
impl<T: Speak> Greet for T {
    fn greet(&self) {
        println!("Hello! {}", self.say_hi());
    }
}

// 给两个类型实现 Speak
struct Person;
impl Speak for Person {
    fn say_hi(&self) -> &str {
        "I'm a person"
    }
}

struct Dog;
impl Speak for Dog {
    fn say_hi(&self) -> &str {
        "Woof!"
    }
}

fn main() {
    // Person 和 Dog 都自动拥有了 greet 方法！
    let p = Person;
    p.greet();
    
    let d = Dog;
    d.greet();
}

