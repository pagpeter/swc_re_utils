// Parse strings like the JavaScript parseInt() function
pub fn parse_int(s: &str) -> Option<f64> {
    let s = s.trim_start();
    if s.is_empty() {
        return None;
    }
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        let digits: String = hex.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
        if digits.is_empty() {
            return None;
        }
        return Some(
            i64::from_str_radix(&digits, 16)
                .map(|v| v as f64)
                .unwrap_or(f64::NAN),
        );
    }

    let mut chars = s.chars().peekable();
    let mut num = String::new();
    if let Some(&c) = chars.peek() {
        if c == '-' || c == '+' {
            num.push(c);
            chars.next();
        }
    }
    num.extend(chars.take_while(|c| c.is_ascii_digit()));
    if num.len() == 0 || num == "+" || num == "-" {
        return None;
    }
    Some(num.parse::<f64>().unwrap_or(f64::NAN))
}
