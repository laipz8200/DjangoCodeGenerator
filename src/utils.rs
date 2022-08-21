/// Returns field name and type.
pub fn parse_field(field: &str) -> Result<(&str, &str), String> {
    let v: Vec<&str> = field.split(":").collect();
    if v.len() == 2 {
        Ok((v[0], v[1]))
    } else {
        Err(format!("{}: {}", ERROR_MESSAGE, field))
    }
}

const ERROR_MESSAGE: &str = "Field format does not match";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_field() {
        #[derive(PartialEq)]
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
