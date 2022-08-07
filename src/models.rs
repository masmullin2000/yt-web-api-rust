use std::cell::RefCell;
use std::rc::Rc;

use lru::LruCache;

pub type Int = u16;
const FRAMEWORK: &str = "Rust (actix";

pub struct User {
    pub(crate) Id: Int,
    pub(crate) Age: Int,
    pub(crate) FirstName: String,
    pub(crate) LastName: String,
    pub(crate) Framework: &'static str, //String,
}

#[inline(always)]
pub fn get_int_string(int: Int) -> Rc<String> {
    thread_local! {
        static INT_STRINGS: RefCell<LruCache<Int, Rc<String>>> = RefCell::new(LruCache::new(10_000));
    }

    INT_STRINGS.with(|h| {
        let map = &mut *h.borrow_mut();

        if let Some(ret) = map.get_or_insert(int, || Rc::new(int.to_string())) {
            ret.clone()
        } else {
            panic!("this should not happen");
        }
    })
}

trait PushRawStr {
    fn push_raw(&mut self, s: &str);
}
impl PushRawStr for String {
    #[inline(always)]
    fn push_raw(&mut self, s: &str) {
        self.push('\"');
        self.push_str(s);
        self.push('\"');
    }
}

trait PushField<T> {
    fn push_field(&mut self, field: &str, t: T);
}
impl PushField<Int> for String {
    #[inline(always)]
    fn push_field(&mut self, field: &str, t: Int) {
        self.push_raw(field);
        self.push(':');
        let t_str = get_int_string(t);
        self.push_str(&t_str);
    }
}
impl PushField<&str> for String {
    #[inline(always)]
    fn push_field(&mut self, field: &str, t: &str) {
        self.push_raw(field);
        self.push(':');
        self.push_raw(t);
    }
}

pub trait JsonSerializeToString {
    fn serialize_to_string(&self, resp: &mut String);
}

impl User {
    #[inline(always)]
    pub fn new(id: Int, age: Int, f_name: String, l_name: String) -> Self {
        User {
            Id: id,
            Age: age,
            FirstName: f_name,
            LastName: l_name,
            Framework: FRAMEWORK,//.to_owned(),
        }
    }
}

impl JsonSerializeToString for User {
    fn serialize_to_string(&self, resp: &mut String) {
        // for Strings 
        macro_rules! push_field_last_str {
            ($field:ident) => {
                resp.push_field(stringify!($field), self.$field.as_str());
            };
        }

        // for anything that impls Copy
        macro_rules! push_field_last_cpy {
            ($field:ident) => {
                resp.push_field(stringify!($field), self.$field);
            };
        }

        macro_rules! push_field_str {
            ($field:ident) => {
                push_field_last_str!($field);
                resp.push(',');
            };
        }
        macro_rules! push_field_cpy {
            ($field:ident) => {
                push_field_last_cpy!($field);
                resp.push(',');
            };
        }

        resp.push('{');

        push_field_cpy!(Id);
        push_field_cpy!(Age);
        push_field_str!(FirstName);
        push_field_str!(LastName);
        push_field_last_cpy!(Framework);

        resp.push('}');
    }
}

impl JsonSerializeToString for &[User] {
    fn serialize_to_string(&self, resp: &mut String) {
        resp.push('[');
        for u in self.iter() {
            u.serialize_to_string(resp);
            resp.push(',');
        }
        resp.remove(resp.len() - 1);
        resp.push(']');
    }
}
