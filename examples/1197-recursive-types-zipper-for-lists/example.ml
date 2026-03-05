(* Recursive Types — Zipper for Lists *)
(* Navigate a list with a zipper data structure *)

type 'a zipper = { left: 'a list; focus: 'a; right: 'a list }

let of_list = function
  | [] -> failwith "empty"
  | x :: xs -> { left = []; focus = x; right = xs }

let move_right z = match z.right with
  | [] -> None
  | x :: xs -> Some { left = z.focus :: z.left; focus = x; right = xs }

let move_left z = match z.left with
  | [] -> None
  | x :: xs -> Some { left = xs; focus = x; right = z.focus :: z.right }

let modify f z = { z with focus = f z.focus }
let to_list z = List.rev z.left @ [z.focus] @ z.right

let z = of_list [1;2;3;4;5]
let z = Option.get (move_right z)  (* focus = 2 *)
let z = Option.get (move_right z)  (* focus = 3 *)
let z = modify (( * ) 10) z        (* focus = 30 *)
let () = List.iter (fun x -> Printf.printf "%d " x) (to_list z)
