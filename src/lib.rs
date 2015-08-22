// upper - helper function to uppercase chars
fn upper(c: char) -> char {
    let upper_c: Option<char> = c.to_uppercase().next();
    if upper_c.is_some() {
        return upper_c.unwrap();
    } else {
        return ' '
    }
}

// american_soundex_code - helper function that responds with the code based on the char
fn soundex_code(c: char) -> char {
    // soundex processing, replacements of letters
    match upper(c) {
        'B' | 'F' | 'P' | 'V' => {
            // replace with 1
            return '1';
        },
        'C' | 'G' | 'J' | 'K' | 'Q' | 'S' | 'X' | 'Z' => {
            // replace with 2 
            return '2';
        },
        'D' | 'T' => {
            // replace with 3
            return '3';
        },
        'L' => {
            // replace with 4
            return '4';
        },
        'M' | 'N' => {
            // replace with 5
            return '5';
        },
        'R' => {
            // replace with 6
            return '6';
        },
         'A' | 'E' | 'I' | 'O' | 'U' | 'Y' | 'H' | 'W' => {
            // drop these, they aren't used for soundex
            return ' ';
        },
        _ => {
            // default to add a zero
            return '0';
        }
    }
}

/// Performs Soundex calculation on a string passed in
///
/// # Examples
///
/// ```
/// use soundex;
/// let code: String = soundex::american_soundex("Sirname".to_string());
/// ```
pub fn american_soundex(s: String) -> String {
    // Define a result
    let mut result = String::from("");
    // if we need to skip, outline which element to skip
    let mut skip_i: usize = 0;
    let mut count:i32 = 0;
    let w_code = soundex_code('w');
    let h_code = soundex_code('h');
    // iterate over chars enumerated
    for (i, v) in s.chars().enumerate() {
        if i != 0 && skip_i == i {
            // if this isn't the first, and skip index == current index break
            continue;
        }
        if count > 3 {
            // if the lenght is > 3, done
            break;
        } else if count >= 0 {
            // if we have more than 0 elements, perform the indexing
            // if the next is a duplicate, just move to next
            let mut next: char = ' ';
            let mut next_next: char = ' ';

            // grab next element, and the element after the next
            let next_opt = s.chars().nth(i+1);
            let next_next_opt = s.chars().nth(i+2);

            if next_opt.is_some() {
                // validate this contains something before trying to unwrap
                next = upper(next_opt.unwrap());
            }
            if next_next_opt.is_some() {
                // validate this contains something before trying to unwrap
                next_next = upper(next_next_opt.unwrap());
            }

            let code: char = soundex_code(v);
            let next_code: char = soundex_code(next);
            let next_next_code: char = soundex_code(next_next);

            if i + 1 <= s.len() && next_code == h_code ||  next_code == w_code {
                // if the next is an h or w, compare current to the letter after the h or w, for
                // duplication
                if i +2 <= s.len() && code == next_next_code {
                    skip_i = i + 2;
                }
            }
            // if there are two duplicates in a row, remove the duplicate
            if i + 1 <= s.len() && code == next_code {
                skip_i = i + 1;
            }

            if count == 0 {
                // this is the first char in the string, goes through
                result.push(upper(v));
            } else {
                // push the code value to the result
                if code == ' ' {
                    continue;
                }
                result.push(code);
            }
        }
        // increment counter
        count+=1;
    }
    // we didn't have enough chars in the string to fill 
    // so we will backfill the last positions with '0'
    while count <= 3 {
        result.push('0');
        count+=1;
    }
    // return the result
    return result
}

#[test]
fn american_soundex_correct() {
    let params: Vec<(&str, &str)> = vec![
        ("Pfister","P236"),
        ("husobee","H210"),
        ("Tymczak","T522"),
        ("Ashcraft","A261"),
        ("Robert","R163"),
        ("Rupert","R163"),
        ("Rubin","R150"),
    ];
    for (i, o) in params {
        assert_eq!(american_soundex(i.to_string()), o.to_string());
    }
}
