fn main() {
    {
        // test case 1
        let mut t = TimeMap::new();
        t.set("foo".into(), "bar".into(), 1); // store the key "foo" and value "bar" along with timestamp = 1.
        let v = t.get("foo".into(), 1); // return "bar"
        assert_eq!(v, "bar");
        let v = t.get("foo".into(), 3); // return "bar", since there is no value corresponding to foo at timestamp 3 and timestamp 2, then the only value is at timestamp 1 is "bar".
        assert_eq!(v, "bar");
        t.set("foo".into(), "bar2".into(), 4); // store the key "foo" and value "bar2" along with timestamp = 4.
        let v = t.get("foo".into(), 4); // return "bar2"
        assert_eq!(v, "bar2");
        let v = t.get("foo".into(), 5); // return "bar2"
        assert_eq!(v, "bar2");
    }
    {
        // test case 49
        let mut t = TimeMap::new();
        t.set("love".into(), "high".into(), 10);
        t.set("love".into(), "low".into(), 20);
        // println!("{:?}", t.h.get("love"));
        let v = t.get("love".into(), 5);
        assert_eq!(v, "");
        let v = t.get("love".into(), 10);
        assert_eq!(v, "high");
        let v = t.get("love".into(), 15);
        assert_eq!(v, "high");
        let v = t.get("love".into(), 20);
        assert_eq!(v, "low");
        let v = t.get("love".into(), 25);
        assert_eq!(v, "low");
    }
}

use std::collections::HashMap;
#[derive(Default)]
struct TimeMap {
    h: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self::default()
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        // void set(String key, String value, int timestamp) Stores the key key with the value value at the given time timestamp.
        self.h
            .entry(key)
            .and_modify(|values| {
                let mut l = 0;
                let mut r = values.len() - 1;
                while l < r {
                    let m = (l + r) / 2;
                    if timestamp <= values[m].0 {
                        r = m;
                    } else {
                        l = m + 1;
                    }
                }
                // there's a chance that this is the biggest timestamp we've seen
                // in that case, we want to make sure it goes _after_ the current last one
                if values[r].0 < timestamp {
                    r += 1;
                }
                values.insert(r, (timestamp, value.clone()));
            })
            .or_insert(vec![(timestamp, value)]);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        // String get(String key, int timestamp) Returns a value such that set was called previously, with timestamp_prev <= timestamp.
        // If there are multiple such values, it returns the value associated with the largest timestamp_prev.
        // If there are no values, it returns "".

        // println!("looking for {key} {timestamp}");

        if let Some(values) = self.h.get(&key) {
            let mut l = 0;
            let mut r = values.len() - 1;
            while l < r {
                let m = (l + r) / 2;
                if timestamp <= values[m].0 {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            // println!("r {r}");
            // values[r].0 is either a match, or it's the smallest thing that's bigger, or it's smaller but it's just the biggest thing we have
            // so if values[r].0 is bigger, and r isn't the first index, let's try the preceding index
            if r > 0 && timestamp < values[r].0 {
                r -= 1;
            }
            // println!("r {r}");
            if timestamp >= values[r].0 {
                return values[r].1.clone();
            }
        }

        "".into()
    }
}
