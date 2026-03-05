(* Hashtbl — LRU Cache Pattern *)
(* Simple memoization with Hashtbl *)

let memoize f =
  let cache = Hashtbl.create 16 in
  fun x ->
    match Hashtbl.find_opt cache x with
    | Some v -> v
    | None ->
      let v = f x in
      Hashtbl.add cache x v;
      v

let rec fib_slow n =
  if n <= 1 then n else fib_slow (n-1) + fib_slow (n-2)

(* Need explicit rec + memo for recursive memoization *)
let fib =
  let cache = Hashtbl.create 64 in
  let rec f n =
    match Hashtbl.find_opt cache n with
    | Some v -> v
    | None ->
      let v = if n <= 1 then n else f (n-1) + f (n-2) in
      Hashtbl.add cache n v; v
  in f

let () = Printf.printf "fib(40) = %d\n" (fib 40)
