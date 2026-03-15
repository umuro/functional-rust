(* 732: Benchmarking harness — Criterion-style, stdlib-only in OCaml *)
(* Rust uses Criterion or a manual black_box + Instant loop.
   In OCaml:
   - Use [Sys.time()] or [Unix.gettimeofday()] for wall-clock measurements.
   - Use [Sys.opaque_identity] as the OCaml equivalent of std::hint::black_box.
   - The [bechamel] library is the idiomatic OCaml benchmarking framework.
   - This example uses only the stdlib to mirror the Rust std-only approach. *)

type bench_result = {
  label     : string;
  mean_ns   : float;
  min_ns    : float;
  max_ns    : float;
  stddev_ns : float;
  iters     : int;
}

let bench label ~warmup ~iters f =
  (* Warmup — fill caches, let CPU ramp up *)
  for _ = 1 to warmup do
    ignore (Sys.opaque_identity (f ()))
  done;
  let samples = Array.make iters 0.0 in
  for i = 0 to iters - 1 do
    let t0 = Unix.gettimeofday () in
    let result = f () in
    let elapsed = (Unix.gettimeofday () -. t0) *. 1e9 in
    ignore (Sys.opaque_identity result);
    samples.(i) <- elapsed
  done;
  let total = Array.fold_left (+.) 0.0 samples in
  let mean  = total /. float_of_int iters in
  let mn    = Array.fold_left Float.min infinity samples in
  let mx    = Array.fold_left Float.max neg_infinity samples in
  let variance =
    Array.fold_left (fun acc x ->
      let d = x -. mean in acc +. d *. d
    ) 0.0 samples /. float_of_int iters
  in
  { label; mean_ns = mean; min_ns = mn; max_ns = mx;
    stddev_ns = Float.sqrt variance; iters }

let print_result r =
  Printf.printf "%-40s mean=%8.1fns  min=%8.1fns  max=%8.1fns  σ=%.0fns  (n=%d)\n"
    r.label r.mean_ns r.min_ns r.max_ns r.stddev_ns r.iters

(* ── Functions to benchmark ──────────────────────────────────────────────── *)

let sum_naive n =
  let s = ref 0 in
  for i = 0 to n - 1 do s := !s + i done;
  !s

let sum_formula n = n * (n - 1) / 2

let string_push n =
  let buf = Buffer.create n in
  for _ = 1 to n do Buffer.add_char buf 'x' done;
  Buffer.contents buf

let vec_collect n = Array.init n (fun i -> i)

let () =
  (* Functional tests *)
  assert (sum_naive 5 = 10);
  assert (sum_naive 0 = 0);
  print_endline "sum_naive: ok";

  for n = 1 to 20 do
    assert (sum_naive n = sum_formula n)
  done;
  print_endline "sum_formula matches naive: ok";

  (* Benchmark runs *)
  let r1 = bench "sum_naive(1000)"  ~warmup:5 ~iters:20
    (fun () -> sum_naive (Sys.opaque_identity 1000)) in
  assert (r1.iters = 20);
  assert (r1.min_ns <= r1.mean_ns);
  assert (r1.mean_ns <= r1.max_ns);
  print_result r1;

  let r2 = bench "sum_formula(1000)" ~warmup:5 ~iters:20
    (fun () -> sum_formula (Sys.opaque_identity 1000)) in
  print_result r2;

  let _r3 = bench "string_push(100)" ~warmup:3 ~iters:10
    (fun () -> string_push (Sys.opaque_identity 100)) in

  let _r4 = bench "vec_collect(100)" ~warmup:3 ~iters:10
    (fun () -> vec_collect (Sys.opaque_identity 100)) in

  print_endline "All assertions passed."
