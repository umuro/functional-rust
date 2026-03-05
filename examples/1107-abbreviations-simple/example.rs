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

const USER_INPUT: &str =
    "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin";

/// Parses a command table where each entry is `word [min_length]`.
///
/// Returns `(lowercase_command, min_length)` pairs.
/// Commands without an explicit number require an exact match.
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
pub fn resolve_line(line: &str, commands: &[(String, usize)]) -> Vec<String> {
    line.split_whitespace()
        .map(|word| abbreviate(word, commands))
        .collect()
}

fn main() {
    let commands = parse_commands(TABLE);
    println!("Input:  {USER_INPUT}");
    let results = resolve_line(USER_INPUT, &commands);
    println!("Output: {}", results.join(" "));

    // Individual lookups
    println!();
    println!("a       → {}", abbreviate("a", &commands));
    println!("add     → {}", abbreviate("add", &commands));
    println!("alt     → {}", abbreviate("alt", &commands));
    println!("al      → {}", abbreviate("al", &commands));
    println!("get     → {}", abbreviate("get", &commands));
    println!("ge      → {}", abbreviate("ge", &commands));
    println!("ri      → {}", abbreviate("ri", &commands));
    println!("fup.    → {}", abbreviate("fup.", &commands));
}

/* Output:
   Input:  riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin
   Output: RIGHT REPEAT *error* PUT MOVE RESTORE *error* *error* *error* POWERINPUT

   a       → ADD
   add     → ADD
   alt     → ALTER
   al      → *error*
   get     → GET
   ge      → *error*
   ri      → RIGHT
   fup.    → *error*
*/
