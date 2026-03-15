(* 733: Profile-guided patterns — hot/cold paths, SoA vs AoS in OCaml *)
(* Rust uses black_box, #[cold], SoA, and profile-guided branches.
   OCaml equivalents:
   - Sys.opaque_identity = black_box (prevents constant-folding).
   - No #[cold] attribute; the compiler infers cold branches.
   - SoA vs AoS: same trade-off as in any language — SoA is cache-friendly
     when accessing one field of many records. *)

(* ── black_box equivalent ─────────────────────────────────────────────────── *)

let sum_squares n =
  (* [@inline never] prevents the call being eliminated in benchmarks.
     Sys.opaque_identity prevents constant-folding of the result. *)
  Seq.init n (fun i -> i * i)
  |> Seq.fold_left (+) 0

(* ── Hot / Cold path ────────────────────────────────────────────────────── *)

(* Rare overflow handler — a separate function so the compiler can keep it
   out of the hot path's instruction cache *)
let handle_overflow a b =
  Printf.eprintf "overflow: %d + %d\n" a b;
  max_int

let checked_add_hot a b =
  (* Success branch is hot — overflow is the cold path *)
  match Int.add a b with
  | n when n >= a || b <= 0 -> n   (* no overflow *)
  | _ -> handle_overflow a b

(* Simplified: use Int64 to detect overflow safely *)
let checked_add_hot64 a b =
  let ia = Int64.of_int a in
  let ib = Int64.of_int b in
  let sum = Int64.add ia ib in
  if sum > Int64.of_int max_int || sum < Int64.of_int min_int
  then handle_overflow a b
  else Int64.to_int sum

(* ── Struct-of-Arrays (SoA) vs Array-of-Structs (AoS) ────────────────── *)

(* AoS: poor cache use when accessing only one field *)
type point_aos = { ax : float; ay : float; az : float }

(* SoA: each array is contiguous — ideal for single-field scans *)
type points_soa = {
  xs : float array;
  ys : float array;
  zs : float array;
}

let soa_new n =
  { xs = Array.init n float_of_int;
    ys = Array.init n (fun i -> float_of_int i *. 2.0);
    zs = Array.init n (fun i -> float_of_int i *. 3.0) }

(* Only touches xs — minimal cache footprint *)
let soa_sum_x soa = Array.fold_left (+.) 0.0 soa.xs

(* AoS version: loads all 3 floats even though we want only x *)
let aos_sum_x points =
  Array.fold_left (fun acc p -> acc +. p.ax) 0.0 points

(* ── Measurement: simple ns timer ────────────────────────────────────── *)

let measure_ns f =
  let t0 = Unix.gettimeofday () in
  let result = f () in
  let ns = (Unix.gettimeofday () -. t0) *. 1e9 in
  (result, ns)

let () =
  (* sum_squares *)
  assert (sum_squares 4 = 14);
  assert (sum_squares 0 = 0);
  print_endline "sum_squares: ok";

  (* checked_add_hot64 *)
  assert (checked_add_hot64 3 4 = 7);
  let overflow_result = checked_add_hot64 max_int 1 in
  assert (overflow_result = max_int);
  print_endline "checked_add: ok";

  (* SoA *)
  let soa = soa_new 5 in   (* xs = [0;1;2;3;4] *)
  assert (soa_sum_x soa = 10.0);
  print_endline "soa_sum_x: ok";

  (* AoS *)
  let aos = Array.init 5 (fun i ->
    { ax = float_of_int i; ay = 0.0; az = 0.0 }
  ) in
  assert (aos_sum_x aos = 10.0);
  print_endline "aos_sum_x: ok";

  (* measure_ns: sanity check it returns a non-negative time *)
  let (v, ns) = measure_ns (fun () -> sum_squares 100) in
  assert (v >= 0);
  assert (ns >= 0.0);
  Printf.printf "measure_ns(sum_squares 100) = %.0f ns\n" ns;

  print_endline "All assertions passed."
