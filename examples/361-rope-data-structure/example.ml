(* OCaml: simple rope implementation *)

type rope =
  | Leaf of string
  | Node of rope * rope * int  (* left, right, total_len *)

let length = function
  | Leaf s -> String.length s
  | Node (_,_,n) -> n

let make_node l r = Node (l, r, length l + length r)

let concat a b = match a, b with
  | Leaf "", _ -> b | _, Leaf "" -> a
  | _ -> make_node a b

let rec to_string = function
  | Leaf s -> s
  | Node (l,r,_) -> to_string l ^ to_string r

let rec index_at rope i =
  match rope with
  | Leaf s -> s.[i]
  | Node (l,_,_) when i < length l -> index_at l i
  | Node (l,r,_) -> index_at r (i - length l)

let () =
  let r = concat (Leaf "Hello, ") (concat (Leaf "World") (Leaf "!")) in
  Printf.printf "%s\n" (to_string r);
  Printf.printf "Length: %d\n" (length r);
  Printf.printf "Char at 7: %c\n" (index_at r 7)
