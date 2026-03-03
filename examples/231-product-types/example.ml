(* Product types: combine multiple types into one.
   In category theory, the categorical product. *)

(* Record product type *)
type point2d = { x: float; y: float }
type point3d = { x: float; y: float; z: float }

(* Product as tuple *)
let swap (a, b) = (b, a)
let fst (a, _) = a
let snd (_, b) = b

(* Pair as a product with projections *)
let pair a b = (a, b)
let uncurry f (a, b) = f a b
let curry f a b = f (a, b)

let distance p q =
  let dx = p.x -. q.x and dy = p.y -. q.y in
  sqrt (dx *. dx +. dy *. dy)

let () =
  let p = { x = 0.; y = 0. } and q = { x = 3.; y = 4. } in
  Printf.printf "distance = %.1f\n" (distance p q);

  let pair = (42, "hello") in
  Printf.printf "fst=%d snd=%s\n" (fst pair) (snd pair);
  Printf.printf "swap: snd of swap = %d\n" (snd (swap pair));

  (* curry/uncurry *)
  let add_pair = uncurry ( + ) in
  Printf.printf "uncurry (+) (3,4) = %d\n" (add_pair (3, 4));
  Printf.printf "curry: %d\n" (curry add_pair 3 4)
