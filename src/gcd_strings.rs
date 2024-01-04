pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if str1.clone() + &str2.clone() != str2.clone() + &str1.clone() {
        return "".to_string();
    }
    "".to_string()
}
