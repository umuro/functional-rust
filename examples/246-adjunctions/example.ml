(* Adjunction: F ⊣ G means Hom(F A, B) ≅ Hom(A, G B)
   Classic example: (- × C) ⊣ (C → -)
   i.e., curry/uncurry is an adjunction *)

(* The curry/uncurry adjunction *)
let curry   f a b = f (a, b)
let uncurry f (a, b) = f a b

(* unit of the adjunction: a -> C -> (A × C) *)
let unit c a = (a, c)

(* counit: (A × C → B) × (A × C) → B *)
let counit f pair = f pair

(* Verify the adjunction: curry . uncurry = id, uncurry . curry = id *)
let () =
  let f (a, b) = a + b in
  let g a b = a * b in

  let f' = curry (uncurry f) in
  assert (f' 3 4 = f (3, 4));

  let g' = uncurry (curry g) in
  assert (g' (3, 4) = g 3 4);

  Printf.printf "curry . uncurry = id: %b\n" (f' 3 4 = f (3, 4));
  Printf.printf "uncurry . curry = id: %b\n" (g' (3, 4) = g 3 4);

  (* The product-exponential adjunction *)
  (* Hom(A × B, C) ≅ Hom(A, B → C) *)
  let add_curried = curry (fun (a, b) -> a + b) in
  Printf.printf "curried add 3 4 = %d\n" (add_curried 3 4);
  let add_uncurried = uncurry (fun a b -> a + b) in
  Printf.printf "uncurried add (3,4) = %d\n" (add_uncurried (3, 4))
