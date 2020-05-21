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
                match c{
                    '}' if "([".contains(t) => return false,
                    ']' if "({".contains(t) => return false,
                    ')' if "{[".contains(t) => return false,
                    _ => continue
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
