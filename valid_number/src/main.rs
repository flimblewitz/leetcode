fn main() {
    println!("{:?}", Solution::is_number("0".into()));
    println!("{:?}", Solution::is_number("70".into()));
    println!("{:?}", Solution::is_number("-70".into()));
    println!("{:?}", Solution::is_number("+70".into()));
    println!("{:?}", Solution::is_number("-90E3".into()));
    println!("{:?}", Solution::is_number("0089".into()));
    println!("{:?}", Solution::is_number(".5".into()));
    println!("{:?}", Solution::is_number("53.".into()));
    println!("{:?}", Solution::is_number("53.5".into()));
    println!("{:?}", Solution::is_number("53.5e93".into()));
    println!("{:?}", Solution::is_number("..".into()));
    println!("{:?}", Solution::is_number("92e1740e91".into())); // their description implies this should be allowed, but it's not
}
struct Solution {}
impl Solution {
    pub fn is_number(s: String) -> bool {
        // so I want to split into chars and then I have to determine if the rest of the chars are valid
        // I will consume the current char, and based on what it is, I'll pass the remainder to one of n possible validators
        is_integer_maybe_with_scientific_notation(&mut s.clone().chars())
            || is_decimal(&mut s.clone().chars())
    }
}

fn is_sign(c: char) -> bool {
    c == '+' || c == '-'
}
fn is_dot(c: char) -> bool {
    c == '.'
}
fn is_integer_maybe_with_scientific_notation(mut chars: &mut dyn Iterator<Item = char>) -> bool {
    let mut current = match chars.next() {
        Some(c) => c,
        _ => return false,
    };
    if is_sign(current) {
        current = match chars.next() {
            Some(c) => c,
            _ => return false,
        };
    }
    if !current.is_digit(10) {
        return false;
    }
    // that's the minimum requirement for an integer
    // now we need to permit any number of trailing digits ending in an optional scientific notation
    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            continue;
        }
        if is_scientific_notation(c, &mut chars) {
            return true;
        }
        return false;
    }

    true
}
// this is virtually identical to is_integer_maybe_with_scientific_notation
fn is_integer_without_scientific_notation(chars: &mut dyn Iterator<Item = char>) -> bool {
    let mut current = match chars.next() {
        Some(c) => c,
        _ => return false,
    };
    if is_sign(current) {
        current = match chars.next() {
            Some(c) => c,
            _ => return false,
        };
    }
    if !current.is_digit(10) {
        return false;
    }
    // that's the minimum requirement for an integer
    // now we need to permit any number of trailing digits ending in an optional scientific notation
    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            continue;
        }
        return false;
    }

    true
}
fn is_decimal(mut chars: &mut dyn Iterator<Item = char>) -> bool {
    let mut current = match chars.next() {
        Some(c) => c,
        _ => return false,
    };
    if is_sign(current) {
        current = match chars.next() {
            Some(c) => c,
            _ => return false,
        };
    }

    // our first non-sign char must either be a digit or dot
    match current {
        // first char is digit
        current if current.is_digit(10) => {
            while let Some(c) = chars.next() {
                // if that first digit is followed by a dot, there are nested implications and we must decide in here if it's valid
                if is_dot(c) {
                    // a dot must be the last thing, or it must be followed by any number of chars or terminate with scinetific notation
                    while let Some(c) = chars.next() {
                        if c.is_digit(10) {
                            continue;
                        }
                        // if it's not a digit, the last acceptable answer is scientific notation
                        return is_scientific_notation(c, &mut chars);
                    }
                    // if we got here, nothing followed the dot except may digits, and that's fine
                    return true;
                }
                // if the first digit is followed by more digits, cool
                if c.is_digit(10) {
                    continue;
                }
                // the only other option is to terminate with scientific notation
                return is_scientific_notation(c, &mut chars);
            }
        }
        // first char is dot
        '.' => {
            // it must be followed by at least one digit
            let current = chars.next();
            if current.is_none() {
                return false;
            }
            if let Some(c) = current {
                if !c.is_digit(10) {
                    return false;
                }
            }
            // at this point, we know it was followed by a digit

            // any remaining chars must compose extra digits or end with scientific notation
            while let Some(c) = chars.next() {
                if c.is_digit(10) {
                    continue;
                }
                return is_scientific_notation(c, &mut chars);
            }

            // if we got here, there were was no terminating scientific notation. It must have ended with digits
            return true;
        }
        _ => return false,
    }

    true
}
fn is_scientific_notation(current: char, chars: &mut dyn Iterator<Item = char>) -> bool {
    if current != 'e' && current != 'E' {
        return false;
    }
    is_integer_without_scientific_notation(chars)
}
