pub fn brackets_are_balanced(string: &str) -> bool {
    let brackets: &[(char,char)] = &[('{','}'), ('[',']'), ('(',')')];
    let string: String = String::from(string).replace(' ',"");
    let mut match_counter: usize = 0;
    let mut total_brackets: usize = 0;

    for c in string.chars() {
        if brackets.iter().find(|b| b.0 == c || b.1 == c  ).is_some() {
            total_brackets += 1;
        }
    }

    for (i,c) in string.chars().enumerate() {
        if let Some(b) = brackets.iter().find(|b| b.0 == c) {
            if let Some(secondary_string) = string.get(i+1..) {
                let mut opening_counter: i32 = 0;
                for sc in secondary_string.chars() {
                    if brackets.iter().find(|b| b.0 == sc).is_some() {
                        opening_counter += 1;
                    }
                    if sc == b.1 {
                        if opening_counter == 0 {
                            match_counter += 2;
                            break;
                        }
                    }                     
                    if brackets.iter().find(|b| b.1 == sc).is_some() {
                        opening_counter -= 1;
                    }
                }
            }
        }
    }
    if total_brackets == match_counter {
        return true;
    }
    false
}
#[test]
fn early_mismatched_brackets() {
    assert!(!brackets_are_balanced("{)()"));
}
#[test]
fn math_expression() {
    assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
}

#[test]
fn early_incomplete_brackets() {
    assert!(!brackets_are_balanced(")()"));
}
#[test]
fn paired_square_brackets() {
    assert!(brackets_are_balanced("[]"));
}
#[test]
fn empty_string() {
    assert!(brackets_are_balanced(""));
}
#[test]
fn unpaired_brackets() {
    assert!(!brackets_are_balanced("[["));
}
#[test]
fn wrong_ordered_brackets() {
    assert!(!brackets_are_balanced("}{"));
}
#[test]
fn wrong_closing_bracket() {
    assert!(!brackets_are_balanced("{]"));
}
#[test]
fn paired_with_whitespace() {
    assert!(brackets_are_balanced("{ }"));
}
#[test]
fn partially_paired_brackets() {
    assert!(!brackets_are_balanced("{[])"));
}
#[test]
fn simple_nested_brackets() {
    assert!(brackets_are_balanced("{[]}"));
}
#[test]
fn several_paired_brackets() {
    assert!(brackets_are_balanced("{}[]"));
}
#[test]
fn paired_and_nested_brackets() {
    assert!(brackets_are_balanced("([{}({}[])])"));
}
#[test]
fn unopened_closing_brackets() {
    assert!(!brackets_are_balanced("{[)][]}"));
}
#[test]
fn unpaired_and_nested_brackets() {
    assert!(!brackets_are_balanced("([{])"));
}
#[test]
fn paired_and_wrong_nested_brackets() {
    assert!(!brackets_are_balanced("[({]})"));
}
#[test]
fn paired_and_incomplete_brackets() {
    assert!(!brackets_are_balanced("{}["));
}
#[test]
fn too_many_closing_brackets() {
    assert!(!brackets_are_balanced("[]]"));
}
#[test]
fn complex_latex_expression() {
    let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                 \\end{array}\\right)";
    assert!(brackets_are_balanced(input));
}