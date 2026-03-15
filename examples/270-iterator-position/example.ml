(* 270: position — find the index of the first matching element.
   OCaml: List.find_index (5.1+), or manual fold with Option.
   Also: List.find_mapi for transform-and-find. *)

(* Find index of first element satisfying predicate — returns None if not found *)
let position pred lst =
  let rec go i = function
    | []      -> None
    | x :: xs -> if pred x then Some i else go (i + 1) xs
  in go 0 lst

(* rposition: find index of LAST matching element *)
let rposition pred lst =
  let n = List.length lst in
  let rec go i rev_lst =
    match rev_lst with
    | []      -> None
    | x :: xs -> if pred x then Some (n - 1 - i) else go (i + 1) xs
  in go 0 (List.rev lst)

let () =
  let v = [10; 20; 30; 40] in

  (* Find index of 30 *)
  Printf.printf "position(=30) = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int
      (position (fun x -> x = 30) v));

  (* Not found *)
  Printf.printf "position(=99) = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int
      (position (fun x -> x = 99) [1;2;3]));

  (* rposition: last occurrence *)
  let v2 = [1;2;3;2;1] in
  Printf.printf "rposition(=2) in [1;2;3;2;1] = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int
      (rposition (fun x -> x = 2) v2));

  (* First occurrence *)
  Printf.printf "position(=5) in [5;5;5] = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int
      (position (fun x -> x = 5) [5;5;5]));

  (* OCaml 5.1+ List.find_index *)
  (match List.find_index (fun x -> x = 30) v with
   | Some (idx, _) -> Printf.printf "List.find_index(=30) = %d\n" idx
   | None          -> print_endline "not found");

  (* Find index in a string list *)
  let words = ["apple"; "banana"; "cherry"] in
  Printf.printf "position of \"banana\" = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int
      (position (fun s -> s = "banana") words))
