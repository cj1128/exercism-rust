pub fn reply(msg: &str) -> &str {
    let msg = msg.trim();

    let yell =  msg.to_uppercase() == msg && msg.to_lowercase() != msg;
    let question = msg.ends_with("?");
    let silent = msg.is_empty();

    match (yell, question, silent) {
        (_, _, true) => "Fine. Be that way!",
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, _, _) => "Whoa, chill out!",
        (_, true, _) => "Sure.",
        _ => "Whatever.",
    }
}
