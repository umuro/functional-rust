(* 959: CSV Writer

   Escape quotes, handle commas/newlines in fields, produce valid RFC 4180 CSV. *)

(* ── Approach 1: Escape a single field ───────────────────────────────────── *)

let needs_quoting s =
  String.contains s ',' ||
  String.contains s '"' ||
  String.contains s '\n' ||
  String.contains s '\r'

let escape_field s =
  if needs_quoting s then begin
    let buf = Buffer.create (String.length s + 2) in
    Buffer.add_char buf '"';
    String.iter (fun c ->
      if c = '"' then (Buffer.add_char buf '"'; Buffer.add_char buf '"')
      else Buffer.add_char buf c
    ) s;
    Buffer.add_char buf '"';
    Buffer.contents buf
  end else s

(* ── Approach 2: Write a single row ──────────────────────────────────────── *)

let write_row fields =
  String.concat "," (List.map escape_field fields)

(* ── Approach 3: Write complete CSV ─────────────────────────────────────── *)

let write_csv rows =
  String.concat "\n" (List.map write_row rows)

(* ── Approach 4: Write CSV with header ──────────────────────────────────── *)

let write_csv_with_header header rows =
  write_csv (header :: rows)

(* ── Parse back (round-trip check) ──────────────────────────────────────── *)

(* Minimal re-parse for round-trip verification *)
let parse_row line =
  (* Simple: split on comma, respecting quoted fields *)
  let current = Buffer.create 16 in
  let in_quote = ref false in
  let result = ref [] in
  String.iter (fun c ->
    match (!in_quote, c) with
    | (false, '"') -> in_quote := true
    | (true,  '"') -> in_quote := false
    | (false, ',') ->
      result := Buffer.contents current :: !result;
      Buffer.clear current
    | (_,      c ) -> Buffer.add_char current c
  ) line;
  result := Buffer.contents current :: !result;
  List.rev !result

let () =
  (* no quoting needed *)
  assert (escape_field "hello" = "hello");
  assert (escape_field "42"    = "42");
  assert (escape_field ""      = "");

  (* comma quoting *)
  assert (escape_field "one, two" = "\"one, two\"");

  (* quote escaping: " becomes "" *)
  assert (escape_field "say \"hi\"" = "\"say \"\"hi\"\"\"");

  (* newline quoting *)
  assert (escape_field "line1\nline2" = "\"line1\nline2\"");

  (* write_row plain *)
  assert (write_row ["name"; "age"; "city"] = "name,age,city");

  (* write_row with special *)
  assert (write_row ["Alice, Smith"; "30"; "Amsterdam"] =
          "\"Alice, Smith\",30,Amsterdam");

  (* write_csv *)
  let rows = [
    ["name"; "age"; "city"];
    ["Alice, Smith"; "30"; "Amsterdam"];
    ["Bob"; "25"; "say \"hi\""];
  ] in
  let csv = write_csv rows in
  let lines = String.split_on_char '\n' csv in
  assert (List.length lines = 3);
  assert (List.nth lines 0 = "name,age,city");
  assert (List.nth lines 1 = "\"Alice, Smith\",30,Amsterdam");
  assert (List.nth lines 2 = "Bob,25,\"say \"\"hi\"\"\"");

  (* Round trip: write then parse *)
  let original = ["Alice, Bob"; "30"; "New \"York\""] in
  let written  = write_row original in
  let parsed   = parse_row written in
  assert (parsed = original);

  print_endline "959-csv-writer: all tests passed"
