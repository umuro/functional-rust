(* 753: Bench Harness — OCaml *)

let time_fn f =
  let t0 = Unix.gettimeofday () in
  f ();
  let t1 = Unix.gettimeofday () in
  (t1 -. t0) *. 1e9  (* nanoseconds *)

let bench_function ?(iters=10_000) name f =
  (* warmup *)
  for _ = 1 to 100 do ignore (f ()) done;
  (* measure *)
  let samples = Array.init iters (fun _ -> time_fn (fun () -> ignore (f ()))) in
  Array.sort compare samples;
  let n = float_of_int iters in
  let mean = Array.fold_left (+.) 0.0 samples /. n in
  let p50 = samples.(iters / 2) in
  let p90 = samples.(int_of_float (n *. 0.9)) in
  let p99 = samples.(int_of_float (n *. 0.99)) in
  Printf.printf "%-30s mean=%.1fns p50=%.1fns p90=%.1fns p99=%.1fns\n"
    name mean p50 p90 p99

(* Functions to benchmark *)
let sum_recursive n =
  let rec go acc i =
    if i > n then acc else go (acc + i) (i + 1)
  in go 0 0

let sum_formula n = n * (n + 1) / 2

let () =
  bench_function "sum_recursive(100)" (fun () -> sum_recursive 100);
  bench_function "sum_formula(100)"   (fun () -> sum_formula 100)
