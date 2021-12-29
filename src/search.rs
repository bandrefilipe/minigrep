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
    use super::*;

    const TEST_CONTENTS: &str = "\
    I'm nobody! Who are you?\n\
    Are you nobody, too?\n\
    Then there's a pair of us - don't tell!\n\
    They'd banish us, you know.\n\
    \n\
    How dreary to be somebody!\n\
    How public, like a frog\n\
    To tell your name the livelong day\n\
    To an admiring bog!\
    ";

    #[test]
    fn search_finds_one_line() {
        let query = "frog";
        assert_eq!(
            search(query, TEST_CONTENTS),
            vec!["How public, like a frog"]
        );
    }

    #[test]
    fn search_finds_multiple_lines() {
        let query = "body";
        assert_eq!(
            search(query, TEST_CONTENTS),
            vec![
                "I'm nobody! Who are you?",
                "Are you nobody, too?",
                "How dreary to be somebody!",
            ]
        );
    }

    #[test]
    fn search_finds_nothing() {
        let query = "monomorphization";
        assert_eq!(search(query, TEST_CONTENTS).len(), 0);
    }
}
