(* 976: Matrix Multiplication *)
(* Naive O(n^3) multiplication. Note: Strassen is O(n^2.807) *)
(* OCaml: list of lists (functional style) and array of arrays *)

(* Approach 1: List-of-lists (functional, educational) *)

let mat_rows m = List.length m
let mat_cols m = match m with [] -> 0 | row :: _ -> List.length row

let dot_product xs ys =
  List.fold_left2 (fun acc x y -> acc +. x *. y) 0.0 xs ys

let transpose_list m =
  let cols = mat_cols m in
  List.init cols (fun c ->
    List.map (fun row -> List.nth row c) m
  )

let multiply_lists a b =
  let bt = transpose_list b in
  List.map (fun row_a ->
    List.map (fun col_b ->
      dot_product row_a col_b
    ) bt
  ) a

(* Approach 2: Array-of-arrays (imperative, practical) *)

let mat_multiply a b =
  let n = Array.length a in
  let m = Array.length b.(0) in
  let k = Array.length b in
  let result = Array.make_matrix n m 0.0 in
  for i = 0 to n - 1 do
    for j = 0 to m - 1 do
      let s = ref 0.0 in
      for l = 0 to k - 1 do
        s := !s +. a.(i).(l) *. b.(l).(j)
      done;
      result.(i).(j) <- !s
    done
  done;
  result

(* Strassen is O(n^2.807): divide 2x2 blocks, use 7 multiplications instead of 8 *)
(* For n x n matrices: split into n/2 x n/2 blocks, apply recursively *)
(* Real implementation requires padding to power-of-2 sizes *)
(* Here we implement for 2x2 as a demonstration *)

let strassen_2x2 a b =
  let a11 = a.(0).(0) and a12 = a.(0).(1) in
  let a21 = a.(1).(0) and a22 = a.(1).(1) in
  let b11 = b.(0).(0) and b12 = b.(0).(1) in
  let b21 = b.(1).(0) and b22 = b.(1).(1) in
  let m1 = (a11 +. a22) *. (b11 +. b22) in
  let m2 = (a21 +. a22) *. b11 in
  let m3 = a11 *. (b12 -. b22) in
  let m4 = a22 *. (b21 -. b11) in
  let m5 = (a11 +. a12) *. b22 in
  let m6 = (a21 -. a11) *. (b11 +. b12) in
  let m7 = (a12 -. a22) *. (b21 +. b22) in
  [| [| m1 +. m4 -. m5 +. m7;  m3 +. m5 |];
     [| m2 +. m4;               m1 -. m2 +. m3 +. m6 |] |]

let () =
  (* List multiplication *)
  let a = [[1.0; 2.0]; [3.0; 4.0]] in
  let b = [[5.0; 6.0]; [7.0; 8.0]] in
  let c = multiply_lists a b in
  assert (c = [[19.0; 22.0]; [43.0; 50.0]]);

  (* Array multiplication *)
  let a_arr = [| [| 1.0; 2.0 |]; [| 3.0; 4.0 |] |] in
  let b_arr = [| [| 5.0; 6.0 |]; [| 7.0; 8.0 |] |] in
  let c_arr = mat_multiply a_arr b_arr in
  assert (c_arr.(0).(0) = 19.0);
  assert (c_arr.(0).(1) = 22.0);
  assert (c_arr.(1).(0) = 43.0);
  assert (c_arr.(1).(1) = 50.0);

  (* Strassen 2x2 *)
  let c_s = strassen_2x2 a_arr b_arr in
  assert (c_s.(0).(0) = 19.0);
  assert (c_s.(0).(1) = 22.0);
  assert (c_s.(1).(0) = 43.0);
  assert (c_s.(1).(1) = 50.0);

  (* Non-square: 2x3 * 3x2 *)
  let m23 = [| [| 1.0; 2.0; 3.0 |]; [| 4.0; 5.0; 6.0 |] |] in
  let m32 = [| [| 7.0; 8.0 |]; [| 9.0; 10.0 |]; [| 11.0; 12.0 |] |] in
  let result = mat_multiply m23 m32 in
  assert (result.(0).(0) = 58.0);
  assert (result.(0).(1) = 64.0);
  assert (result.(1).(0) = 139.0);
  assert (result.(1).(1) = 154.0);

  Printf.printf "✓ All tests passed\n"
