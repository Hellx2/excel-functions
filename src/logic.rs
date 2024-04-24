pub fn and<T: Into<bool>>(array: Vec<T>) -> bool {
    for i in array {
        if !i.into() { return false }
    }
    true
}

pub fn bycol() {}
