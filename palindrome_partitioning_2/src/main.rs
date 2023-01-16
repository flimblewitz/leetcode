use std::time::SystemTime;
fn main() {
    println!("{}", Solution::min_cut("abab".into()));
    // println!("{}", FirstPassSolution::min_cut("abab".into()));
    // println!("{}", OtherSolution::min_cut("aab".into()));
    // println!("{}", Solution::min_cut("aab".into()));
    // println!("{}", Solution::min_cut("a".into()));
    // println!("{}", Solution::min_cut("ab".into()));
    // println!("{}", Solution::min_cut("abbayabbay".into()));
    // println!("{}", Solution::min_cut("aba".into()));
    // println!("{}", OtherSolution::min_cut("aaabaa".into()));
    // println!("{}", Solution::min_cut("aaabaa".into()));
    // println!("{}", OtherSolution::min_cut("aabbcc".into()));
    // println!("{}", Solution::min_cut("aabbcc".into()));
    // println!("{}", Solution::min_cut("abaabaa".into()));
    // println!("{}", Solution::min_cut("abaabaabaa".into()));
    // 273
    // println!("{}", Solution::min_cut("adabdcaebdcebdcacaaaadbbcadabcbeabaadcbcaaddebdbddcbdacdbbaedbdaaecabdceddccbdeeddccdaabbabbdedaaabcdadbdabeacbeadbaddcbaacdbabcccbaceedbcccedbeecbccaecadccbdbdccbcbaacccbddcccbaedbacdbcaccdcaadcbaebebcceabbdcdeaabdbabadeaaaaedbdbcebcbddebccacacddebecabccbbdcbecbaeedcdacdcbdbebbacddddaabaedabbaaabaddcdaadcccdeebcabacdadbaacdccbeceddeebbbdbaaaaabaeecccaebdeabddacbedededebdebabdbcbdcbadbeeceecdcdbbdcbdbeeebcdcabdeeacabdeaedebbcaacdadaecbccbededceceabdcabdeabbcdecdedadcaebaababeedcaacdbdacbccdbcece".into()));

    // let now = SystemTime::now();
    // println!("{}", OtherSolution::min_cut("fiefhgdcdcgfeibggchibffahiededbbegegdfibdbfdadfbdbceaadeceeefiheibahgececggaehbdcgebaigfacifhdbecbebfhiefchaaheiichgdbheacfbhfiaffaecicbegdgeiaiccghggdfggbebdaefcagihbdhhigdgbghbahhhdagbdaefeccfiaifffcfehfcdiiieibadcedibbedgfegibefagfccahfcbegdfdhhdgfhgbchiaieehdgdabhidhfeecgfiibediiafacagigbhchcdhbaigdcedggehhgdhedaebchcafcdehcffdiagcafcgiidhdhedgaaegdchibhdaegdfdaiiidcihifbfidechicighbcbgibadbabieaafgeagfhebfaheaeeibagdfhadifafghbfihehgcgggffgbfccgafigieadfehieafaehaggeeaaaehggffccddchibegfhdfafhadgeieggiigacbfgcagigbhbhefcadafhafdiegahbhccidbeeagcgebehheebfaechceefdiafgeddhdfcadfdafbhiifigcbddahbabbeedidhaieagheihhgffbfbiacgdaifbedaegbhigghfeiahcdieghhdabdggfcgbafgibiifdeefcbegcfcdihaeacihgdchihdadifeifdgecbchgdgdcifedacfddhhbcagaicbebbiadgbddcbagbafeadhddaeebdgdebafabghcabdhdgieiahggddigefddccfccibifgbfcdccghgceigdfdbghdihechfabhbacifgbiiiihcgifhdbhfcaiefhccibebcahidachfabicbdabibiachahggffiibbgchbidfbbhfcicfafgcagaaadbacddfiigdiiffhbbehaaacidggfbhgeaghigihggfcdcidbfccahhgaffiibbhidhdacacdfebedbiacaidaachegffaiiegeabfdgdcgdacfcfhdcbfiaaifgfaciacfghagceaaebhhibbieehhcbiggabefbeigcbhbcidbfhfcgdddgdffghidbbbfbdhcgabaagddcebaechbbiegeiggbabdhgghciheabdibefdfghbfbfebidhicdhbeghebeddgfdfhefebiiebdchifbcbahaddhbfafbbcebiigadhgcfbebgbebhfddgdeehhgdegaeedfadegfeihcgeefbbagbbacbgggciehdhiggcgaaicceeaefgcehfhfdciaghcbbgdihbhecfbgffefhgiefgeiggcebgaacefidghdfdhiabgibchdicdehahbibeddegfciaeaffgbefbbeihbafbagagedgbdadfdggfeaebaidchgdbcifhahgfdcehbahhdggcdggceiabhhafghegfdiegbcadgaecdcdddfhicabdfhbdiiceiegiedecdifhbhhfhgdbhibbdgafhgdcheefdhifgddchadbdggiidhbhegbdfdidhhfbehibiaacdfbiagcbheabaaebfeaeafbgigiefeaeheabifgcfibiddadicheahgbfhbhddaheghddceedigddhchecaghdegigbegcbfgbggdgbbigegffhcfcbbebdchffhddbfhhfgegggibhafiebcfgeaeehgdgbccbfghagfdbdfcbcigbigaccecfehcffahiafgabfcaefbghccieehhhiighcfeabffggfchfdgcfhadgidabdceediefdccceidcfbfiiaidechhbhdccccaigeegcaicabbifigcghcefaafaefd".into()));
    // println!(
    //     "ms duration for theirs: {}",
    //     now.elapsed().unwrap().as_millis()
    // );

    // let now = SystemTime::now();
    // println!("{}", Solution::min_cut("fiefhgdcdcgfeibggchibffahiededbbegegdfibdbfdadfbdbceaadeceeefiheibahgececggaehbdcgebaigfacifhdbecbebfhiefchaaheiichgdbheacfbhfiaffaecicbegdgeiaiccghggdfggbebdaefcagihbdhhigdgbghbahhhdagbdaefeccfiaifffcfehfcdiiieibadcedibbedgfegibefagfccahfcbegdfdhhdgfhgbchiaieehdgdabhidhfeecgfiibediiafacagigbhchcdhbaigdcedggehhgdhedaebchcafcdehcffdiagcafcgiidhdhedgaaegdchibhdaegdfdaiiidcihifbfidechicighbcbgibadbabieaafgeagfhebfaheaeeibagdfhadifafghbfihehgcgggffgbfccgafigieadfehieafaehaggeeaaaehggffccddchibegfhdfafhadgeieggiigacbfgcagigbhbhefcadafhafdiegahbhccidbeeagcgebehheebfaechceefdiafgeddhdfcadfdafbhiifigcbddahbabbeedidhaieagheihhgffbfbiacgdaifbedaegbhigghfeiahcdieghhdabdggfcgbafgibiifdeefcbegcfcdihaeacihgdchihdadifeifdgecbchgdgdcifedacfddhhbcagaicbebbiadgbddcbagbafeadhddaeebdgdebafabghcabdhdgieiahggddigefddccfccibifgbfcdccghgceigdfdbghdihechfabhbacifgbiiiihcgifhdbhfcaiefhccibebcahidachfabicbdabibiachahggffiibbgchbidfbbhfcicfafgcagaaadbacddfiigdiiffhbbehaaacidggfbhgeaghigihggfcdcidbfccahhgaffiibbhidhdacacdfebedbiacaidaachegffaiiegeabfdgdcgdacfcfhdcbfiaaifgfaciacfghagceaaebhhibbieehhcbiggabefbeigcbhbcidbfhfcgdddgdffghidbbbfbdhcgabaagddcebaechbbiegeiggbabdhgghciheabdibefdfghbfbfebidhicdhbeghebeddgfdfhefebiiebdchifbcbahaddhbfafbbcebiigadhgcfbebgbebhfddgdeehhgdegaeedfadegfeihcgeefbbagbbacbgggciehdhiggcgaaicceeaefgcehfhfdciaghcbbgdihbhecfbgffefhgiefgeiggcebgaacefidghdfdhiabgibchdicdehahbibeddegfciaeaffgbefbbeihbafbagagedgbdadfdggfeaebaidchgdbcifhahgfdcehbahhdggcdggceiabhhafghegfdiegbcadgaecdcdddfhicabdfhbdiiceiegiedecdifhbhhfhgdbhibbdgafhgdcheefdhifgddchadbdggiidhbhegbdfdidhhfbehibiaacdfbiagcbheabaaebfeaeafbgigiefeaeheabifgcfibiddadicheahgbfhbhddaheghddceedigddhchecaghdegigbegcbfgbggdgbbigegffhcfcbbebdchffhddbfhhfgegggibhafiebcfgeaeehgdgbccbfghagfdbdfcbcigbigaccecfehcffahiafgabfcaefbghccieehhhiighcfeabffggfchfdgcfhadgidabdceediefdccceidcfbfiiaidechhbhdccccaigeegcaicabbifigcghcefaafaefd".into()));
    // println!(
    //     "ms duration for mine: {}",
    //     now.elapsed().unwrap().as_millis()
    // );
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        // I'm slightly tweaking someone else's solution here
        // the idea is that we can consider all possible substrings from left to right and accumulate knowledge about whether these substrings are palindromes, which will then inform us if larger substrings - ones that immediately contain them between two fringe chars - are themselves palindromes
        // this works because it strictly inspects all substrings from smallest to largest as it gradually expands the window
        // so, given a substring,
        //  if the substring is 0 or 1 chars, it's a palindrome
        //  if the substring's fringe chars match and the inner substring - which we must have already seen - is itself a palindrome, then so too must be this substring
        // the reasons why this is so much faster than a solution where you simply try to find the largest palindrome and then split into a smaller subproblems from there is that each of those is basically its own whole discrete problem whereas this approach continuously builds upon relevant information since it's expanding a window
        let s = s.as_bytes();
        let mut palindrome_statuses = vec![vec![false; s.len()]; s.len()];
        let mut min_cuts_given_end = vec![0; s.len()];
        for end in 0..s.len() {
            let mut min_cuts = end as i32;
            for start in 0..=end {
                // if substring is 0 or 1 char
                // if substring's outer chars match and the inner substring is a known palindrome
                // if (start + 1 >= end || palindrome_statuses[start + 1][end - 1])
                if (end - start <= 1 || palindrome_statuses[start + 1][end - 1])
                    && s[start] == s[end]
                {
                    // println!("{}, {} is a palindrome", start, end,);
                    palindrome_statuses[start][end] = true;
                    min_cuts = if start == 0 {
                        // println!("{}, {} min_cuts is 0", start, end,);
                        // if we're starting at 0, and this is a palindrome, no cuts are needed!
                        0
                    } else {
                        min_cuts.min(min_cuts_given_end[start - 1] + 1)
                    };
                }
            }
            min_cuts_given_end[end] = min_cuts;
        }
        min_cuts_given_end[s.len() - 1]
    }
}

