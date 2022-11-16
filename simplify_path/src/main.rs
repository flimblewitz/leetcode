fn main() {
    println!("{:?}", Solution::simplify_path("/home/".into()));
    println!("{:?}", Solution::simplify_path("/../".into()));
    println!("{:?}", Solution::simplify_path("/..hi/.hoho./.com/".into()));
    println!("{:?}", Solution::simplify_path("/home//foo/".into()));
    println!("{:?}", Solution::simplify_path("/home//foo/..".into()));
    println!("{:?}", Solution::simplify_path("/home//foo/../".into()));
    println!("{:?}", Solution::simplify_path("/home//foo/...".into()));
    println!("{:?}", Solution::simplify_path("/home//foo/.../".into()));
    println!("{:?}", Solution::simplify_path("/home//foo/../hoi".into()));
    println!("{:?}", Solution::simplify_path("/home//foo/.../hoi".into()));
}
struct Solution {}

enum PathToken {
    Slash,
    Name(String),
}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        // so I can compose a list of path tokens
        let mut v = vec![PathToken::Slash];
        let mut chars = path.chars();
        // it feels like every time I get a char, I have to consider the possibilities and pass the iterator to a func which will return a path token and the current char
        let mut next = chars.next();
        let mut i = 0;
        while let Some(c) = next {
            match (c, &mut v[i]) {
                ('/', PathToken::Slash) => next = chars.next(),
                ('/', PathToken::Name(_)) => {
                    v.push(PathToken::Slash);
                    i += 1;
                    next = chars.next();
                }
                ('.', PathToken::Slash) => {
                    next = chars.next();
                    match next {
                        None => break,
                        Some('/') => next = chars.next(),
                        Some('.') => {
                            next = chars.next();
                            match next {
                                // None => break,
                                None | Some('/') => {
                                    v.pop();
                                    v.pop();
                                    i = i.checked_sub(2).unwrap_or(i);
                                    // i -= 2;
                                    if v.is_empty() {
                                        v.push(PathToken::Slash);
                                        // i += 1;
                                    }
                                    next = chars.next();
                                }
                                Some(c) => {
                                    v.push(PathToken::Name(format!("..{c}")));
                                    i += 1;
                                    next = chars.next();
                                }
                            }
                        }
                        Some(c) => {
                            v.push(PathToken::Name(format!(".{c}")));
                            i += 1;
                            next = chars.next();
                        }
                    }
                }
                (c, PathToken::Name(name)) => {
                    name.push(c);
                    next = chars.next();
                }
                (c, PathToken::Slash) => {
                    v.push(PathToken::Name(c.into()));
                    i += 1;
                    next = chars.next();
                }
            }
        }
        let mut s: String = v.iter().fold("".into(), |mut acc, token| match token {
            PathToken::Slash => {
                acc.push('/');
                acc
            }
            PathToken::Name(name) => {
                acc.push_str(name);
                acc
            }
        });

        if s.len() > 1 && s.ends_with('/') {
            s.pop();
        }

        s
    }
}
