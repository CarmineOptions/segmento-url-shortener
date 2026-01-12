use url::Url;

pub fn is_valid_url(input: &str) -> bool {
    let Ok(url) = Url::parse(input) else {
        return false;
    };

    // Allow only http and https
    match url.scheme() {
        "http" | "https" => {}
        _ => return false,
    }

    let Some(host) = url.host_str() else {
        return false;
    };

    // Block localhost
    if host == "localhost" {
        return false;
    }

    true
}

pub fn is_valid_code(input: &str) -> bool {
    if input.len() < 2 || input.len() > 10 {
        return false;
    }
    if !input.bytes().all(|b| {
        (b'0'..=b'9').contains(&b) || (b'A'..=b'Z').contains(&b) || (b'a'..=b'z').contains(&b)
    }) {
        return false;
    }
    return true;
}
