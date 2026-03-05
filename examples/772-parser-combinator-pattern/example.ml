(* Parser combinator in OCaml — nom-style *)

(* ── Core type ──────────────────────────────────────────────────────────────── *)
(* A parser: input → (value, rest) option *)
type 'a parser = string -> ('a * string) option

(* ── Primitives ─────────────────────────────────────────────────────────────── *)
let char_p c : char parser = fun s ->
  if s = "" then None
  else if s.[0] = c then Some (c, String.sub s 1 (String.length s - 1))
  else None

let tag prefix : string parser = fun s ->
  let n = String.length prefix in
  if String.length s >= n && String.sub s 0 n = prefix
  then Some (prefix, String.sub s n (String.length s - n))
  else None

let take_while pred : string parser = fun s ->
  let i = ref 0 in
  let len = String.length s in
  while !i < len && pred s.[!i] do incr i done;
  Some (String.sub s 0 !i, String.sub s !i (len - !i))

let digit = take_while (fun c -> c >= '0' && c <= '9')
let alpha  = take_while (fun c -> (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z'))

(* ── Combinators ─────────────────────────────────────────────────────────────── *)
let map p f : 'b parser = fun s ->
  match p s with None -> None | Some (v, rest) -> Some (f v, rest)

let ( *> ) p1 p2 : 'b parser = fun s ->
  match p1 s with None -> None | Some (_, rest) -> p2 rest

let ( <* ) p1 p2 : 'a parser = fun s ->
  match p1 s with None -> None
  | Some (v, rest) ->
    match p2 rest with None -> None | Some (_, rest2) -> Some (v, rest2)

let pair p1 p2 : ('a * 'b) parser = fun s ->
  match p1 s with None -> None
  | Some (a, rest) ->
    match p2 rest with None -> None | Some (b, rest2) -> Some ((a, b), rest2)

let sep_by p sep : 'a list parser = fun s ->
  match p s with
  | None -> Some ([], s)
  | Some (first, rest) ->
    let acc = ref [first] in
    let current = ref rest in
    let continue = ref true in
    while !continue do
      match (sep *> p) !current with
      | None -> continue := false
      | Some (v, rest2) -> acc := v :: !acc; current := rest2
    done;
    Some (List.rev !acc, !current)

(* ── Key-value parser ─────────────────────────────────────────────────────────── *)
let key_value =
  pair (alpha <* char_p '=') (take_while (fun c -> c <> ',' && c <> '\n'))

let kv_list = sep_by key_value (char_p ',')

let () =
  let input = "name=Alice,age=30,city=Berlin" in
  match kv_list input with
  | Some (pairs, rest) ->
    Printf.printf "Parsed %d pairs (rest=%S):\n" (List.length pairs) rest;
    List.iter (fun (k, v) -> Printf.printf "  %s => %s\n" k v) pairs
  | None -> Printf.printf "Parse failed\n"
