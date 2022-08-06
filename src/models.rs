pub type Int = u16;

#[derive(serde::Serialize, Debug)]
pub struct User {
    pub(crate) Id: Int,
    pub(crate) Age: Int,
    pub(crate) FirstName: String,
    pub(crate) LastName: String,
    pub(crate) Framework: String,
}

const FRAMEWORK: &str = "Rust (actix";

impl User {
    pub fn new(id: Int, age: Int, f_name: String, l_name: String) -> Self {
        User {
            Id: id,
            Age: age,
            FirstName: f_name,
            LastName: l_name,
            Framework: FRAMEWORK.to_owned(),
        }
    }
}
