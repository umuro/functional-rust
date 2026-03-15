(* 727: SIMD portable concepts — lane-wise operations in OCaml *)
(* Rust's portable_simd provides f32x8, u32x8, etc. processed in SIMD registers.
   OCaml does not have a built-in SIMD API (as of OCaml 5.2).
   The idiomatic OCaml equivalent is:
   - Use the `owl` numerical library for vectorised float arrays in production.
   - For direct SIMD: use C stubs via the FFI.
   - For pedagogical purposes: process arrays in chunks of LANES — the same
     "data parallel" mental model, without the HW register hint.
   The ocaml-simd library (experimental, OCaml 5.3+) will expose Vec128/Vec256.

   Here we demonstrate the same algorithmic patterns using float arrays. *)

let lanes = 8

(* Simulated f32x8: an array of exactly LANES floats *)
type f32x8 = float array  (* always length = lanes *)

let splat v : f32x8 = Array.make lanes v

let from_array (a : float array) : f32x8 =
  assert (Array.length a = lanes);
  Array.copy a

let to_array (v : f32x8) = Array.copy v

(* Lane-wise operations — these map directly to AVX2 VADDPS / VMULPS etc. *)
let add (a : f32x8) (b : f32x8) : f32x8 =
  Array.init lanes (fun i -> a.(i) +. b.(i))

let mul (a : f32x8) (b : f32x8) : f32x8 =
  Array.init lanes (fun i -> a.(i) *. b.(i))

(* Fused multiply-add: a * b + c *)
let mul_add (a : f32x8) (b : f32x8) (c : f32x8) : f32x8 =
  Array.init lanes (fun i -> a.(i) *. b.(i) +. c.(i))

let reduce_sum (v : f32x8) = Array.fold_left (+.) 0.0 v

let vmax (a : f32x8) (b : f32x8) : f32x8 =
  Array.init lanes (fun i -> Float.max a.(i) b.(i))

let vmin (a : f32x8) (b : f32x8) : f32x8 =
  Array.init lanes (fun i -> Float.min a.(i) b.(i))

let gt (a : f32x8) (b : f32x8) : bool array =
  Array.init lanes (fun i -> a.(i) > b.(i))

let vselect (mask : bool array) (on_true : f32x8) (on_false : f32x8) : f32x8 =
  Array.init lanes (fun i -> if mask.(i) then on_true.(i) else on_false.(i))

(* Dot product using 8-wide chunks *)
let dot_product_simd a b =
  assert (Array.length a = Array.length b);
  let n = Array.length a in
  let full = n / lanes in
  let acc = ref (splat 0.0) in
  for i = 0 to full - 1 do
    let off = i * lanes in
    let va = from_array (Array.sub a off lanes) in
    let vb = from_array (Array.sub b off lanes) in
    acc := mul_add va vb !acc
  done;
  let result = ref (reduce_sum !acc) in
  for i = full * lanes to n - 1 do
    result := !result +. a.(i) *. b.(i)
  done;
  !result

(* Element-wise clamp *)
let clamp_simd data lo hi =
  let vlo = splat lo and vhi = splat hi in
  let n = Array.length data in
  let full = n / lanes in
  for i = 0 to full - 1 do
    let off = i * lanes in
    let chunk = from_array (Array.sub data off lanes) in
    let clamped = vmin (vmax chunk vlo) vhi in
    Array.blit clamped 0 data off lanes
  done;
  for i = full * lanes to n - 1 do
    data.(i) <- Float.min (Float.max data.(i) lo) hi
  done

let relu_simd data = clamp_simd data 0.0 infinity

let sum_simd data =
  let n = Array.length data in
  let full = n / lanes in
  let acc = ref (splat 0.0) in
  for i = 0 to full - 1 do
    let off = i * lanes in
    let v = from_array (Array.sub data off lanes) in
    acc := add !acc v
  done;
  let result = ref (reduce_sum !acc) in
  for i = full * lanes to n - 1 do
    result := !result +. data.(i)
  done;
  !result

let dot_scalar a b = Array.fold_left2 (fun s x y -> s +. x *. y) 0.0 a b

let () =
  (* lane add *)
  let a = splat 2.0 and b = splat 3.0 in
  assert (add a b = Array.make lanes 5.0);
  print_endline "lane_add: ok";

  (* reduce_sum *)
  let v = from_array [|1.;2.;3.;4.;5.;6.;7.;8.|] in
  assert (reduce_sum v = 36.0);
  print_endline "reduce_sum: ok";

  (* dot product matches scalar *)
  let a64 = Array.init 64 (fun i -> float_of_int i) in
  let b64 = Array.init 64 (fun i -> float_of_int (64 - i)) in
  let d_simd = dot_product_simd a64 b64 in
  let d_scalar = dot_scalar a64 b64 in
  assert (Float.abs (d_simd -. d_scalar) < 0.01);
  print_endline "dot_product: ok";

  (* relu zeroes negatives *)
  let v2 = [|-2.;-1.;0.;1.;2.;-3.;4.;-0.5|] in
  relu_simd v2;
  assert (v2 = [|0.;0.;0.;1.;2.;0.;4.;0.|]);
  print_endline "relu: ok";

  (* sum *)
  let data16 = Array.init 16 (fun i -> float_of_int (i + 1)) in
  assert (sum_simd data16 = 136.0);
  print_endline "sum_simd: ok";

  (* mask select *)
  let a2 = from_array [|1.;2.;3.;4.;5.;6.;7.;8.|] in
  let threshold = splat 4.5 in
  let mask = gt a2 threshold in
  let selected = vselect mask (splat 1.0) (splat 0.0) in
  assert (Array.sub selected 0 4 = [|0.;0.;0.;0.|]);
  assert (Array.sub selected 4 4 = [|1.;1.;1.;1.|]);
  print_endline "mask_select: ok";

  print_endline "All assertions passed."
