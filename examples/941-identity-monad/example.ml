(* Identity monad — the simplest possible monad.
   Wraps a value with zero extra effects.
   Useful as a base case in monad transformers. *)

type 'a t = Identity of 'a

let return x = Identity x

let bind (Identity x) f = f x

let (>>=) m f = bind m f

let map f (Identity x) = Identity (f x)

let run (Identity x) = x

(* Functor law: map id = id *)
let () =
  let v = Identity 42 in
  assert (map (fun x -> x) v = v);
  Printf.printf "Identity monad laws hold\n"

(* Monad: sequence computations *)
let () =
  let result =
    return 10
    >>= (fun x -> return (x * 2))
    >>= (fun x -> return (x + 1))
  in
  assert (run result = 21);
  Printf.printf "chain: %d\n" (run result)
