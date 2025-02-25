pub fn safe_truncate(string: &mut String, i: usize) {
    if string.len() <= i {
        return
    } else if string.is_char_boundary(i) {
        string.truncate(i)
    } else {
        safe_truncate(string, i - 1)
    }
}

pub fn safe_split_off(string: &mut String, i: usize) -> String {
    if string.len() <= i {
        String::new()
    } else if string.is_char_boundary(i) {
        string.split_off(i)
    } else {
        safe_split_off(string, i - 1)
    }
}
