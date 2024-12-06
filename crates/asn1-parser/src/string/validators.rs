pub fn validate_utf8(_: &str) -> bool {
    true
}

pub fn validate_general(_: &str) -> bool {
    // TODO: properly validate GeneralString
    // I'm too lazy to do it. UTF8 is enough
    true
}

pub fn validate_printable(data: &str) -> bool {
    const ALLOWED_SPECIAL: &[u8] = b" '()+,-./:=?";

    for c in data.as_bytes() {
        if !(c.is_ascii_lowercase() || c.is_ascii_uppercase() || c.is_ascii_digit() || ALLOWED_SPECIAL.contains(c)) {
            return false;
        }
    }

    true
}

pub fn validate_ia5(data: &str) -> bool {
    for c in data.chars() {
        if !c.is_ascii() {
            return false;
        }
    }
    true
}

pub fn validate_numeric(data: &str) -> bool {
    for c in data.chars() {
        if !c.is_ascii_digit() && c != ' ' {
            return false;
        }
    }
    true
}

pub fn validate_visible(data: &str) -> bool {
    for c in data.chars() {
        if c.is_ascii_control() || !c.is_ascii() {
            return false;
        }
    }
    true
}
