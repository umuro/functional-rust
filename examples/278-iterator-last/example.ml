(* 278: last — return the final element (or None for empty).
   OCaml: List.rev + List.hd, or a tail-recursive fold. *)

(* last: return the last element of a list *)
let last = function
  | [] -> None
  | lst -> Some (List.nth lst (List.length lst - 1))

(* Tail-recursive version using fold_left *)
let last_fold = function
  | []      -> None
  | x :: xs -> Some (List.fold_left (fun _ y -> y) x xs)

let () =
  (* Basic last *)
  Printf.printf "last [1..5] = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (last [1;2;3;4;5]));

  (* Empty *)
  Printf.printf "last [] = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (last []));

  (* Last after filter *)
  let last_even = List.init 10 (fun i -> i + 1)
    |> List.filter (fun x -> x mod 2 = 0)
    |> last in
  Printf.printf "last even in 1..10 = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int last_even);

  (* Single element *)
  Printf.printf "last [42] = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (last [42]));

  (* Using fold version *)
  Printf.printf "last_fold [1;2;3] = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (last_fold [1;2;3]));

  (* Lazy: last of a Seq requires consuming it all *)
  let lazy_last = Seq.ints 1
    |> Seq.take 10
    |> Seq.fold_left (fun _ x -> Some x) None in
  Printf.printf "lazy last of first 10 = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int lazy_last)
