/// An implementation to match on simple strings.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Matcher {
    /// Considers the entire string (trimmed) to be the match.
    AllTrimmed,

    /// After finding the `prefix` followed by one or more spaces, returns the following word.
    PrefixedWord { prefix: &'static str },

    /// Similar to `PrefixedWord`, but only if the word is a valid version.
    PrefixedVersion { prefix: &'static str },

    /// Takes a set of lines (separated by `\n`) and searches for the value in a key/value pair
    /// separated by the `=` character. For example `VERSION_ID="8.1"`.
    KeyValue { key: &'static str },
    /// Takes a string and returns the substring between two characters. For example, `"22.04.1 LTS (Jammy Jellyfish)"`
    /// would return `Jammy Jellyfish` if the start character is `(` and the end character is `)`.
    /// The start and end characters are inclusive.
    Between { start: char, end: char },
}

impl Matcher {
    /// Find the match on the input `string` according to the matcher variant.
    ///
    /// # Arguments
    ///
    /// * `string` - The input string to search.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the matched value, or `None` if no match is found.
    ///
    /// # Examples
    ///
    /// ```
    /// use osinfo::Matcher;
    /// let matcher = Matcher::AllTrimmed;
    /// assert_eq!(matcher.find("  hello "), Some("hello".to_string()));
    /// ```
    pub fn find(&self, string: &str) -> Option<String> {
        match *self {
            Self::AllTrimmed => Some(string.trim().to_string()),
            Self::PrefixedWord { prefix } => find_prefixed_word(string, prefix).map(str::to_owned),
            Self::PrefixedVersion { prefix } => find_prefixed_word(string, prefix)
                .filter(|&v| is_valid_version(v))
                .map(str::to_owned),
            Self::KeyValue { key } => find_by_key(string, key).map(str::to_owned),
            Self::Between { start, end } => slice_string(string, start, end),
        }
    }
}

/// Returns the substring between the first occurrence of `start_char` and the next occurrence of `end_char`.
///
/// # Arguments
///
/// * `input` - The input string.
/// * `start_char` - The character marking the start of the substring.
/// * `end_char` - The character marking the end of the substring.
///
/// # Returns
///
/// An `Option<String>` containing the substring, or `None` if not found.
///
/// # Example
/// 
/// ```
/// use osinfo::Matcher;
/// let matcher = Matcher::Between { start: '(', end: ')' };
/// assert_eq!(matcher.find("Ubuntu (Jammy Jellyfish)"), Some("Jammy Jellyfish".to_string()));
/// ```
fn slice_string(input: &str, start_char: char, end_char: char) -> Option<String> {
    if let Some(start_idx) = input.find(start_char) {
        if let Some(end_idx) = input[start_idx + 1..].find(end_char) {
            return Some(&input[start_idx+1..start_idx+end_idx+1]).and_then(|s| Some(s.to_string()));
        }
        return Some(&input[start_idx..]).and_then(|s| Some(s.to_string()));
    }
    None
}

/// Finds the value for a given key in a key-value formatted string (lines separated by `\n`).
///
/// # Arguments
///
/// * `string` - The input string.
/// * `key` - The key to search for.
///
/// # Returns
///
/// An `Option<&str>` containing the value, or `None` if not found.
///
/// # Example
///
/// ```
/// use osinfo::Matcher;
/// let matcher = Matcher::KeyValue { key: "VERSION_ID" };
/// assert_eq!(matcher.find("VERSION_ID=\"8.1\""), Some("8.1".to_string()));
/// ```
fn find_by_key<'a>(string: &'a str, key: &str) -> Option<&'a str> {
    let key = [key, "="].concat();
    for line in string.lines() {
        if line.starts_with(&key) {
            return Some(line[key.len()..].trim_matches(|c: char| c == '"' || c.is_whitespace()));
        }
    }

    None
}

