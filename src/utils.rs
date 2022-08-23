use regex::Regex;

pub fn upper_camel_case(word: &str) -> String {
    let re = Regex::new(r"^[a-zA-Z_][a-zA-Z_0-9]*$").unwrap();
    if !re.is_match(word) {
        panic!("invalid word: {}", word);
    }
    let mut res = String::new();
    let mut flag = true;
    for c in word.chars() {
        match c {
            '_' => flag = true,
            c => {
                if flag {
                    res.push_str(c.to_uppercase().to_string().as_str());
                    flag = false;
                } else {
                    res.push(c)
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper_camel_case() {
        let tests = vec![
            ("name", "Name"),
            ("Person", "Person"),
            ("task_manager", "TaskManager"),
            ("mapDomain", "MapDomain"),
            ("__user", "User"),
            ("user__", "User"),
        ];
        for (params, want) in tests {
            assert_eq!(want, upper_camel_case(params));
        }
    }
}
