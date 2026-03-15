(* 942: Matrix Operations — Functional 2D

   Matrix transpose and multiply using nested lists (or arrays).
   OCaml: purely functional using List.map + List.init, or imperative using Array. *)

(* ── Type alias ──────────────────────────────────────────────────────────── *)

type matrix = int array array   (* row-major: matrix.(row).(col) *)

(* ── Construction ────────────────────────────────────────────────────────── *)

let rows m = Array.length m
let cols m = if rows m = 0 then 0 else Array.length m.(0)

let make_matrix r c init = Array.init r (fun _ -> Array.make c init)

(* ── Transpose ───────────────────────────────────────────────────────────── *)

let transpose m =
  let r = rows m and c = cols m in
  if r = 0 || c = 0 then [||]
  else Array.init c (fun j -> Array.init r (fun i -> m.(i).(j)))

(* ── Dot product ─────────────────────────────────────────────────────────── *)

let dot a b =
  Array.fold_left ( + ) 0
    (Array.init (Array.length a) (fun i -> a.(i) * b.(i)))

(* ── Matrix multiplication: A(m×n) × B(n×p) = C(m×p) ───────────────────── *)

let multiply a b =
  let bt = transpose b in
  Array.map (fun row ->
    Array.map (fun col -> dot row col) bt
  ) a

(* ── Scalar multiplication ───────────────────────────────────────────────── *)

let scale scalar m =
  Array.map (Array.map (fun x -> x * scalar)) m

(* ── Map over all elements ───────────────────────────────────────────────── *)

let map_matrix f m = Array.map (Array.map f) m

(* ── Functional (list-based) version ─────────────────────────────────────── *)

let transpose_lists m =
  match m with
  | [] -> []
  | [] :: _ -> []
  | _ ->
    let n_cols = List.length (List.hd m) in
    List.init n_cols (fun j ->
      List.map (fun row -> List.nth row j) m)

(* ── Matrix equality ─────────────────────────────────────────────────────── *)

let equal_matrix a b =
  rows a = rows b && cols a = cols b &&
  Array.for_all2 (fun ra rb ->
    Array.for_all2 ( = ) ra rb
  ) a b

(* ── Helpers ─────────────────────────────────────────────────────────────── *)

let of_lists xss =
  Array.of_list (List.map Array.of_list xss)

let to_lists m =
  Array.to_list (Array.map Array.to_list m)

let () =
  (* transpose *)
  let m1 = of_lists [[1;2;3]; [4;5;6]] in
  let t1 = transpose m1 in
  assert (to_lists t1 = [[1;4]; [2;5]; [3;6]]);

  (* empty *)
  assert (transpose [||] = [||]);

  (* dot product *)
  assert (dot [|1;2;3|] [|4;5;6|] = 32);

  (* multiply *)
  let a = of_lists [[1;2;3]; [4;5;6]] in
  let b = of_lists [[7;8]; [9;10]; [11;12]] in
  let c = multiply a b in
  assert (to_lists c = [[58;64]; [139;154]]);

  (* scale *)
  let m2 = of_lists [[1;2]; [3;4]] in
  let m3 = scale 3 m2 in
  assert (to_lists m3 = [[3;6]; [9;12]]);

  (* identity multiply *)
  let identity = of_lists [[1;0]; [0;1]] in
  assert (equal_matrix (multiply m2 identity) m2);

  (* list-based transpose *)
  assert (transpose_lists [[1;2;3]; [4;5;6]] = [[1;4]; [2;5]; [3;6]]);
  assert (transpose_lists [] = []);

  (* map_matrix *)
  let doubled = map_matrix (fun x -> x * 2) m2 in
  assert (to_lists doubled = [[2;4]; [6;8]]);

  print_endline "942-matrix-operations: all tests passed"
