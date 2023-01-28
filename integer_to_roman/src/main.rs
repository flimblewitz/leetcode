fn main() {
    println!("{}", Solution::int_to_roman(3));
    println!("{}", Solution::int_to_roman(58));
    println!("{}", Solution::int_to_roman(1994));
}

struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        // blow up after each step
        // divide num by 1000 to get m. If > 3, blow up
        // if >= 900, CM
        // else if >= 500, D
        // now we're in the realm of 100 and below. Repeat like M
        // now we're in the realm of 90 or 50 and below. Repeat like CM and D
        // keep going, stopping with I like M

        let mut s = String::new();

        let mut reduction = num;

        Self::accumulate(&mut reduction, &mut s, 1000, 'M', 'D', 'C');
        Self::accumulate(&mut reduction, &mut s, 100, 'C', 'L', 'X');
        Self::accumulate(&mut reduction, &mut s, 10, 'X', 'V', 'I');
        (0..reduction).for_each(|_| s.push('I'));

        s
    }

    fn accumulate(
        reduction: &mut i32,
        s: &mut String,
        power_ten: i32,
        ten_char: char,
        five_char: char,
        one_char: char,
    ) {
        let power_ten_multiple = *reduction / power_ten;
        if power_ten_multiple > 3 {
            panic!("oops: {}", ten_char);
        }
        (0..power_ten_multiple).for_each(|_| s.push(ten_char));
        *reduction -= power_ten * power_ten_multiple;
        if *reduction >= power_ten * 9 / 10 {
            s.push(one_char);
            s.push(ten_char);
            *reduction -= power_ten * 9 / 10;
        } else if *reduction >= power_ten / 2 {
            s.push(five_char);
            *reduction -= power_ten / 2;
        } else if *reduction >= power_ten * 4 / 10 {
            s.push(one_char);
            s.push(five_char);
            *reduction -= power_ten * 4 / 10;
        }
    }
}
