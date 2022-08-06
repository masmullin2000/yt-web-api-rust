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

fn push_raw(s: &mut String, d: &str) {
    s.push('\"');
    s.push_str(d);
    s.push('\"');
}

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

    // this function could take half the time if we didn't have to
    // turn Id and Age into Strings
    pub fn fill_json_string(&self, resp: &mut String) {
        resp.push('{');

        push_raw(resp, "Id");
        resp.push(':');
        resp.push_str(&self.Id.to_string());
        //resp.push_str("000");
        resp.push(',');

        push_raw(resp, "Age");
        resp.push(':');
        resp.push_str(&self.Age.to_string());
        //resp.push_str("000");
        resp.push(',');

        push_raw(resp, "FirstName");
        resp.push(':');
        push_raw(resp, &self.FirstName);
        resp.push(',');

        push_raw(resp, "LastName");
        resp.push(':');
        push_raw(resp, &self.LastName);
        resp.push(',');

        push_raw(resp, "Framework");
        resp.push(':');
        push_raw(resp, &self.Framework);
        resp.push('}');
    }
}
