(* Trampoline: convert deep recursion to a loop by returning thunks.
   Each step either returns a value (Done) or a thunk to continue (Bounce). *)

type 'a trampoline =
  | Done   : 'a -> 'a trampoline
  | Bounce : (unit -> 'a trampoline) -> 'a trampoline

let done_ x = Done x
let bounce f = Bounce f

(* Run the trampoline loop — O(1) stack! *)
let run t =
  let rec loop = function
    | Done x    -> x
    | Bounce f  -> loop (f ())
  in loop t

(* Even/odd without stack overflow via mutual recursion + trampoline *)
let rec is_even_t n =
  if n = 0 then done_ true
  else bounce (fun () -> is_odd_t (n - 1))
and is_odd_t n =
  if n = 0 then done_ false
  else bounce (fun () -> is_even_t (n - 1))

(* Factorial via trampoline *)
let factorial_t n =
  let rec go n acc =
    if n <= 1 then done_ acc
    else bounce (fun () -> go (n - 1) (n * acc))
  in run (go n 1)

let () =
  Printf.printf "is_even 1000000 = %b\n" (run (is_even_t 1_000_000));
  Printf.printf "is_odd  999999  = %b\n" (run (is_odd_t  999_999));
  Printf.printf "10!     = %d\n"         (factorial_t 10);
  Printf.printf "No stack overflow on deep recursion\n"
