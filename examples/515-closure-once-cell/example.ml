(* Lazy evaluation in OCaml using the built-in lazy keyword *)

(* lazy: defer computation *)
let expensive_value = lazy (
  Printf.printf "[computing expensive value...]\n";
  let sum = ref 0 in
  for i = 1 to 1000000 do sum := !sum + i done;
  !sum
)

(* Lazy fibonacci sequence *)
let make_lazy_fib () =
  let cache = Hashtbl.create 100 in
  let rec fib n =
    match Hashtbl.find_opt cache n with
    | Some v -> v
    | None ->
      let v = if n <= 1 then n else fib (n-1) + fib (n-2) in
      Hashtbl.add cache n v; v
  in
  fib

let () =
  Printf.printf "Before forcing lazy value\n";
  Printf.printf "Value: %d\n" (Lazy.force expensive_value);
  Printf.printf "Value again: %d\n" (Lazy.force expensive_value);  (* cached *)

  let fib = make_lazy_fib () in
  Printf.printf "fib(30) = %d\n" (fib 30);
  Printf.printf "fib(30) = %d (cached)\n" (fib 30)
