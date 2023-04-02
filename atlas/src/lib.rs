use macros::annotate;

#[annotate]
pub struct SomeObject {
    pub age: u32,
    pub speed: f64,
    pub name: String,
    pub surname: &'static str,
}

impl SomeObject {
    pub fn new() -> Self {
        Self {
            age: 21,
            speed: 1.0,
            name: String::from("John"),
            surname: "smith",
        }
    }
}
