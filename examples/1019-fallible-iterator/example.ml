(* 1019: Fallible Iterator *)
(* Iterator that can fail: producing Option(Result) values *)

(* Approach 1: Seq-based fallible iteration *)
let parse_line line =
  match int_of_string_opt (String.trim line) with
  | Some n -> Ok n
  | None -> Error (Printf.sprintf "bad line: %s" line)

let lines_to_numbers lines =
  List.to_seq lines |> Seq.map parse_line

(* Process until first error *)
let take_while_ok seq =
  let rec aux acc seq =
    match seq () with
    | Seq.Nil -> Ok (List.rev acc)
    | Seq.Cons (Ok v, rest) -> aux (v :: acc) rest
    | Seq.Cons (Error e, _) -> Error e
  in
  aux [] seq

(* Approach 2: Manual stateful iterator *)
type 'a fallible_iter = {
  mutable items: string list;
  parse: string -> ('a, string) result;
}

let next iter =
  match iter.items with
  | [] -> None
  | x :: rest ->
    iter.items <- rest;
    Some (iter.parse x)

let collect_fallible iter =
  let rec aux acc =
    match next iter with
    | None -> Ok (List.rev acc)
    | Some (Ok v) -> aux (v :: acc)
    | Some (Error e) -> Error e
  in
  aux []

let test_seq () =
  let lines = ["1"; "2"; "3"] in
  let seq = lines_to_numbers lines in
  assert (take_while_ok seq = Ok [1; 2; 3]);
  let bad_lines = ["1"; "abc"; "3"] in
  let seq = lines_to_numbers bad_lines in
  (match take_while_ok seq with
   | Error e -> assert (e = "bad line: abc")
   | Ok _ -> assert false);
  Printf.printf "  Approach 1 (Seq-based): passed\n"

let test_stateful () =
  let iter = { items = ["10"; "20"; "30"]; parse = parse_line } in
  assert (collect_fallible iter = Ok [10; 20; 30]);
  let iter = { items = ["10"; "bad"; "30"]; parse = parse_line } in
  (match collect_fallible iter with
   | Error _ -> ()
   | Ok _ -> assert false);
  Printf.printf "  Approach 2 (stateful iterator): passed\n"

let () =
  Printf.printf "Testing fallible iterator:\n";
  test_seq ();
  test_stateful ();
  Printf.printf "✓ All tests passed\n"
