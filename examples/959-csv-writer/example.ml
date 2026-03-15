(* 959: CSV Writer *)
(* Escape quotes and handle special characters *)

(* Approach 1: Escape a single field *)

let needs_quoting s =
  String.contains s ',' ||
  String.contains s '"' ||
  String.contains s '\n' ||
  String.contains s '\r'

let escape_field s =
  if needs_quoting s then begin
    (* Replace " with "" and wrap in quotes *)
    let buf = Buffer.create (String.length s + 2) in
    Buffer.add_char buf '"';
    String.iter (fun c ->
      if c = '"' then Buffer.add_string buf "\"\""
      else Buffer.add_char buf c
    ) s;
    Buffer.add_char buf '"';
    Buffer.contents buf
  end else
    s

(* Approach 2: Write a row *)

let write_row fields =
  String.concat "," (List.map escape_field fields)

(* Approach 3: Write a complete CSV *)

let write_csv rows =
  String.concat "\n" (List.map write_row rows)

let () =
  (* Simple field - no quoting needed *)
  assert (escape_field "hello" = "hello");
  assert (escape_field "42" = "42");

  (* Comma triggers quoting *)
  assert (escape_field "one, two" = "\"one, two\"");

  (* Quote gets doubled *)
  assert (escape_field "say \"hi\"" = "\"say \"\"hi\"\"\"");

  (* Newline triggers quoting *)
  assert (escape_field "line1\nline2" = "\"line1\nline2\"");

  (* Write rows *)
  assert (write_row ["name"; "age"; "city"] = "name,age,city");
  assert (write_row ["Alice, Smith"; "30"; "Amsterdam"] = "\"Alice, Smith\",30,Amsterdam");

  (* Full CSV *)
  let rows = [
    ["name"; "age"; "city"];
    ["Alice, Smith"; "30"; "Amsterdam"];
    ["Bob"; "25"; "say \"hi\""];
  ] in
  let csv = write_csv rows in
  assert (String.length csv > 0);
  let lines = String.split_on_char '\n' csv in
  assert (List.length lines = 3);
  assert (List.nth lines 0 = "name,age,city");
  assert (List.nth lines 1 = "\"Alice, Smith\",30,Amsterdam");
  assert (List.nth lines 2 = "Bob,25,\"say \"\"hi\"\"\"");

  Printf.printf "✓ All tests passed\n"
