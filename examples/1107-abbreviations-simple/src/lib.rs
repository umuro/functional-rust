/// Parses a command table where each entry is `word [min_length]`.
///
/// Each token is either a command name optionally followed by a number:
/// - `"add 1"` → command "add", min abbreviation length 1
/// - `"get"` → command "get", exact match required (min = word length)
///
/// Returns a `Vec<(String, usize)>` of `(lowercase_command, min_length)`.
pub fn parse_commands(table: &str) -> Vec<(String, usize)> {
    let tokens: Vec<&str> = table.split_whitespace().collect();
    let mut commands = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let name = tokens[i].to_lowercase();
        let (min_len, advance_by) = if i + 1 < tokens.len() {
            match tokens[i + 1].parse::<usize>() {
                Ok(n) => (n, 2),
                Err(_) => (name.len(), 1),
            }
        } else {
            (name.len(), 1)
        };
        commands.push((name, min_len));
        i += advance_by;
    }
    commands
}

/// Looks up a single word against a parsed command table.
///
/// A word matches a command when:
/// - `word.len() >= min_length` (meets the minimum abbreviation length)
/// - the command name starts with `word` (case-insensitive prefix)
///
/// Returns the matched command in uppercase, or `"*error*"` if no match.
pub fn abbreviate(word: &str, commands: &[(String, usize)]) -> String {
    let lower = word.to_lowercase();
    let len = lower.len();
    commands
        .iter()
        .find(|(cmd, min)| len >= *min && cmd.starts_with(lower.as_str()))
        .map(|(cmd, _)| cmd.to_uppercase())
        .unwrap_or_else(|| "*error*".to_string())
}

/// Resolves every whitespace-separated word in `line` against the command table.
///
/// Idiomatic Rust: splits into words, maps each through `abbreviate`, collects.
pub fn resolve_line(line: &str, commands: &[(String, usize)]) -> Vec<String> {
    line.split_whitespace()
        .map(|word| abbreviate(word, commands))
        .collect()
}

/// Convenience wrapper: parses the raw table string and resolves the input line.
pub fn resolve(line: &str, raw_table: &str) -> Vec<String> {
    let commands = parse_commands(raw_table);
    resolve_line(line, &commands)
}

/// Recursive variant: walks the command table via slice patterns, mirroring OCaml list recursion.
///
/// Matches the head entry first; on failure recurses into the tail — exactly the OCaml
/// `List.find_opt` over an association list.
pub fn abbreviate_recursive(word: &str, commands: &[(String, usize)]) -> String {
    let lower = word.to_lowercase();
    let len = lower.len();

    fn go(lower: &str, len: usize, table: &[(String, usize)]) -> Option<String> {
        match table {
            [] => None,
            [(cmd, min), rest @ ..] => {
                if len >= *min && cmd.starts_with(lower) {
                    Some(cmd.to_uppercase())
                } else {
                    go(lower, len, rest)
                }
            }
        }
    }

    go(&lower, len, commands).unwrap_or_else(|| "*error*".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rosetta Code "Abbreviations, simple" command table.
    // Format: `command [min_length]` — if no number, exact match required.
    const TABLE: &str = "add 1 alter 3  backup 2  bottom 1  Cappend 2  change 1 \
       Schange  Cinsert 2  Clast 3 compress 4 copy 2 count 3 \
       Coverlay 3 cursor 3  delete 3 Cdelete 2  down 1  duplicate 3 \
       xEdit 1 expand 3 extract 3  find 1 Nfind 2 Nfindup 6 NfUP 3 \
       Cfind 2 findUP 3 fUP 2 forward 2  get  help 1 hexType 4 \
       input 1 powerInput 3  join 1 split 2 spltJOIN load locate 1 \
       Clocate 2 lowerCase 3 upperCase 3 Lprefix 2  macro  merge 2 \
       modify 3 move 2 msg  next 1 overlay 1 parse preserve 4 purge 3 \
       put putD query 1 quit  read recover 3 refresh renum 3 repeat 3 \
       replace 1 Creplace 2 reset 3 restore 4 rgtLEFT right 2 left 2 \
       save  set  shift 2  si  sort  sos  stack 3 status 4 top \
       transfer 3  type 1  up 1";

    #[test]
    fn test_parse_commands_count() {
        let commands = parse_commands(TABLE);
        assert_eq!(commands.len(), 81);
    }

    #[test]
    fn test_abbreviate_with_explicit_min_length() {
        let commands = parse_commands(TABLE);
        // "add 1": any prefix >= 1 char matches
        assert_eq!(abbreviate("a", &commands), "ADD");
        assert_eq!(abbreviate("ad", &commands), "ADD");
        assert_eq!(abbreviate("add", &commands), "ADD");
        // "alter 3": need at least 3 chars
        assert_eq!(abbreviate("alt", &commands), "ALTER");
        assert_eq!(abbreviate("alter", &commands), "ALTER");
        assert_eq!(abbreviate("al", &commands), "*error*");
        // "right 2": need at least 2 chars
        assert_eq!(abbreviate("ri", &commands), "RIGHT");
        assert_eq!(abbreviate("rig", &commands), "RIGHT");
    }

    #[test]
    fn test_abbreviate_exact_match_only() {
        let commands = parse_commands(TABLE);
        // No number in table → exact match required
        assert_eq!(abbreviate("get", &commands), "GET");
        assert_eq!(abbreviate("ge", &commands), "*error*");
        assert_eq!(abbreviate("sort", &commands), "SORT");
        assert_eq!(abbreviate("sor", &commands), "*error*");
        assert_eq!(abbreviate("save", &commands), "SAVE");
        assert_eq!(abbreviate("sav", &commands), "*error*");
    }

    #[test]
    fn test_abbreviate_case_insensitive() {
        let commands = parse_commands(TABLE);
        assert_eq!(abbreviate("ADD", &commands), "ADD");
        assert_eq!(abbreviate("Ad", &commands), "ADD");
        assert_eq!(abbreviate("RIG", &commands), "RIGHT");
        assert_eq!(abbreviate("POWERINPUT", &commands), "POWERINPUT");
    }

    #[test]
    fn test_abbreviate_errors() {
        let commands = parse_commands(TABLE);
        // Unknown words
        assert_eq!(abbreviate("6", &commands), "*error*");
        assert_eq!(abbreviate("xyz", &commands), "*error*");
        // Word with trailing punctuation — no command starts with "fup."
        assert_eq!(abbreviate("fup.", &commands), "*error*");
        // "copies" is longer than "copy", so "copy" can't starts_with "copies"
        assert_eq!(abbreviate("copies", &commands), "*error*");
        // "types" is longer than "type"
        assert_eq!(abbreviate("types", &commands), "*error*");
    }

    #[test]
    fn test_abbreviate_recursive_matches_iterative() {
        let commands = parse_commands(TABLE);
        // recursive and iterative variants must agree on every result
        for word in [
            "riG", "rePEAT", "copies", "put", "mo", "rest", "types", "fup.", "6", "poweRin",
        ] {
            assert_eq!(
                abbreviate_recursive(word, &commands),
                abbreviate(word, &commands),
                "mismatch for word {:?}",
                word
            );
        }
    }

    #[test]
    fn test_resolve_rosetta_example() {
        let user = "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin";
        let result = resolve(user, TABLE);
        assert_eq!(
            result,
            vec![
                "RIGHT",
                "REPEAT",
                "*error*",
                "PUT",
                "MOVE",
                "RESTORE",
                "*error*",
                "*error*",
                "*error*",
                "POWERINPUT",
            ]
        );
    }
}
