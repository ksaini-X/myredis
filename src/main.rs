use std::{assert_eq, collections::HashMap};

fn main() {}

struct Db {
    pub store: HashMap<String, String>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.store.insert(key, value)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }
    pub fn del(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }
}

#[test]
pub fn test() {
    let mut db = Db::new();

    let key1 = String::from("name");
    let key2 = String::from("age");
    let key3 = String::from("gender");
    let val1 = String::from("kartik");
    let val2 = String::from("26");
    let val3 = String::from("male");

    let res1 = db.set(key1, val1);
    let res2 = db.set(key2, val2);
    let res3 = db.set(key3, val3);

    assert_eq!(res1, None);
    assert_eq!(res2, None);
    assert_eq!(res3, None);

    let res4 = db.set(String::from("gender"), String::from("female"));
    assert_eq!(res4, Some(String::from("male")));

    let res5 = db.get("name");
    let res6 = db.get("age");
    let res7 = db.get("gender");
    let res8 = db.get("address");

    assert_eq!(res5, Some(&"kartik".to_string()));
    assert_eq!(res6, Some(&"26".to_string()));
    assert_eq!(res7, Some(&"female".to_string()));
    assert_eq!(res8, None);

    let res9 = db.del("name");
    let res10 = db.del("address");

    assert_eq!(res9, Some("kartik".to_string()));
    assert_eq!(res10, None);
}
