(* OCaml — GC handles memory, patterns bind by value *)
type tree = Leaf | Node of int * tree * tree

let rec sum = function
  | Leaf -> 0
  | Node(v,l,r) -> v + sum l + sum r

let rec map f = function
  | Leaf -> Leaf
  | Node(v,l,r) -> Node(f v, map f l, map f r)

let () =
  let t = Node(1, Node(2,Leaf,Leaf), Node(3,Leaf,Leaf)) in
  Printf.printf "sum=%d\n" (sum t);
  Printf.printf "sum*2=%d\n" (sum (map ((*) 2) t))
