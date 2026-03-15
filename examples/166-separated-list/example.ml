(* Example 166: Separated List *)
(* separated_list0, separated_list1: comma-separated values *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

let satisfy pred desc : char parser = fun input ->
  if String.length input > 0 && pred input.[0] then
    Ok (input.[0], String.sub input 1 (String.length input - 1))
  else Error (Printf.sprintf "Expected %s" desc)

let many1 p : 'a list parser = fun input ->
  let rec go acc r = match p r with Ok (v, r') -> go (v::acc) r' | Error _ -> Ok (List.rev acc, r)
  in match p input with Error e -> Error e
  | Ok (v, r) -> go [v] r

let map f p : 'b parser = fun input ->
  match p input with Ok (v, r) -> Ok (f v, r) | Error e -> Error e

let ws0 : unit parser = fun input ->
  let rec skip i = if i < String.length input &&
    (input.[i] = ' ' || input.[i] = '\t' || input.[i] = '\n') then skip (i+1) else i in
  let i = skip 0 in Ok ((), String.sub input i (String.length input - i))

(* Approach 1: separated_list0 — zero or more items separated by sep *)
let separated_list0 (sep : 'b parser) (item : 'a parser) : 'a list parser = fun input ->
  match item input with
  | Error _ -> Ok ([], input)
  | Ok (first, rest) ->
    let rec go acc remaining =
      match sep remaining with
      | Error _ -> Ok (List.rev acc, remaining)
      | Ok (_, after_sep) ->
        match item after_sep with
        | Error _ -> Ok (List.rev acc, remaining)  (* trailing sep: backtrack *)
        | Ok (v, rest') -> go (v :: acc) rest'
    in
    go [first] rest

(* Approach 2: separated_list1 — one or more *)
let separated_list1 (sep : 'b parser) (item : 'a parser) : 'a list parser = fun input ->
  match separated_list0 sep item input with
  | Ok ([], _) -> Error "Expected at least one item"
  | result -> result

(* Approach 3: separated_list with trailing separator allowed *)
let separated_list_trailing (sep : 'b parser) (item : 'a parser) : 'a list parser = fun input ->
  match item input with
  | Error _ -> Ok ([], input)
  | Ok (first, rest) ->
    let rec go acc remaining =
      match sep remaining with
      | Error _ -> Ok (List.rev acc, remaining)
      | Ok (_, after_sep) ->
        match item after_sep with
        | Error _ -> Ok (List.rev acc, after_sep)  (* consume trailing sep *)
        | Ok (v, rest') -> go (v :: acc) rest'
    in
    go [first] rest

let digit_str = map (fun ds -> String.init (List.length ds) (List.nth ds))
  (many1 (satisfy (fun c -> c >= '0' && c <= '9') "digit"))

let comma : char parser = fun input ->
  match ws0 input with
  | Ok ((), r) ->
    if String.length r > 0 && r.[0] = ',' then
      match ws0 (String.sub r 1 (String.length r - 1)) with
      | Ok ((), r') -> Ok (',', r')
      | Error e -> Error e
    else Error "Expected ','"
  | Error e -> Error e

(* Tests *)
let () =
  assert (separated_list0 comma digit_str "1, 2, 3" = Ok (["1"; "2"; "3"], ""));
  assert (separated_list0 comma digit_str "" = Ok ([], ""));
  assert (separated_list0 comma digit_str "42" = Ok (["42"], ""));

  assert (separated_list1 comma digit_str "1, 2" = Ok (["1"; "2"], ""));
  assert (Result.is_error (separated_list1 comma digit_str ""));

  assert (separated_list_trailing comma digit_str "1, 2, " = Ok (["1"; "2"], ""));

  print_endline "✓ All tests passed"
