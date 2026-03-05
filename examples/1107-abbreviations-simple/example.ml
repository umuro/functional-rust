(* Abbreviations, simple *)
(* Rosetta Code Abbreviations, simple implementation in OCaml *)

let table_as_string =
  "add 1 alter 3  backup 2  bottom 1  Cappend 2  change 1 \
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
   transfer 3  type 1  up 1"

(* Parse the table into (lowercase_name, min_length) pairs.
   When the next token is an integer it is the minimum; otherwise min = name length. *)
let parse_commands table =
  let tokens =
    String.split_on_char ' ' table
    |> List.concat_map (String.split_on_char '\n')
    |> List.filter (fun s -> s <> "")
  in
  let rec loop acc = function
    | [] -> List.rev acc
    | w :: rest ->
      let name = String.lowercase_ascii w in
      (match rest with
       | num :: tl when Option.is_some (int_of_string_opt num) ->
         loop ((name, int_of_string num) :: acc) tl
       | _ ->
         loop ((name, String.length name) :: acc) rest)
  in
  loop [] tokens

(* Look up one word in the parsed command table.
   Returns the matched command in uppercase, or "*error*". *)
let abbreviate word commands =
  let lower = String.lowercase_ascii word in
  let n = String.length lower in
  let starts_with s prefix =
    let lp = String.length prefix in
    String.length s >= lp && String.sub s 0 lp = prefix
  in
  match List.find_opt (fun (cmd, min_len) -> n >= min_len && starts_with cmd lower) commands with
  | Some (cmd, _) -> String.uppercase_ascii cmd
  | None -> "*error*"

let () =
  let commands = parse_commands table_as_string in
  let user = "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin" in
  let words =
    String.split_on_char ' ' user
    |> List.filter (fun s -> s <> "")
  in
  let results = List.map (fun w -> abbreviate w commands) words in
  print_endline (String.concat " " results);
  (* Expected: RIGHT REPEAT *error* PUT MOVE RESTORE *error* *error* *error* POWERINPUT *)
  assert (abbreviate "a" commands = "ADD");
  assert (abbreviate "al" commands = "*error*");
  assert (abbreviate "alt" commands = "ALTER");
  assert (abbreviate "get" commands = "GET");
  assert (abbreviate "ge" commands = "*error*");
  assert (abbreviate "ri" commands = "RIGHT");
  assert (abbreviate "fup." commands = "*error*");
  print_endline "ok"
