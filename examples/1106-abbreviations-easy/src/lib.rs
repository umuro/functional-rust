/// Extracts the minimum abbreviation from a command definition.
///
/// In the command table, uppercase letters mark the minimum required prefix.
/// For example, "ALTer" → minimum abbreviation is "ALT".
/// A command with no uppercase letters (e.g. "GET") has empty abbr → always matches at full length.
pub fn get_abbr(s: &str) -> String {
    s.chars().filter(|c| c.is_uppercase()).collect()
}

/// Looks up a user-supplied word in the command table.
///
/// Each command table entry is `(min_abbr, full_uppercase_name)`.
/// A word matches a command when:
///   - word length >= min_abbr length  (meets the minimum)
///   - word length <= full command length  (not longer than the command)
///   - the first `word.len()` chars of the full command equal `word.to_uppercase()`
pub fn lookup<'a>(word: &str, commands: &[(&str, &'a str)]) -> &'a str {
    let n = word.len();
    let word_upper = word.to_uppercase();
    commands
        .iter()
        .find(|(abbr, full)| {
            let na = abbr.len();
            let nc = full.len();
            n >= na && n <= nc && full[..n] == word_upper
        })
        .map(|(_, full)| *full)
        .unwrap_or("*error*")
}

/// Builds the command table from the raw command string.
///
/// Each token is a mixed-case command name where uppercase letters indicate the
/// minimum abbreviation. Returns pairs of `(min_abbr, full_uppercase_name)`.
pub fn build_table(raw: &str) -> Vec<(String, String)> {
    raw.split_whitespace()
        .map(|s| (get_abbr(s), s.to_uppercase()))
        .collect()
}

/// Resolves a list of user words against the command table.
pub fn resolve_all(user_input: &str, raw_commands: &str) -> Vec<String> {
    let table = build_table(raw_commands);
    let table_refs: Vec<(&str, &str)> = table
        .iter()
        .map(|(a, c)| (a.as_str(), c.as_str()))
        .collect();
    user_input
        .split_whitespace()
        .map(|word| lookup(word, &table_refs).to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CMDS: &str = "\
      Add ALTer  BAckup Bottom  CAppend Change SCHANGE  CInsert CLAst COMPress COpy
      COUnt COVerlay CURsor DELete CDelete Down DUPlicate Xedit EXPand EXTract Find
      NFind NFINDUp NFUp CFind FINdup FUp FOrward GET Help HEXType Input POWerinput
      Join SPlit SPLTJOIN  LOAD  Locate CLocate  LOWercase UPPercase  LPrefix MACRO
      MErge MODify MOve MSG Next Overlay PARSE PREServe PURge PUT PUTD  Query  QUIT
      READ  RECover REFRESH RENum REPeat  Replace CReplace  RESet  RESTore  RGTLEFT
      RIght LEft  SAVE  SET SHift SI  SORT  SOS  STAck STATus  TOP TRAnsfer Type Up";

    #[test]
    fn test_get_abbr_mixed_case() {
        assert_eq!(get_abbr("ALTer"), "ALT");
        assert_eq!(get_abbr("COMPress"), "COMP");
        assert_eq!(get_abbr("POWerinput"), "POW");
    }

    #[test]
    fn test_get_abbr_all_uppercase_and_mixed() {
        assert_eq!(get_abbr("GET"), "GET");
        assert_eq!(get_abbr("PUT"), "PUT");
        assert_eq!(get_abbr("Add"), "A");
    }

    #[test]
    fn test_lookup_valid_abbreviations() {
        let table = build_table(CMDS);
        let refs: Vec<(&str, &str)> = table
            .iter()
            .map(|(a, c)| (a.as_str(), c.as_str()))
            .collect();

        assert_eq!(lookup("riG", &refs), "RIGHT");
        assert_eq!(lookup("rePEAT", &refs), "REPEAT");
        assert_eq!(lookup("fup", &refs), "FUP");
        assert_eq!(lookup("poweRin", &refs), "POWERINPUT");
        assert_eq!(lookup("put", &refs), "PUT");
        assert_eq!(lookup("mo", &refs), "MOVE");
        assert_eq!(lookup("rest", &refs), "RESTORE");
    }

    #[test]
    fn test_lookup_error_on_unknown() {
        let table = build_table(CMDS);
        let refs: Vec<(&str, &str)> = table
            .iter()
            .map(|(a, c)| (a.as_str(), c.as_str()))
            .collect();

        assert_eq!(lookup("6", &refs), "*error*");
        assert_eq!(lookup("fup.", &refs), "*error*");
        assert_eq!(lookup("copies", &refs), "*error*");
        assert_eq!(lookup("types", &refs), "*error*");
    }

    #[test]
    fn test_resolve_all_rosetta_example() {
        let user = "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin";
        let result = resolve_all(user, CMDS);
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

    #[test]
    fn test_build_table_structure() {
        let table = build_table(CMDS);
        assert_eq!(table.len(), 81);
        assert_eq!(table[0], ("A".to_string(), "ADD".to_string()));
        assert_eq!(table[1], ("ALT".to_string(), "ALTER".to_string()));
    }
}
