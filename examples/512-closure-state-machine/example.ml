(* State machine using closures in OCaml *)
(* Parsing a simple pattern: a*b+ (zero or more 'a', one or more 'b') *)

type result = Accept | Reject | Continue of (char -> result)

let rec state_start () : char -> result =
  fun c -> match c with
    | 'a' -> Continue (state_a ())
    | 'b' -> Continue (state_b ())
    | _   -> Reject

and state_a () : char -> result =
  fun c -> match c with
    | 'a' -> Continue (state_a ())
    | 'b' -> Continue (state_b ())
    | _   -> Reject

and state_b () : char -> result =
  fun c -> match c with
    | 'b' -> Continue (state_b ())
    | _   -> Reject

let run_machine input =
  let chars = List.of_seq (String.to_seq input) in
  match chars with
  | [] -> false  (* empty string doesn't match a*b+ *)
  | first :: rest ->
    let initial = state_start () first in
    let rec step state chars =
      match chars with
      | [] -> (match state with Accept -> true | Continue _ -> false | Reject -> false)
      | c :: cs ->
        match state with
        | Continue f -> step (f c) cs
        | Accept -> false
        | Reject -> false
    in
    (* Check if last state after processing all is Accept or we need final check *)
    let final_state = List.fold_left (fun st c ->
      match st with
      | Continue f -> f c
      | other -> other
    ) initial rest in
    match final_state with
    | Continue _ -> false  (* not in accept state at end *)
    | Accept -> true
    | Reject -> false

let () =
  let tests = ["b"; "ab"; "aab"; "abb"; "aabb"; ""; "a"; "ba"; "abc"] in
  List.iter (fun s ->
    Printf.printf "%S -> %b\n" s (run_machine s)
  ) tests
