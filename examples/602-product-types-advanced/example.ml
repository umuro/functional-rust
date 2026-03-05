(* Product types and record update in OCaml *)
type point = { x: float; y: float }
type rect  = { origin: point; size: point }

(* Projections *)
let fst_point { x; _ } = x
let snd_point { y; _ } = y

(* Pairing morphism *)
let make_point x y = { x; y }

(* Record update *)
let translate dx dy p = { p with x=p.x+.dx; y=p.y+.dy }
let scale     s     p = { x=p.x*.s; y=p.y*.s }

(* Product bifunctor *)
let bimap f g (a,b) = (f a, g b)

(* Associativity via isomorphism *)
let assoc_l ((a,b),c) = (a,(b,c))
let assoc_r (a,(b,c)) = ((a,b),c)

let () =
  let p = make_point 1. 2. in
  let p2 = translate 3. 4. p in
  Printf.printf "(%.1f,%.1f)\n" p2.x p2.y;
  let scaled = scale 2. p in
  Printf.printf "scaled=(%.1f,%.1f)\n" scaled.x scaled.y;
  let (a,b) = bimap (fun x->x*2) (fun s->String.length s) (5,"hello") in
  Printf.printf "bimap=(%d,%d)\n" a b