struct FirstPassSolution;

impl FirstPassSolution {
    pub fn min_cut(s: String) -> i32 {
        // num_cuts_looking_left(&s[..]).min(1 + num_cuts_looking_left(&s[1..]))

        // let's consider the best number of palindromes within some &s[i..j]
        // for each possible cut, save the min subsequent cuts
        let mut palindrome_statuses: HashMap<&[u8], bool> = HashMap::new();
        // let mut min_cuts_in_substrings: HashMap<(usize, usize), i32> = HashMap::new();
        // get_min_cuts_in_substring(
        //     &s,
        //     0,
        //     s.len(),
        //     &mut min_cuts_in_substrings,
        //     &mut palindrome_statuses,
        // )
        let mut min_cuts_in_substrings: HashMap<&[u8], i32> = HashMap::new();
        get_min_cuts_in_substring(
            s.as_bytes(),
            &mut min_cuts_in_substrings,
            &mut palindrome_statuses,
        )
    }
}

fn get_min_cuts_in_substring<'a>(
    s: &'a [u8],
    min_cuts_in_substrings: &mut HashMap<&'a [u8], i32>,
    palindrome_statuses: &mut HashMap<&'a [u8], bool>,
) -> i32 {
    if let Some(min_cuts) = min_cuts_in_substrings.get(s.into()) {
        return *min_cuts;
    }

    if is_palindrome(s, palindrome_statuses) {
        min_cuts_in_substrings.insert(s.into(), 0);
        return 0;
    }

    let max_possible_cuts = s.len() as i32 - 1;
    let mut min_cuts = max_possible_cuts;

    // ultimately, we know this string ends in a palindrome (even if it's only 1 char). Let's look for the longest ending palindrome and memoize what substrings are/aren't palindromes as we go
    // once we've found that longest ending palindrome, we have broken down this problem into a subproblem: we need to consider the "prefix" substring that was left over
    for cut_index in 1..s.len() {
        if is_palindrome(&s[cut_index..], palindrome_statuses) {
            let num_cuts_on_left = get_min_cuts_in_substring(
                &s[..cut_index],
                min_cuts_in_substrings,
                palindrome_statuses,
            );
            min_cuts = min_cuts.min(num_cuts_on_left);
        }
    }

    // add one because we implicitly already made a cut before that for loop above
    min_cuts += 1;

    min_cuts_in_substrings.insert(s.into(), min_cuts);
    min_cuts
}

fn is_palindrome<'a>(s: &'a [u8], palindrome_statuses: &mut HashMap<&'a [u8], bool>) -> bool {
    let len = s.len();
    if len <= 1 {
        return true;
    }
    if let Some(&palindrome_status) = palindrome_statuses.get(s) {
        // println!(
        //     "we already know that {} is a palindrome: {}",
        //     s, palindrome_status
        // );
        return palindrome_status;
    }

    // this is my naive, human-friendly implementation that is maybe slightly suboptimal
    let palindrome_status = (0..len / 2).all(|i| s[i] == s[len - i - 1]);

    // this is an idea I saw that seems slightly more optimal, probably because it leverages memoization
    // let palindrome_status = if s[..1] == s[len - 1..] {
    //     is_palindrome(&s[1..len - 1], palindrome_statuses)
    // } else {
    //     false
    // };

    palindrome_statuses.insert(s, palindrome_status);
    palindrome_status
}
