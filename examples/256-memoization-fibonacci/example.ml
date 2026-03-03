(* Example 256: Memoization — Fibonacci with Hashtable Cache *)

(* Generic memoize wrapper: wraps any function with a Hashtbl cache *)
let memoize f =
  let cache = Hashtbl.create 16 in
  fun x ->
    match Hashtbl.find_opt cache x with
    | Some v -> v
    | None ->
      let v = f x in
      Hashtbl.add cache x v;
      v

(* Recursive Fibonacci memoized via mutual recursion with memoize *)
let fib =
  let rec fib' n =
    if n <= 1 then n
    else memo_fib (n - 1) + memo_fib (n - 2)
  and memo_fib = memoize fib'
  in memo_fib

let () =
  assert (fib 0 = 0);
  assert (fib 1 = 1);
  assert (fib 10 = 55);
  assert (fib 35 = 9227465);
  Printf.printf "fib(35) = %d\n" (fib 35);
  print_endline "ok"
