(* Bifunctor: a type with two type parameters, each mappable independently.
   bimap f g applies f to the "left" and g to the "right". *)

(* Either type as a canonical bifunctor *)
type ('a, 'b) either = Left of 'a | Right of 'b

let bimap f g = function
  | Left  a -> Left  (f a)
  | Right b -> Right (g b)

let map_left  f e = bimap f (fun x -> x) e
let map_right g e = bimap (fun x -> x) g e

(* Bifunctor laws:
   bimap id id = id
   bimap (f . g) (h . k) = bimap f h . bimap g k *)

let () =
  let e1 : (int, string) either = Left 42 in
  let e2 : (int, string) either = Right "hello" in

  (* bimap: transform both sides *)
  let r1 = bimap (fun n -> n * 2) String.uppercase_ascii e1 in
  let r2 = bimap (fun n -> n * 2) String.uppercase_ascii e2 in
  (match r1 with Left 84 -> Printf.printf "bimap Left: 84\n" | _ -> assert false);
  (match r2 with Right "HELLO" -> Printf.printf "bimap Right: HELLO\n" | _ -> assert false);

  (* map_left / map_right independently *)
  let r3 = map_left (fun n -> n + 1) e1 in
  let r4 = map_right String.length e2 in
  (match r3 with Left 43 -> Printf.printf "map_left: 43\n" | _ -> assert false);
  (match r4 with Right 5  -> Printf.printf "map_right: 5\n"  | _ -> assert false)
