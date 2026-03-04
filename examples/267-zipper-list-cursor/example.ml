(* Idiomatic OCaml: record-based zipper *)
type 'a zipper = { left: 'a list; focus: 'a; right: 'a list }

let of_list = function
  | [] -> failwith "empty"
  | h :: t -> { left = []; focus = h; right = t }

let go_right z = match z.right with
  | [] -> None
  | h :: t -> Some { left = z.focus :: z.left; focus = h; right = t }

let go_left z = match z.left with
  | [] -> None
  | h :: t -> Some { left = t; focus = h; right = z.focus :: z.right }

let update f z = { z with focus = f z.focus }
let to_list z = List.rev z.left @ [z.focus] @ z.right

(* Functional navigation using |> pipeline *)
let () =
  let z = of_list [1;2;3;4;5] in
  let z = Option.get (go_right z) in
  let z = Option.get (go_right z) in
  let z = update (fun x -> x * 10) z in
  assert (to_list z = [1; 2; 30; 4; 5]);
  List.iter (Printf.printf "%d ") (to_list z);
  print_newline ()

(* Navigation boundary tests *)
let () =
  let z = of_list [1] in
  assert (go_right z = None);
  assert (go_left z = None);
  let z2 = of_list [1;2;3] in
  let z2 = Option.get (go_right z2) in
  let z2 = Option.get (go_left z2) in
  assert (z2.focus = 1);
  print_endline "ok"
