(* Example 170: Error Recovery *)
(* Parser error messages with position info and expected tokens *)

type position = { offset: int; line: int; col: int }

type parse_error = {
  pos: position;
  expected: string list;
  got: string;
}

type 'a parse_result = ('a * string * position, parse_error) result
type 'a parser = string -> position -> 'a parse_result

let advance_pos pos c =
  if c = '\n' then { offset = pos.offset + 1; line = pos.line + 1; col = 1 }
  else { offset = pos.offset + 1; line = pos.line; col = pos.col + 1 }

let init_pos = { offset = 0; line = 1; col = 1 }

let format_error err =
  Printf.sprintf "Error at line %d, col %d: expected %s, got %s"
    err.pos.line err.pos.col
    (String.concat " or " err.expected)
    err.got

(* Approach 1: Satisfy with position tracking *)
let satisfy pred desc : char parser = fun input pos ->
  if String.length input > 0 && pred input.[0] then
    let c = input.[0] in
    Ok (c, String.sub input 1 (String.length input - 1), advance_pos pos c)
  else
    let got = if String.length input > 0 then Printf.sprintf "'%c'" input.[0] else "EOF" in
    Error { pos; expected = [desc]; got }

let tag expected : string parser = fun input pos ->
  let len = String.length expected in
  if String.length input >= len && String.sub input 0 len = expected then
    let new_pos = ref pos in
    String.iter (fun c -> new_pos := advance_pos !new_pos c) expected;
    Ok (expected, String.sub input len (String.length input - len), !new_pos)
  else
    let got = if String.length input >= len then Printf.sprintf "\"%s\"" (String.sub input 0 len)
              else Printf.sprintf "\"%s\"" input in
    Error { pos; expected = [Printf.sprintf "\"%s\"" expected]; got }

(* Approach 2: Error merging for alternatives *)
let alt (p1 : 'a parser) (p2 : 'a parser) : 'a parser = fun input pos ->
  match p1 input pos with
  | Ok _ as r -> r
  | Error e1 ->
    match p2 input pos with
    | Ok _ as r -> r
    | Error e2 ->
      (* Merge errors: if at same position, combine expected lists *)
      if e1.pos.offset = e2.pos.offset then
        Error { pos = e1.pos; expected = e1.expected @ e2.expected; got = e1.got }
      else if e1.pos.offset > e2.pos.offset then Error e1
      else Error e2

(* Approach 3: Error recovery — skip to sync point *)
let recover_until (sync : char -> bool) (p : 'a parser) : 'a option parser = fun input pos ->
  match p input pos with
  | Ok (v, rest, new_pos) -> Ok (Some v, rest, new_pos)
  | Error _err ->
    (* Skip characters until we find a sync point *)
    let rec skip inp p =
      if String.length inp = 0 then Ok (None, "", p)
      else if sync inp.[0] then Ok (None, inp, p)
      else skip (String.sub inp 1 (String.length inp - 1)) (advance_pos p inp.[0])
    in
    skip input pos

(* Tests *)
let () =
  let digit = satisfy (fun c -> c >= '0' && c <= '9') "digit" in
  (match digit "abc" init_pos with
   | Error err ->
     assert (err.pos.line = 1 && err.pos.col = 1);
     assert (err.expected = ["digit"]);
     assert (err.got = "'a'")
   | _ -> failwith "Expected error");

  let digit_or_letter = alt digit
    (satisfy (fun c -> (c >= 'a' && c <= 'z')) "letter") in
  (match digit_or_letter "!" init_pos with
   | Error err ->
     assert (List.length err.expected = 2);
     assert (List.mem "digit" err.expected);
     assert (List.mem "letter" err.expected)
   | _ -> failwith "Expected merged error");

  (* Position tracking *)
  let pos = { offset = 0; line = 1; col = 1 } in
  (match tag "ab" "ab\ncd" pos with
   | Ok (_, _, new_pos) ->
     assert (new_pos.line = 1 && new_pos.col = 3)
   | _ -> failwith "Tag failed");

  print_endline "✓ All tests passed"
