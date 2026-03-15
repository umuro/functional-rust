(* Example 053: Applicative Functor Basics *)
(* Applicative: apply a wrapped function to a wrapped value *)

type 'a maybe = Nothing | Just of 'a

let map f = function Nothing -> Nothing | Just x -> Just (f x)
let pure x = Just x
let apply mf mx = match mf with
  | Nothing -> Nothing
  | Just f -> map f mx

(* Infix operators *)
let ( <$> ) f x = map f x
let ( <*> ) = apply

(* Approach 1: Apply function in context *)
let add x y = x + y

let result1 = (pure add) <*> (Just 3) <*> (Just 4)
(* = Just 7 *)

(* Approach 2: Lifting a multi-argument function *)
let lift2 f a b = (pure f) <*> a <*> b
let lift3 f a b c = (pure f) <*> a <*> b <*> c

let concat3 a b c = a ^ b ^ c

(* Approach 3: Using applicative for independent computations *)
let parse_int s = try Just (int_of_string s) with _ -> Nothing
let parse_float s = try Just (float_of_string s) with _ -> Nothing

let make_pair x y = (x, y)

let () =
  (* Basic apply *)
  assert (result1 = Just 7);
  assert (apply (Just (fun x -> x * 2)) (Just 5) = Just 10);
  assert (apply Nothing (Just 5) = Nothing);
  assert (apply (Just (fun x -> x * 2)) Nothing = Nothing);

  (* lift2 *)
  assert (lift2 add (Just 10) (Just 20) = Just 30);
  assert (lift2 add Nothing (Just 20) = Nothing);

  (* lift3 *)
  assert (lift3 concat3 (Just "a") (Just "b") (Just "c") = Just "abc");

  (* Independent parsing *)
  let pair = lift2 make_pair (parse_int "42") (parse_int "7") in
  assert (pair = Just (42, 7));
  let pair2 = lift2 make_pair (parse_int "bad") (parse_int "7") in
  assert (pair2 = Nothing);

  Printf.printf "✓ All tests passed\n"
