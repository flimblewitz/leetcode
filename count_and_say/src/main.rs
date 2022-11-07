fn main() {
    println!("{:?}", Solution::count_and_say(1));
    println!("{:?}", Solution::count_and_say(2));
    println!("{:?}", Solution::count_and_say(3));
    println!("{:?}", Solution::count_and_say(4));
}

struct Solution {}
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        recursively_count_and_say(n)
    }
}

fn recursively_count_and_say(n: i32) -> String {
    if n == 1 {
        return "1".into();
    }
    let chars: Vec<_> = recursively_count_and_say(n - 1).chars().collect();
    let mut prev = &chars[0];
    let mut num_repeat: u32 = 1;
    let mut s = String::new();
    for c in chars.iter().skip(1) {
        if *prev == *c {
            num_repeat += 1;
        } else {
            s.push(char::from_digit(num_repeat, 10).unwrap());
            s.push(*prev);
            prev = c;
            num_repeat = 1;
        }
    }
    // gotta do it again for the last char
    s.push(char::from_digit(num_repeat, 10).unwrap());
    s.push(*prev);
    s
}
