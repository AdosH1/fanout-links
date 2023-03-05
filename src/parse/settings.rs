use std::collections::HashMap;

pub fn add_custom_link(
    l: Option<HashMap<String, String>>,
    kv: (String, String),
) -> Option<HashMap<String, String>> {
    match l {
        Some(mut hm) => {
            hm.insert(kv.0, kv.1);
            Some(hm)
        }
        None => {
            let mut hm = HashMap::new();
            hm.insert(kv.0, kv.1);
            Some(hm)
        }
    }
}
