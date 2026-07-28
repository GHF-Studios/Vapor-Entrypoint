fn redacted_arguments(arguments: &[OsString]) -> Vec<String> {
    let mut redact_next = false;
    let mut redacted = Vec::new();
    for argument in arguments {
        let argument = argument.to_string_lossy();
        if redact_next {
            redacted.push("<redacted>".to_owned());
            redact_next = false;
            continue;
        }
        if let Some((name, _)) = argument.split_once('=')
            && is_sensitive_name(name.trim_start_matches('-'))
        {
            redacted.push(format!("{name}=<redacted>"));
            continue;
        }
        let name = argument.trim_start_matches('-');
        if argument.starts_with('-') && is_sensitive_name(name) {
            redacted.push(argument.into_owned());
            redact_next = true;
        } else {
            redacted.push(argument.into_owned());
        }
    }
    redacted
}

fn is_sensitive_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    if [
        "password",
        "passwd",
        "token",
        "secret",
        "credential",
        "credentials",
        "cookie",
        "authorization",
        "refresh_token",
        "access_token",
        "authticket",
        "auth_ticket",
        "sessionticket",
        "session_ticket",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
    {
        return true;
    }
    lower
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|part| matches!(part, "key" | "auth" | "ticket"))
}
