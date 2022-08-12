use serde::Serialize;

pub type Int = u16;
const FRAMEWORK: &str = "Rust (actix";

#[derive(Serialize)]
pub struct User {
    pub(crate) Id: Int,
    pub(crate) Age: Int,
    pub(crate) FirstName: String,
    pub(crate) LastName: String,
    pub(crate) Framework: &'static str, //String,
}

impl User {
    #[inline(always)]
    pub fn new(id: Int, age: Int, f_name: String, l_name: String) -> Self {
        User {
            Id: id,
            Age: age,
            FirstName: f_name,
            LastName: l_name,
            Framework: FRAMEWORK, //.to_owned(),
        }
    }
}
