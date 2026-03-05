(* Monte Carlo Methods in OCaml *)

(* Simple LCG PRNG returning float in [0, 1) *)
let state = ref 12345678

let rand_float () =
  state := (!state * 1664525 + 1013904223) land 0x7fffffff;
  float_of_int !state /. float_of_int 0x7fffffff

(* Pi estimation: sample n points in unit square, count those in unit circle *)
let estimate_pi (n : int) : float =
  let inside = ref 0 in
  for _ = 1 to n do
    let x = rand_float () and y = rand_float () in
    if x *. x +. y *. y <= 1.0 then incr inside
  done;
  4.0 *. float_of_int !inside /. float_of_int n

(* Monte Carlo integration: estimate ∫_a^b f(x) dx using n samples *)
let mc_integrate (f : float -> float) (a b : float) (n : int) : float =
  let sum = ref 0.0 in
  for _ = 1 to n do
    let x = a +. (b -. a) *. rand_float () in
    sum := !sum +. f x
  done;
  (b -. a) *. !sum /. float_of_int n

(* Monte Carlo: estimate E[X²] where X ~ Uniform[0, 1] *)
(* Exact answer: ∫₀¹ x² dx = 1/3 ≈ 0.333 *)

(* Monte Carlo: accept-reject sampling from a non-trivial distribution *)
(* Sample from f(x) ∝ sin(x) on [0, π] using uniform proposal *)
let sample_sin_distribution (n : int) : float list =
  let results = ref [] in
  let attempts = ref 0 in
  while List.length !results < n && !attempts < 100 * n do
    incr attempts;
    let x = rand_float () *. Float.pi in
    let u = rand_float () in
    if u < sin x then results := x :: !results
  done;
  !results

let () =
  Printf.printf "Pi estimation:\n";
  List.iter (fun n ->
    let pi_est = estimate_pi n in
    Printf.printf "  n=%7d: π ≈ %.6f  error = %.6f\n" n pi_est (abs_float (pi_est -. Float.pi))
  ) [100; 1000; 10000; 100000];

  Printf.printf "\nMC integration:\n";
  Printf.printf "  ∫₀¹ x² dx ≈ %.6f  (exact = 0.333333)\n"
    (mc_integrate (fun x -> x *. x) 0.0 1.0 100000);
  Printf.printf "  ∫₀^π sin(x) dx ≈ %.6f  (exact = 2.0)\n"
    (mc_integrate sin 0.0 Float.pi 100000)
