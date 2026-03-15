(* Example 057: Monad Laws *)
(* Law 1 (Left Identity):  return a >>= f  ≡  f a *)
(* Law 2 (Right Identity): m >>= return    ≡  m *)
(* Law 3 (Associativity):  (m >>= f) >>= g ≡ m >>= (fun x -> f x >>= g) *)

let return_ x = Some x
let bind m f = match m with None -> None | Some x -> f x
let ( >>= ) = bind

(* Test functions *)
let double x = Some (x * 2)
let inc x = Some (x + 1)
let safe_div10 x = if x = 0 then None else Some (10 / x)

(* Approach 1: Verify for Option *)
let verify_left_identity a f =
  (return_ a >>= f) = (f a)

let verify_right_identity m =
  (m >>= return_) = m

let verify_associativity m f g =
  ((m >>= f) >>= g) = (m >>= (fun x -> f x >>= g))

(* Approach 2: Verify for Result *)
let rbind r f = match r with Error e -> Error e | Ok x -> f x
let rreturn x = Ok x

let verify_result_left_identity a f =
  rbind (rreturn a) f = f a

let verify_result_right_identity m =
  rbind m rreturn = m

let verify_result_associativity m f g =
  rbind (rbind m f) g = rbind m (fun x -> rbind (f x) g)

(* Approach 3: Verify for List monad *)
let lbind xs f = List.concat_map f xs
let lreturn x = [x]

let verify_list_left_identity a f =
  lbind (lreturn a) f = f a

let verify_list_associativity m f g =
  lbind (lbind m f) g = lbind m (fun x -> lbind (f x) g)

let () =
  (* Option: Left identity *)
  assert (verify_left_identity 5 double);
  assert (verify_left_identity 0 safe_div10);

  (* Option: Right identity *)
  assert (verify_right_identity (Some 42));
  assert (verify_right_identity None);

  (* Option: Associativity *)
  assert (verify_associativity (Some 5) double inc);
  assert (verify_associativity None double inc);
  assert (verify_associativity (Some 0) safe_div10 double);

  (* Result laws *)
  let rf x = Ok (x * 2) in
  let rg x = Ok (x + 1) in
  assert (verify_result_left_identity 5 rf);
  assert (verify_result_right_identity (Ok 42));
  assert (verify_result_right_identity (Error "oops"));
  assert (verify_result_associativity (Ok 5) rf rg);

  (* List laws *)
  let expand x = [x; x * 10] in
  let negate x = [-x; x] in
  assert (verify_list_left_identity 3 expand);
  assert (verify_list_associativity [1; 2] expand negate);

  Printf.printf "✓ All tests passed\n"
