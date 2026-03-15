(* 733: Profile-Guided Patterns — OCaml *)

(* OCaml's Sys.opaque_identity is the black_box equivalent *)
let black_box x = Sys.opaque_identity x

(* Array-of-Structs (AoS) — poor cache locality for component-wise ops *)
type point_aos = { x: float; y: float; z: float }

(* Struct-of-Arrays (SoA) — excellent cache locality for SIMD-like ops *)
type points_soa = {
  xs: float array;
  ys: float array;
  zs: float array;
  len: int;
}

let sum_x_aos (points: point_aos array) =
  Array.fold_left (fun acc p -> acc +. p.x) 0.0 points

let sum_x_soa (pts: points_soa) =
  (* Only xs array is touched — perfect for prefetcher *)
  let acc = ref 0.0 in
  for i = 0 to pts.len - 1 do
    acc := !acc +. pts.xs.(i)
  done;
  !acc

let () =
  let n = 1000 in
  let aos = Array.init n (fun i ->
    { x = float_of_int i; y = float_of_int (i*2); z = float_of_int (i*3) }) in
  let soa = {
    xs = Array.init n float_of_int;
    ys = Array.init n (fun i -> float_of_int (i*2));
    zs = Array.init n (fun i -> float_of_int (i*3));
    len = n;
  } in
  Printf.printf "AoS sum_x = %.0f\n" (black_box (sum_x_aos (black_box aos)));
  Printf.printf "SoA sum_x = %.0f\n" (black_box (sum_x_soa (black_box soa)))
