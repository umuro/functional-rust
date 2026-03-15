(* Example 052: Functor Laws *)
(* Law 1 (Identity): map id x = x *)
(* Law 2 (Composition): map (f . g) x = map f (map g x) *)

type 'a maybe = Nothing | Just of 'a

let map f = function
  | Nothing -> Nothing
  | Just x -> Just (f x)

let id x = x
let compose f g x = f (g x)

(* Approach 1: Verify laws for Maybe *)
let verify_identity_law x =
  map id x = x

let verify_composition_law f g x =
  map (compose f g) x = map f (map g x)

(* Approach 2: Verify laws for List *)
let list_verify_identity xs =
  List.map id xs = xs

let list_verify_composition f g xs =
  List.map (compose f g) xs = List.map f (List.map g xs)

(* Approach 3: Counter-example — a "bad functor" that breaks laws *)
module BadFunctor = struct
  type 'a t = Bad of 'a * int  (* tracks map count *)

  let map f (Bad (x, count)) = Bad (f x, count + 1)
  (* Breaks identity law: map id (Bad(x, 0)) = Bad(x, 1) ≠ Bad(x, 0) *)
end

let () =
  (* Identity law for Maybe *)
  assert (verify_identity_law (Just 42));
  assert (verify_identity_law Nothing);

  (* Composition law for Maybe *)
  let f x = x * 2 in
  let g x = x + 3 in
  assert (verify_composition_law f g (Just 5));
  assert (verify_composition_law f g Nothing);

  (* Identity law for List *)
  assert (list_verify_identity [1; 2; 3]);
  assert (list_verify_identity []);

  (* Composition law for List *)
  assert (list_verify_composition f g [1; 2; 3]);

  (* Bad functor breaks identity *)
  let bad = BadFunctor.Bad (42, 0) in
  let mapped = BadFunctor.map id bad in
  assert (mapped <> bad);  (* Identity law violated! *)

  Printf.printf "✓ All tests passed\n"
