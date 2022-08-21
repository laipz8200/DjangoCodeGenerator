use regex::Regex;

const INVALID_FIELD: &str = "invalid field name";
const INVALID_NAME: &str = "invalid name";

/// Returns field name and type.
pub fn parse_field(field: &str) -> Result<(&str, &str), String> {
    let v: Vec<&str> = field.split(":").collect();
    if v.len() == 2 {
        Ok((v[0], v[1]))
    } else {
        Err(format!("{}: {}", INVALID_FIELD, field))
    }
}

/// Convert name to UpperCamelCase.
pub fn convert_name(name: &str) -> Result<String, String> {
    let mut new_name = String::new();
    let mut flag = true;
    let re = Regex::new(r"^[a-zA-Z_][a-zA-Z_0-9]*$").unwrap();
    if !re.is_match(name) {
        return Err(format!("{}: {}", INVALID_NAME, name));
    }
    for c in name.chars() {
        match c {
            '_' => flag = true,
            c => {
                if flag {
                    new_name.push_str(c.to_uppercase().to_string().as_str());
                    flag = false;
                } else {
                    new_name.push(c)
                }
            }
        }
    }
    Ok(new_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_name_failed() {
        let cases = vec!["0a", "shd$"];

        for name in cases {
            assert_eq!(
                Err(format!("{}: {}", INVALID_NAME, name)),
                convert_name(name)
            );
        }
    }

    #[test]
    fn test_convert_name() {
        struct TestCase<'a> {
            name: &'a str,
            want: &'a str,
        }

        let cases = vec![
            TestCase {
                name: "User",
                want: "User",
            },
            TestCase {
                name: "user",
                want: "User",
            },
            TestCase {
                name: "userName",
                want: "UserName",
            },
            TestCase {
                name: "nCoV2019",
                want: "NCoV2019",
            },
            TestCase {
                name: "__user",
                want: "User",
            },
            TestCase {
                name: "user_group",
                want: "UserGroup",
            },
        ];

        for tt in cases {
            assert_eq!(Ok(String::from(tt.want)), convert_name(tt.name));
        }
    }

    #[test]
    fn test_parse_field_failed() {
        let cases = vec!["id", "id:string:other"];

        for field in cases {
            assert_eq!(
                Err(format!("{}: {}", INVALID_FIELD, field)),
                parse_field(field)
            );
        }
    }

    #[test]
    fn test_parse_field() {
        struct TestCase<'a> {
            field: &'a str,
            want: (&'a str, &'a str),
        }

        let cases = vec![
            TestCase {
                field: "name:string",
                want: ("name", "string"),
            },
            TestCase {
                field: "id:int",
                want: ("id", "int"),
            },
        ];

        for tt in cases {
            let got = parse_field(tt.field);
            assert_eq!(Ok(tt.want), got);
        }
    }
}
