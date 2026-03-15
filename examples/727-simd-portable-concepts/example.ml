(* OCaml: SIMD concepts via scalar simulation and Bigarray
   OCaml doesn't have SIMD intrinsics in the standard library.
   We demonstrate the vectorised mental model using Bigarray and
   show what the equivalent SIMD code achieves. *)

open Bigarray

type f32vec = (float, float32_elt, c_layout) Array1.t

let make_vec n = Array1.create float32 c_layout n

(* --- Simulated SIMD operations (scalar fallback) --- *)

(* Vectorised addition: element-wise a[i] + b[i] *)
let vec_add (a : f32vec) (b : f32vec) : f32vec =
  let n = Array1.dim a in
  let c = make_vec n in
  for i = 0 to n - 1 do
    c.{i} <- a.{i} +. b.{i}
  done;
  c

(* Vectorised multiply-accumulate (FMA pattern) *)
let vec_fma (a : f32vec) (b : f32vec) (c : f32vec) : f32vec =
  let n = Array1.dim a in
  let result = make_vec n in
  for i = 0 to n - 1 do
    result.{i} <- a.{i} *. b.{i} +. c.{i}
  done;
  result

(* Horizontal sum (reduction) *)
let vec_sum (a : f32vec) : float =
  let acc = ref 0.0 in
  for i = 0 to Array1.dim a - 1 do
    acc := !acc +. a.{i}
  done;
  !acc

(* Dot product via vec_fma pattern *)
let dot_product (a : f32vec) (b : f32vec) : float =
  let n = Array1.dim a in
  let acc = ref 0.0 in
  for i = 0 to n - 1 do
    acc := !acc +. a.{i} *. b.{i}
  done;
  !acc

(* Conditional select: select a[i] if mask[i] > 0, else b[i] *)
let vec_select (mask : f32vec) (a : f32vec) (b : f32vec) : f32vec =
  let n = Array1.dim a in
  let r = make_vec n in
  for i = 0 to n - 1 do
    r.{i} <- if mask.{i} > 0.0 then a.{i} else b.{i}
  done;
  r

(* --- Demo --- *)

let fill_vec n f =
  let v = make_vec n in
  for i = 0 to n - 1 do v.{i} <- f i done;
  v

let () =
  let n = 8 in
  let a = fill_vec n (fun i -> float_of_int (i + 1)) in
  let b = fill_vec n (fun i -> float_of_int (n - i)) in

  Printf.printf "a = [%s]\n"
    (String.concat "; " (List.init n (fun i -> Printf.sprintf "%.0f" a.{i})));
  Printf.printf "b = [%s]\n"
    (String.concat "; " (List.init n (fun i -> Printf.sprintf "%.0f" b.{i})));

  let s = vec_add a b in
  Printf.printf "a+b = [%s]\n"
    (String.concat "; " (List.init n (fun i -> Printf.sprintf "%.0f" s.{i})));

  Printf.printf "dot(a,b) = %.1f\n" (dot_product a b);
  Printf.printf "sum(a)   = %.1f\n" (vec_sum a);

  let mask = fill_vec n (fun i -> if i < 4 then 1.0 else -1.0) in
  let ones  = fill_vec n (fun _ -> 1.0) in
  let zeros = fill_vec n (fun _ -> 0.0) in
  let sel = vec_select mask ones zeros in
  Printf.printf "select = [%s]\n"
    (String.concat "; " (List.init n (fun i -> Printf.sprintf "%.0f" sel.{i})));

  (* Note: in real high-perf OCaml, you'd call C SIMD kernels via FFI.
     Owl/owl-base wraps BLAS/LAPACK which uses SIMD internally. *)
  Printf.printf "OCaml SIMD note: use owl-base or C FFI for native SIMD.\n"
