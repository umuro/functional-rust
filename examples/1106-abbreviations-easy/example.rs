/// Extracts the minimum abbreviation from a command definition.
///
/// In the command table, uppercase letters mark the minimum required prefix.
/// For example, "ALTer" → minimum abbreviation is "ALT".
pub fn get_abbr(s: &str) -> String {
    s.chars().filter(|c| c.is_uppercase()).collect()
}

/// Looks up a user-supplied word in the command table.
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

fn main() {
    let cmds = "\
      Add ALTer  BAckup Bottom  CAppend Change SCHANGE  CInsert CLAst COMPress COpy
      COUnt COVerlay CURsor DELete CDelete Down DUPlicate Xedit EXPand EXTract Find
      NFind NFINDUp NFUp CFind FINdup FUp FOrward GET Help HEXType Input POWerinput
      Join SPlit SPLTJOIN  LOAD  Locate CLocate  LOWercase UPPercase  LPrefix MACRO
      MErge MODify MOve MSG Next Overlay PARSE PREServe PURge PUT PUTD  Query  QUIT
      READ  RECover REFRESH RENum REPeat  Replace CReplace  RESet  RESTore  RGTLEFT
      RIght LEft  SAVE  SET SHift SI  SORT  SOS  STAck STATus  TOP TRAnsfer Type Up";

    let user = "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin";

    let results = resolve_all(user, cmds);
    println!("{}", results.join(" "));

    // Show how abbreviation extraction works
    println!("\nAbbreviation extraction examples:");
    for cmd in ["ALTer", "COMPress", "POWerinput", "GET", "Add"] {
        println!("  {} → min abbr: {:?}", cmd, get_abbr(cmd));
    }
}

/* Output:
   RIGHT REPEAT *error* PUT MOVE RESTORE *error* *error* *error* POWERINPUT

   Abbreviation extraction examples:
     ALTer → min abbr: "ALT"
     COMPress → min abbr: "COMP"
     POWerinput → min abbr: "POW"
     GET → min abbr: "GET"
     Add → min abbr: "A"
*/
