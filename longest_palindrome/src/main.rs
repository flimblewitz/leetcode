fn main() {
    println!("{}", Solution::longest_palindrome("civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth".into()));
}

struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut lp: String = s.chars().next().unwrap().to_string();
        //println!("{lp}");

        let chars: Vec<char> = s.chars().collect();

        for l in 0..s.len() {
            //println!("l:{l}");
            // for r in l..s.len() {
            'rloop: for r in ((l + lp.len())..s.len()).rev() {
                let substring_length = r - l + 1;
                //println!("r:{r}");
                let mut ltemp = l;
                let mut rtemp = r;
                loop {
                    if ltemp >= rtemp {
                        println!("{lp}, {l}, {r}");
                        lp = s.chars().skip(l).take(substring_length).collect();
                        break 'rloop;
                    }
                    if chars[ltemp] != chars[rtemp] {
                        break;
                    }
                    ltemp += 1;
                    rtemp -= 1;
                }
            }
        }

        lp
    }
}
