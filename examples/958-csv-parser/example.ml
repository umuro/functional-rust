(* 958: CSV Parser

   OCaml: state machine over characters using a variant type.
   Three approaches:
   1. Simple split (no quote handling)
   2. Full state machine with RFC 4180 quote handling
   3. Multi-row parser *)

(* ── Approach 1: Simple split ────────────────────────────────────────────── *)

let split_simple line =
  String.split_on_char ',' line

(* ── Approach 2: Full CSV state machine ─────────────────────────────────── *)

type state = Normal | InQuote | AfterQuote

let parse_csv_line line =
  let fields  = Buffer.create 64 in
  let current = Buffer.create 16 in
  let state   = ref Normal in
  String.iter (fun c ->
    match (!state, c) with
    | (Normal,     '"') -> state := InQuote
    | (Normal,     ',') ->
      Buffer.add_string fields (Buffer.contents current);
      Buffer.add_char fields '\x00';   (* separator between fields *)
      Buffer.clear current
    | (Normal,      c ) -> Buffer.add_char current c
    | (InQuote,    '"') -> state := AfterQuote
    | (InQuote,     c ) -> Buffer.add_char current c
    | (AfterQuote, '"') ->
      Buffer.add_char current '"';
      state := InQuote
    | (AfterQuote, ',') ->
      Buffer.add_string fields (Buffer.contents current);
      Buffer.add_char fields '\x00';
      Buffer.clear current;
      state := Normal
    | (AfterQuote,  _ ) -> state := Normal
  ) line;
  (* Push last field *)
  Buffer.add_string fields (Buffer.contents current);
  (* Split on null separator *)
  String.split_on_char '\x00' (Buffer.contents fields)

(* Alternative: accumulate into a list, avoid the double-buffer trick *)
let parse_csv_line_clean line =
  let current = Buffer.create 16 in
  let state   = ref Normal in
  let push_field acc =
    let f = Buffer.contents current in
    Buffer.clear current;
    f :: acc
  in
  let result = String.fold_left (fun acc c ->
    match (!state, c) with
    | (Normal,     '"') -> state := InQuote; acc
    | (Normal,     ',') -> push_field acc
    | (Normal,      c ) -> Buffer.add_char current c; acc
    | (InQuote,    '"') -> state := AfterQuote; acc
    | (InQuote,     c ) -> Buffer.add_char current c; acc
    | (AfterQuote, '"') ->
      Buffer.add_char current '"';
      state := InQuote; acc
    | (AfterQuote, ',') ->
      let a = push_field acc in
      state := Normal; a
    | (AfterQuote,  _ ) -> state := Normal; acc
  ) [] line in
  List.rev (push_field result)

(* ── Approach 3: Multi-row parser ────────────────────────────────────────── *)

let parse_csv text =
  text
  |> String.split_on_char '\n'
  |> List.filter (fun l -> l <> "")
  |> List.map parse_csv_line_clean

let () =
  (* simple split *)
  assert (split_simple "a,b,c" = ["a"; "b"; "c"]);
  assert (split_simple "one"   = ["one"]);

  let parse = parse_csv_line_clean in

  (* quoted fields *)
  assert (parse "\"hello\",\"world\",plain" = ["hello"; "world"; "plain"]);

  (* comma inside quotes *)
  assert (parse "\"one, two\",three" = ["one, two"; "three"]);

  (* escaped quotes: "" inside quoted field *)
  assert (parse "\"say \"\"hi\"\"\",end" = ["say \"hi\""; "end"]);

  (* empty fields *)
  assert (parse ",,"  = [""; ""; ""]);
  assert (parse "a,,c" = ["a"; ""; "c"]);

  (* mixed *)
  assert (parse "name,\"Alice, Bob\",42" = ["name"; "Alice, Bob"; "42"]);

  (* multi-row *)
  let csv = "a,b,c\n1,2,3\n\"x,y\",z,w" in
  let rows = parse_csv csv in
  assert (List.length rows = 3);
  assert (List.nth rows 0 = ["a"; "b"; "c"]);
  assert (List.nth rows 2 = ["x,y"; "z"; "w"]);

  print_endline "958-csv-parser: all tests passed"
