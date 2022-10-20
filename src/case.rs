#[derive(
    Debug, Clone, clap::ValueEnum, PartialEq, strum_macros::EnumIter, strum_macros::Display,
)]
pub enum Case {
    Snake,
    ScreamingSnake,
    Kebab,
    Path,
    Dot,
    Camel,
    Pascal,
}

impl Case {
    pub fn guess(string: &str) -> Self {
        if string.chars().all(|ch| ch.is_uppercase()) {
            Case::ScreamingSnake
        } else if string.contains("_") {
            if string
                .chars()
                .filter(|ch| ch != &'_')
                .all(|ch| ch.is_lowercase())
            {
                Case::Snake
            } else {
                Case::ScreamingSnake
            }
        } else if string.contains("-") {
            Case::Kebab
        } else if string.contains("/") {
            Case::Path
        } else if string.contains(".") {
            Case::Dot
        } else {
            match string.chars().nth(0) {
                Some(ch) if ch.is_uppercase() => Case::Pascal,
                _ => Case::Camel,
            }
        }
    }

    pub fn tokenize(&self, string: &str) -> Vec<String> {
        match self {
            Case::Snake | Case::ScreamingSnake => string
                .split("_")
                .map(|token| token.to_lowercase())
                .collect(),

            Case::Kebab => string
                .split("-")
                .map(|token| token.to_lowercase())
                .collect(),

            Case::Path => string
                .split("/")
                .map(|token| token.to_lowercase())
                .collect(),

            Case::Dot => string
                .split(".")
                .map(|token| token.to_lowercase())
                .collect(),

            Case::Pascal => {
                let mut tokens: Vec<String> = Vec::new();

                for ch in string.chars() {
                    if ch.is_uppercase() {
                        tokens.push(ch.to_string())
                    } else {
                        match tokens.last_mut() {
                            Some(last) => last.push(ch),
                            None => tokens.push(ch.to_string()),
                        }
                    }
                }

                tokens.iter().map(|token| token.to_lowercase()).collect()
            }

            Case::Camel => {
                let mut tokens: Vec<String> = Vec::new();

                let mut chars = string.chars();
                if let Some(ch) = chars.next() {
                    tokens.push(ch.to_string())
                }

                for ch in chars {
                    if ch.is_uppercase() {
                        tokens.push(ch.to_string());
                    } else {
                        match tokens.last_mut() {
                            Some(last) => last.push(ch),
                            None => tokens.push(ch.to_string()),
                        }
                    }
                }

                tokens.iter().map(|token| token.to_lowercase()).collect()
            }
        }
    }

    pub fn join(&self, tokens: Vec<String>) -> String {
        match self {
            Case::Snake => tokens.join("_").to_lowercase(),
            Case::ScreamingSnake => tokens.join("_").to_uppercase(),
            Case::Kebab => tokens.join("-").to_lowercase(),
            Case::Path => tokens.join("/").to_lowercase(),
            Case::Dot => tokens.join(".").to_lowercase(),

            Case::Camel => {
                let mut tokens = tokens.into_iter();
                match tokens.next() {
                    None => String::new(),
                    Some(token) => {
                        token.to_lowercase()
                            + tokens
                                .map(|token| capitalize(&token))
                                .collect::<Vec<String>>()
                                .join("")
                                .as_str()
                    }
                }
            }

            Case::Pascal => tokens
                .iter()
                .map(|token| capitalize(token))
                .collect::<Vec<String>>()
                .join(""),
        }
    }
}

fn capitalize(string: &str) -> String {
    let mut chars = string.chars();
    match chars.next() {
        None => String::new(),
        Some(ch) => ch.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str(),
    }
}

#[test]
fn capitalize_test() {
    assert_eq!(capitalize("something"), "Something");
    assert_eq!(capitalize("Something"), "Something");
    assert_eq!(capitalize("SomeThing"), "Something");
    assert_eq!(capitalize("someThing"), "Something");
}

#[cfg(test)]
mod test {
    use super::Case;
    use strum::IntoEnumIterator;

    enum Test {
        Guess(&'static str),
        Tokenize(&'static str, Vec<&'static str>),
        Join(Vec<&'static str>, &'static str),
    }

    #[test]
    fn table() {
        for case in Case::iter() {
            for test in tests(&case) {
                match test {
                    Test::Guess(guess) => assert_eq!(Case::guess(guess), case),
                    Test::Tokenize(string, tokens) => assert_eq!(case.tokenize(string), tokens),
                    Test::Join(tokens, string) => assert_eq!(
                        case.join(tokens.iter().map(|token| token.to_string()).collect()),
                        string
                    ),
                }
            }
        }
    }

    fn tests(case: &Case) -> Vec<Test> {
        match case {
            Case::Snake => vec![
                Test::Guess("snake_case"),
                Test::Tokenize("snake_case", vec!["snake", "case"]),
                Test::Join(vec!["snake", "case"], "snake_case"),
            ],
            Case::ScreamingSnake => vec![
                Test::Guess("SCREAMING"),
                Test::Guess("SCREAMING_SNAKE_CASE"),
                Test::Tokenize("SCREAMING_SNAKE_CASE", vec!["screaming", "snake", "case"]),
                Test::Join(vec!["screaming", "snake", "case"], "SCREAMING_SNAKE_CASE"),
            ],
            Case::Kebab => vec![
                Test::Guess("kebab-case"),
                Test::Tokenize("kebab-case", vec!["kebab", "case"]),
                Test::Join(vec!["kebab", "case"], "kebab-case"),
            ],
            Case::Path => vec![
                Test::Guess("path/case"),
                Test::Tokenize("path/case", vec!["path", "case"]),
                Test::Join(vec!["path", "case"], "path/case"),
            ],
            Case::Dot => vec![
                Test::Guess("dot.case"),
                Test::Tokenize("dot.case", vec!["dot", "case"]),
                Test::Join(vec!["dot", "case"], "dot.case"),
            ],
            Case::Camel => vec![
                Test::Guess("camelCase"),
                Test::Tokenize("camelCase", vec!["camel", "case"]),
                Test::Join(vec!["camel", "case"], "camelCase"),
            ],
            Case::Pascal => vec![
                Test::Guess("PascalCase"),
                Test::Tokenize("PascalCase", vec!["pascal", "case"]),
                Test::Join(vec!["pascal", "case"], "PascalCase"),
            ],
        }
    }
}
