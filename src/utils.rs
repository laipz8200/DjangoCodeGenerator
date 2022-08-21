/// Returns field name and type.
pub fn parse_field(field: &str) -> Result<(&str, &str), String> {
    let v: Vec<&str> = field.split(":").collect();
    if v.len() == 2 {
        Ok((v[0], v[1]))
    } else {
        Err(format!("{}: {}", ERROR_MESSAGE, field))
    }
}

/// Convert name to UpperCamelCase.
pub fn convert_name(name: &str) -> String {
    let mut new_name = String::new();
    let mut flag = true;
    for c in name.chars() {
        if flag {
            new_name.push_str(c.to_uppercase().to_string().as_str());
            flag = false;
            continue;
        }
        match c {
            '-' => flag = true,
            '_' => flag = true,
            c => new_name.push(c),
        }
    }
    new_name
}

const ERROR_MESSAGE: &str = "Field format does not match";

#[cfg(test)]
mod tests {
    use super::*;

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
                name: "user-group",
                want: "UserGroup",
            },
            TestCase {
                name: "user_group",
                want: "UserGroup",
            },
        ];

        for tt in cases {
            assert_eq!(tt.want, convert_name(tt.name).as_str());
        }
    }

    #[test]
    fn test_parse_field() {
        struct TestCase<'a> {
            field: &'a str,
            want: (&'a str, &'a str),
            error_message: String,
        }

        let cases = vec![
            TestCase {
                field: "name:string",
                want: ("name", "string"),
                error_message: String::new(),
            },
            TestCase {
                field: "id:int",
                want: ("id", "int"),
                error_message: String::new(),
            },
            TestCase {
                field: "id",
                want: ("", ""),
                error_message: format!("{}: {}", ERROR_MESSAGE, "id"),
            },
            TestCase {
                field: "id:string:other",
                want: ("", ""),
                error_message: format!("{}: {}", ERROR_MESSAGE, "id:string:other"),
            },
        ];

        for tt in cases {
            let got = parse_field(tt.field);
            match got {
                Ok(got) => assert_eq!(tt.want, got),
                Err(e) => assert_eq!(tt.error_message, e),
            }
        }
    }
}
