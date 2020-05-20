pub fn reply(message: &str) -> &str {
    let msg = message.trim().to_owned();

    let any_lower = msg.chars().any(|c| c.is_lowercase());
    let no_alpha = msg.chars().all(|c| !c.is_alphabetic());

    match (msg.ends_with('?'), !any_lower && !no_alpha, msg.is_empty()) {
        (true, false, false) => "Sure.",
        (false, true, false) => "Whoa, chill out!",
        (true, true, false) => "Calm down, I know what I'm doing!",
        (_, _, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
