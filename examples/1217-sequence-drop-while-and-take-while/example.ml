(* Sequence — Drop While and Take While *)
(* Conditional prefix operations on sequences *)

let rec take_while p s () = match s () with
  | Seq.Nil -> Seq.Nil
  | Seq.Cons (x, rest) ->
    if p x then Seq.Cons (x, take_while p rest) else Seq.Nil

let rec drop_while p s () = match s () with
  | Seq.Nil -> Seq.Nil
  | Seq.Cons (x, rest) ->
    if p x then drop_while p rest () else Seq.Cons (x, rest)

let data = List.to_seq [1; 2; 3; 10; 20; 1; 2]
let prefix = take_while (fun x -> x < 10) data |> List.of_seq
let suffix = drop_while (fun x -> x < 10) data |> List.of_seq

let () =
  Printf.printf "take_while < 10: %s\n"
    (String.concat " " (List.map string_of_int prefix));
  Printf.printf "drop_while < 10: %s\n"
    (String.concat " " (List.map string_of_int suffix))
