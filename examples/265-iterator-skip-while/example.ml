(* 265: skip_while — discard elements while predicate holds, then yield the rest.
   Key property: once predicate fails, ALL remaining elements are kept,
   even if they would match the predicate again. *)

let rec skip_while f = function
  | []      -> []
  | x :: xs -> if f x then skip_while f xs else x :: xs

let () =
  (* Basic skip_while *)
  let r1 = skip_while (fun x -> x < 3) [1;2;3;4;5] in
  Printf.printf "skip_while (<3) [1..5]   = [%s]\n"
    (r1 |> List.map string_of_int |> String.concat ";");

  (* Includes elements after first failure — including later matches *)
  let r2 = skip_while (fun x -> x = 0) [0; 0; 1; 0] in
  Printf.printf "skip leading zeros       = [%s]\n"
    (r2 |> List.map string_of_int |> String.concat ";");

  (* Skip all — empty result *)
  let r3 = skip_while (fun x -> x < 10) [1;2;3] in
  Printf.printf "skip all [1;2;3]         = [%s]\n"
    (r3 |> List.map string_of_int |> String.concat ";");

  (* Skip none — predicate never true *)
  let r4 = skip_while (fun x -> x > 10) [1;2;3] in
  Printf.printf "skip none [1;2;3]        = [%s]\n"
    (r4 |> List.map string_of_int |> String.concat ";");

  (* Lazy version using Seq *)
  let r5 = List.to_seq [1;2;3;4;5]
           |> Seq.drop_while (fun x -> x < 3)
           |> List.of_seq in
  Printf.printf "seq skip_while (<3)      = [%s]\n"
    (r5 |> List.map string_of_int |> String.concat ";");

  (* Practical: skip leading whitespace tokens *)
  let tokens = [""; ""; "hello"; "world"; ""] in
  let non_empty = skip_while (fun s -> s = "") tokens in
  Printf.printf "skip empty tokens        = [%s]\n" (String.concat ";" non_empty)
