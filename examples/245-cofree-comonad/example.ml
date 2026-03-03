(* Cofree comonad: annotates every node in a structure with a label.
   Cofree f a = a * f (Cofree f a)
   For f = [], this gives rose trees (annotated with a-values at each node) *)

(* Cofree over list = rose tree *)
type 'a rose = Rose of 'a * 'a rose list

let leaf x       = Rose (x, [])
let node x children = Rose (x, children)

(* Comonad operations *)
let extract (Rose (a, _)) = a

let rec extend (Rose (a, children)) f =
  Rose (f (Rose (a, children)), List.map (fun child -> extend child f) children)

let duplicate t = extend t (fun x -> x)

(* Functor: map over annotations *)
let rec fmap f (Rose (a, children)) =
  Rose (f a, List.map (fmap f) children)

(* Fold: reduce the tree *)
let rec fold f (Rose (a, children)) =
  f a (List.map (fold f) children)

(* Size and depth *)
let size  = fold (fun _ cs -> 1 + List.fold_left ( + ) 0 cs)
let depth = fold (fun _ cs -> 1 + (List.fold_left max 0 cs))
let sum   = fold (fun a cs -> a + List.fold_left ( + ) 0 cs)

let () =
  let t = node 1 [
    node 2 [leaf 4; leaf 5];
    node 3 [leaf 6; node 7 [leaf 8]];
  ] in

  Printf.printf "root   = %d\n" (extract t);
  Printf.printf "size   = %d\n" (size t);
  Printf.printf "depth  = %d\n" (depth t);
  Printf.printf "sum    = %d\n" (sum t);

  let doubled = fmap (fun n -> n * 2) t in
  Printf.printf "root*2 = %d\n" (extract doubled);

  (* extend: annotate each node with its subtree sum *)
  let annotated = extend t sum in
  Printf.printf "subtree sums at root = %d\n" (extract annotated)
