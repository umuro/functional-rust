(* OCaml: SoA vs AoS for cache efficiency *)

(* --- Array of Structures (AoS) --- *)
type particle = {
  x    : float;
  y    : float;
  z    : float;
  mass : float;
}

(* Sum just the x-coordinates: must traverse all fields even though we skip y/z/mass *)
let sum_x_aos particles =
  Array.fold_left (fun acc p -> acc +. p.x) 0.0 particles

(* --- Structure of Arrays (SoA) --- *)
type particles_soa = {
  xs    : float array;
  ys    : float array;
  zs    : float array;
  masses: float array;
}

(* Sum just x: touches ONLY xs — contiguous in memory *)
let sum_x_soa soa =
  Array.fold_left (+.) 0.0 soa.xs

(* Gravity update: needs x, y, mass — still more cache-friendly than AoS *)
let apply_gravity_soa soa dt =
  Array.iteri (fun i m ->
    soa.ys.(i) <- soa.ys.(i) -. 9.81 *. m *. dt
  ) soa.masses

let make_aos n =
  Array.init n (fun i ->
    { x = float_of_int i; y = float_of_int (i * 2);
      z = 0.0; mass = 1.0 })

let make_soa n =
  { xs     = Array.init n float_of_int;
    ys     = Array.init n (fun i -> float_of_int (i * 2));
    zs     = Array.make n 0.0;
    masses = Array.make n 1.0 }

let time_it label f =
  let t0 = Sys.time () in
  let result = f () in
  let t1 = Sys.time () in
  Printf.printf "%s: result=%.2f  time=%.6fs\n" label result (t1 -. t0)

let () =
  let n = 1_000_000 in
  let aos = make_aos n in
  let soa = make_soa n in

  time_it "AoS sum_x" (fun () -> sum_x_aos aos);
  time_it "SoA sum_x" (fun () -> sum_x_soa soa);

  (* OCaml note: records are heap-allocated objects with GC overhead;
     floats in float arrays are unboxed (OCaml optimises float arrays).
     So float array SoA in OCaml is actually already cache-friendly! *)
  Printf.printf "Note: OCaml float arrays are unboxed — SoA advantage is native here.\n"
