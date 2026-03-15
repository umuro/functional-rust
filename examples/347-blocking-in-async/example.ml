(* OCaml: compute vs I/O separation *)

let cpu_bound n =
  (* Fibonacci: CPU heavy *)
  let rec fib = function
    | 0 | 1 -> 1
    | n -> fib (n-1) + fib (n-2)
  in fib n

let blocking_io label =
  Thread.delay 0.02;  (* simulate blocking I/O *)
  Printf.sprintf "result from %s" label

let run_mixed tasks =
  let handles = List.map (fun f -> Thread.create f ()) tasks in
  let results = ref [] in
  List.iter (fun t ->
    (* Thread.join returns unit in OCaml *)
    Thread.join t
  ) handles;
  !results

let () =
  let start = Unix.gettimeofday () in
  let _ = run_mixed [
    (fun () -> Printf.printf "Fib(35)=%d\n" (cpu_bound 35));
    (fun () -> Printf.printf "IO1: %s\n" (blocking_io "source1"));
    (fun () -> Printf.printf "IO2: %s\n" (blocking_io "source2"));
  ] in
  Printf.printf "Elapsed: %.3fs\n" (Unix.gettimeofday () -. start)
