(* Monad laws in OCaml *)
let (>>=) = Option.bind
let return x = Some x

(* Left identity: return a >>= f == f a *)
let left_identity a f = (return a >>= f) = f a

(* Right identity: m >>= return == m *)
let right_identity m = (m >>= return) = m

(* Associativity: (m >>= f) >>= g == m >>= (fun x -> f x >>= g) *)
let associativity m f g =
  ((m >>= f) >>= g) = (m >>= fun x -> f x >>= g)

let () =
  let f x = if x > 0 then Some (x*2) else None in
  let g x = if x < 100 then Some (x+1) else None in
  Printf.printf "left_identity(5,f): %b\n"     (left_identity 5 f);
  Printf.printf "right_identity(Some 5): %b\n" (right_identity (Some 5));
  Printf.printf "right_identity(None): %b\n"   (right_identity None);
  Printf.printf "associativity: %b\n"          (associativity (Some 5) f g)
