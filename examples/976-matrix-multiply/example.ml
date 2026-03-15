(* 976: Matrix Multiply
   Demonstrates naive O(n^3) multiplication, cache-friendly tiling,
   and transpose-before-multiply (better cache locality).
   OCaml: Bigarray for layout control, plain Array.make_matrix for clarity. *)

(* --- Plain row-major 2D array representation --- *)
type matrix = {
  rows : int;
  cols : int;
  data : float array;  (* row-major: data.(r*cols + c) *)
}

let create rows cols init =
  { rows; cols; data = Array.make (rows * cols) init }

let get m r c = m.data.(r * m.cols + c)
let set m r c v = m.data.(r * m.cols + c) <- v

let of_array_matrix arr =
  let rows = Array.length arr in
  let cols = Array.length arr.(0) in
  let m = create rows cols 0.0 in
  for r = 0 to rows-1 do
    for c = 0 to cols-1 do
      set m r c arr.(r).(c)
    done
  done;
  m

let to_array_matrix m =
  Array.init m.rows (fun r ->
    Array.init m.cols (fun c -> get m r c))

(* --- Naive O(n^3) multiplication --- *)
let matmul a b =
  assert (a.cols = b.rows);
  let c = create a.rows b.cols 0.0 in
  for i = 0 to a.rows - 1 do
    for j = 0 to b.cols - 1 do
      let s = ref 0.0 in
      for k = 0 to a.cols - 1 do
        s := !s +. get a i k *. get b k j
      done;
      set c i j !s
    done
  done;
  c

(* --- Transpose-then-multiply: improves cache locality ---
   Instead of striding through b by column, transpose b so both a and bt
   are accessed row-by-row. *)
let transpose m =
  let t = create m.cols m.rows 0.0 in
  for r = 0 to m.rows - 1 do
    for c = 0 to m.cols - 1 do
      set t c r (get m r c)
    done
  done;
  t

let matmul_transposed a b =
  assert (a.cols = b.rows);
  let bt = transpose b in
  let c = create a.rows b.cols 0.0 in
  for i = 0 to a.rows - 1 do
    for j = 0 to b.cols - 1 do
      let s = ref 0.0 in
      for k = 0 to a.cols - 1 do
        (* Both get a i k and get bt j k are row-sequential *)
        s := !s +. get a i k *. get bt j k
      done;
      set c i j !s
    done
  done;
  c

(* --- Tiled multiplication for better cache behavior --- *)
let matmul_tiled ?(tile=32) a b =
  assert (a.cols = b.rows);
  let c = create a.rows b.cols 0.0 in
  let n = a.rows and m = b.cols and p = a.cols in
  let i = ref 0 in
  while !i < n do
    let j = ref 0 in
    while !j < m do
      let k = ref 0 in
      while !k < p do
        (* Inner tile: i..i+tile, j..j+tile, k..k+tile *)
        for ii = !i to min (n-1) (!i+tile-1) do
          for kk = !k to min (p-1) (!k+tile-1) do
            let a_ik = get a ii kk in
            for jj = !j to min (m-1) (!j+tile-1) do
              set c ii jj (get c ii jj +. a_ik *. get b kk jj)
            done
          done
        done;
        k := !k + tile
      done;
      j := !j + tile
    done;
    i := !i + tile
  done;
  c

let print_matrix label m =
  Printf.printf "%s (%dx%d):\n" label m.rows m.cols;
  for r = 0 to m.rows - 1 do
    for c = 0 to m.cols - 1 do
      Printf.printf "  %6.1f" (get m r c)
    done;
    print_newline ()
  done

let () =
  let a = of_array_matrix [|
    [|1.0; 2.0; 3.0|];
    [|4.0; 5.0; 6.0|];
  |] in
  let b = of_array_matrix [|
    [|7.0; 8.0|];
    [|9.0; 10.0|];
    [|11.0; 12.0|];
  |] in

  let c1 = matmul a b in
  let c2 = matmul_transposed a b in
  let c3 = matmul_tiled a b in

  print_matrix "A" a;
  print_matrix "B" b;
  print_matrix "C = A*B (naive)" c1;
  print_matrix "C = A*B (transpose)" c2;
  print_matrix "C = A*B (tiled)" c3;

  (* Verify all three give same result *)
  let same m1 m2 =
    Array.for_all2 (fun x y -> abs_float (x -. y) < 1e-10)
      m1.data m2.data
  in
  Printf.printf "\nnaive == transposed: %b\n" (same c1 c2);
  Printf.printf "naive == tiled:      %b\n" (same c1 c3);

  (* Identity matrix multiplication *)
  let eye3 = create 3 3 0.0 in
  for i = 0 to 2 do set eye3 i i 1.0 done;
  let ab = of_array_matrix [|[|1.0;2.0;3.0|];[|4.0;5.0;6.0|];[|7.0;8.0;9.0|]|] in
  let result = matmul ab eye3 in
  Printf.printf "\nA * I == A: %b\n" (same result ab)
