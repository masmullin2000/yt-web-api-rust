#[derive(serde::Serialize)]
pub struct User {
    pub Id: u16,
    pub Age: u16,
    pub First_Name: String,
    pub Last_Name: String,
    pub Framework: String,
}
