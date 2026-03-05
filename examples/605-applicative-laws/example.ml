(* Applicative laws in OCaml *)
let pure x = Some x

let (<*>) mf mx = match (mf, mx) with
  | (Some f, Some x) -> Some (f x)
  | _                -> None

(* Identity: pure id <*> v = v *)
let identity_law v =
  (pure Fun.id <*> v) = v

(* Homomorphism: pure f <*> pure x = pure (f x) *)
let homomorphism_law f x =
  (pure f <*> pure x) = pure (f x)

(* Interchange: u <*> pure y = pure (fun f -> f y) <*> u *)
let interchange_law u y =
  (u <*> pure y) = (pure (fun f -> f y) <*> u)

let () =
  Printf.printf "identity: %b\n"      (identity_law (Some 42));
  Printf.printf "homomorphism: %b\n"  (homomorphism_law (fun x->x*2) 5);
  Printf.printf "interchange: %b\n"   (interchange_law (Some (fun x->x+1)) 42)
