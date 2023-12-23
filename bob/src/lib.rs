pub fn is_silence(message: &str) -> bool {
    message.trim().is_empty()
}

pub fn is_a_yell(message: &str) -> bool {
    message.to_uppercase() == message && message.to_lowercase() != message
}

pub fn is_a_question(message: &str) -> bool {
    message.trim().ends_with("?")
}

pub fn is_yell_a_question(message: &str) -> bool {
    is_a_yell(message) && is_a_question(message)
}
pub fn reply(message: &str) -> &str {
    if is_silence(message) {
        return "Fine. Be that way!";
    }

    if is_yell_a_question(message) {
        return "Calm down, I know what I'm doing!";
    }

    if is_a_yell(message) {
        return "Whoa, chill out!";
    }

    if is_a_question(message) {
        return "Sure.";
    }

    return "Whatever.";
}
