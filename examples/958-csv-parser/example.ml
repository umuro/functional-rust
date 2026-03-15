(* 958: CSV Parser *)
(* Handle quoted fields, commas inside quotes, escaped quotes *)

(* Approach 1: Simple split (no quote handling) *)

let split_simple line =
  String.split_on_char ',' line

(* Approach 2: Proper CSV field parser using state machine *)

type state = Normal | InQuote | AfterQuote

let parse_csv_line line =
  let n = String.length line in
  let fields = ref [] in
  let current = Buffer.create 16 in
  let state = ref Normal in

  for i = 0 to n - 1 do
    let c = line.[i] in
    match !state, c with
    | Normal, '"' ->
      state := InQuote
    | Normal, ',' ->
      fields := Buffer.contents current :: !fields;
      Buffer.clear current
    | Normal, c ->
      Buffer.add_char current c
    | InQuote, '"' ->
      state := AfterQuote
    | InQuote, c ->
      Buffer.add_char current c
    | AfterQuote, '"' ->
      (* Escaped quote: "" inside quoted field *)
      Buffer.add_char current '"';
      state := InQuote
    | AfterQuote, ',' ->
      fields := Buffer.contents current :: !fields;
      Buffer.clear current;
      state := Normal
    | AfterQuote, _ ->
      state := Normal
  done;
  (* Add the last field *)
  fields := Buffer.contents current :: !fields;
  List.rev !fields

(* Approach 3: Parse multiple rows *)

let parse_csv text =
  let lines = String.split_on_char '\n' text in
  List.filter_map (fun line ->
    if String.length line = 0 then None
    else Some (parse_csv_line line)
  ) lines

let () =
  (* Simple split *)
  let row = split_simple "a,b,c" in
  assert (row = ["a"; "b"; "c"]);

  (* Quoted fields *)
  let row2 = parse_csv_line "\"hello\",\"world\",plain" in
  assert (row2 = ["hello"; "world"; "plain"]);

  (* Comma inside quotes *)
  let row3 = parse_csv_line "\"one, two\",three" in
  assert (row3 = ["one, two"; "three"]);

  (* Escaped quotes inside quoted field *)
  let row4 = parse_csv_line "\"say \"\"hi\"\"\",end" in
  assert (row4 = ["say \"hi\""; "end"]);

  (* Empty fields *)
  let row5 = parse_csv_line ",," in
  assert (row5 = [""; ""; ""]);

  (* Mixed *)
  let row6 = parse_csv_line "name,\"Alice, Bob\",42" in
  assert (row6 = ["name"; "Alice, Bob"; "42"]);

  (* Multi-row *)
  let csv = "a,b,c\n1,2,3\n\"x,y\",z,w" in
  let rows = parse_csv csv in
  assert (List.length rows = 3);
  assert (List.nth rows 0 = ["a"; "b"; "c"]);
  assert (List.nth rows 2 = ["x,y"; "z"; "w"]);

  Printf.printf "✓ All tests passed\n"
