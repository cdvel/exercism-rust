pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let chars = string.trim().chars();

    for c in chars {
        let t = *stack.last().unwrap_or(&' ');
        match c {
            '{' | '[' | '(' => stack.push(c),
            '}' | ']' | ')' => {
                if stack.is_empty() {
                    return false;
                }
                stack.pop();
                match c {
                    '}' => {
                        if t == '(' || t == '[' {
                            return false;
                        }
                    }
                    ']' => {
                        if t == '{' || t == '(' {
                            return false;
                        }
                    }
                    ')' => {
                        if t == '{' || t == '[' {
                            return false;
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
