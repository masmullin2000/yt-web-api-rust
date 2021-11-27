#[derive(serde::Serialize)]
pub struct User {
    pub(crate) id: u16,
    pub(crate) age: u16,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) framework: String,
}
