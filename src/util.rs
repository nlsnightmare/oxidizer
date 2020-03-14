pub fn resource_route(url: &str) -> String {
    let mut regex = regex::Regex::new(r"ies$")
        .unwrap();
    
    let new_string = regex.replace(url, "y")
        .to_string();

    regex = regex::Regex::new(r"s$")
        .unwrap();

    regex.replace(&new_string, "").to_string()
}