/// Finds the word immediately following a given prefix in the input string.
///
/// # Arguments
///
/// * `string` - The input string.
/// * `prefix` - The prefix to search for.
///
/// # Returns
///
/// An `Option<&str>` containing the word, or `None` if not found.
///
/// # Example
///
/// ```
/// use osinfo::Matcher;
/// let matcher = Matcher::PrefixedWord { prefix: "test" };
/// assert_eq!(matcher.find("test 1.2.3"), Some("1.2.3".to_string()));
/// ```
fn find_prefixed_word<'a>(string: &'a str, prefix: &str) -> Option<&'a str> {
    if let Some(prefix_start) = string.find(prefix) {
        // Ignore prefix and leading whitespace
        let string = &string[prefix_start + prefix.len()..].trim_start();

        // Find where the word boundary ends
        let word_end = string
            .find(|c: char| c.is_whitespace())
            .unwrap_or(string.len());
        let string = &string[..word_end];

        Some(string)
    } else {
        None
    }
}

/// Checks if a word is a valid version (does not start or end with a dot).
///
/// # Arguments
///
/// * `word` - The word to check.
///
/// # Returns
///
/// `true` if the word is a valid version, `false` otherwise.
///
/// # Example
///
/// ```
/// use osinfo::Matcher;
/// let matcher = Matcher::PrefixedVersion { prefix: "test" };
/// assert_eq!(matcher.find("test 1.2.3"), Some("1.2.3".to_string()));
/// assert_eq!(matcher.find("test .1.2.3"), None);
/// ```
fn is_valid_version(word: &str) -> bool {
    !word.starts_with('.') && !word.ends_with('.')
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn trimmed() {
        let data = [
            ("", Some("")),
            ("test", Some("test")),
            (" 		 test", Some("test")),
            ("test  	   ", Some("test")),
            ("  test 	", Some("test")),
        ];

        let matcher = Matcher::AllTrimmed;

        for (input, expected) in &data {
            let result = matcher.find(input);
            assert_eq!(result.as_deref(), *expected);
        }
    }

    #[test]
    fn prefixed_word() {
        let data = [
            ("", None),
            ("test", Some("")),
            ("test1", Some("1")),
            ("test 1", Some("1")),
            (" test 1", Some("1")),
            ("test 1.2.3", Some("1.2.3")),
            (" 		test 1.2.3", Some("1.2.3")),
        ];

        let matcher = Matcher::PrefixedWord { prefix: "test" };

        for (input, expected) in &data {
            let result = matcher.find(input);
            assert_eq!(result.as_deref(), *expected);
        }
    }

    #[test]
    fn prefixed_version() {
        let data = [
            ("", None),
            ("test", Some("")),
            ("test 1", Some("1")),
            ("test .1", None),
            ("test 1.", None),
            ("test .1.", None),
            (" test 1", Some("1")),
            ("test 1.2.3", Some("1.2.3")),
            (" 		test 1.2.3", Some("1.2.3")),
        ];

        let matcher = Matcher::PrefixedVersion { prefix: "test" };

        for (input, expected) in &data {
            let result = matcher.find(input);
            assert_eq!(result.as_deref(), *expected);
        }
    }

    #[test]
    fn key_value() {
        let data = [
            ("", None),
            ("key", None),
            ("key=value", Some("value")),
            ("key=1", Some("1")),
            ("key=\"1\"", Some("1")),
            ("key=\"CentOS Linux\"", Some("CentOS Linux")),
        ];

        let matcher = Matcher::KeyValue { key: "key" };

        for (input, expected) in &data {
            let result = matcher.find(input);
            assert_eq!(result.as_deref(), *expected);
        }
    }

    #[test]
    fn between() {
        let matcher = Matcher::Between { start: '(', end: ')' };
        assert_eq!(
            matcher.find("Ubuntu 22.04.1 LTS (Jammy Jellyfish)"),
            Some("Jammy Jellyfish".to_string())
        );
        assert_eq!(
            matcher.find("No parentheses here"),
            None
        );
        assert_eq!(
            matcher.find("Start only (no end"),
            Some("(no end".to_string())
        );
        assert_eq!(
            matcher.find("Nothing"),
            None
        );
    }
}
