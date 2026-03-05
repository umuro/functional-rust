(* Trampoline in OCaml *)
type 'a bounce = Done of 'a | Bounce of (unit -> 'a bounce)

let run t =
  let rec go = function
    | Done v    -> v
    | Bounce th -> go (th ())
  in go t

let rec fact_t n acc =
  if n <= 0 then Done acc
  else Bounce (fun () -> fact_t (n-1) (n*acc))

let rec even_t n =
  if n = 0 then Done true
  else Bounce (fun () -> odd_t  (n-1))
and odd_t n =
  if n = 0 then Done false
  else Bounce (fun () -> even_t (n-1))

let () =
  Printf.printf "100! > 0: %b\n" (run (fact_t 100 1) > 0);
  Printf.printf "even 100: %b\n" (run (even_t 100));
  Printf.printf "even 101: %b\n" (run (even_t 101))
