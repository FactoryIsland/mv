pub fn parse_char(s: &str, err: fn(String)) -> char {
    if s.len() == 1 {
        s.chars().next().unwrap()
    }
    else if s.len() == 2 && s.starts_with('\\') {
        let c = s.chars().nth(1).unwrap();
        match c {
            's' => ' ',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            'b' => '\x08',
            'f' => '\x0c',
            'v' => '\x0b',
            _ => c
        }
    }
    else if s.starts_with("\\x") || s.starts_with("\\u") {
        let c = u32::from_str_radix(&s[2..], 16).unwrap();
        let c = char::from_u32(c);
        if c.is_none() {
            err(format!("Invalid hex character: {}", s));
        }
        c.unwrap()
    }
    else {
        err(format!("Invalid string literal for character: {}", s));
        '\0'
    }
}