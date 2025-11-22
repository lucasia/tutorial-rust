pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lowercase = &query.to_lowercase();

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query_lowercase) {
            results.push(line);
        }
    }
    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
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

        assert_eq!(vec!["safe, fast, productive."], search(query, CONTENTS));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, CONTENTS));
    }
}