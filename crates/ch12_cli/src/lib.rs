pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query.to_lowercase()))
}

pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents.lines().filter(move |line| line.contains(query))
}

#[cfg(test)]
mod tests {
    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.";

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, CONTENTS).collect::<Vec<_>>()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, CONTENTS).collect::<Vec<_>>()
        );
    }
}
