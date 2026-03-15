(* OCaml: Cache-friendly iteration patterns *)

(* OCaml's `float array` is unboxed (flat), but `float array array`
   is an array of pointers to individual row arrays — two hops per access. *)

(* --- Flat 2D matrix (row-major, cache-friendly) --- *)

type matrix = { data: float array; rows: int; cols: int }

let make_matrix rows cols init =
  { data = Array.init (rows * cols) init; rows; cols }

let get m r c = m.data.(r * m.cols + c)
let set m r c v = m.data.(r * m.cols + c) <- v

(* Row-major sum: sequential access — cache friendly *)
let sum_row_major m =
  Array.fold_left (+.) 0.0 m.data

(* Column-major sum: stride = cols elements between accesses — cache unfriendly *)
let sum_col_major m =
  let acc = ref 0.0 in
  for c = 0 to m.cols - 1 do
    for r = 0 to m.rows - 1 do
      acc := !acc +. get m r c
    done
  done;
  !acc

(* Transpose into a new matrix — writes in row-major order into dst *)
let transpose_naive m =
  let t = make_matrix m.cols m.rows (fun _ -> 0.0) in
  for r = 0 to m.rows - 1 do
    for c = 0 to m.cols - 1 do
      set t c r (get m r c)
    done
  done;
  t

(* Tiled transpose — better cache behaviour for large matrices *)
let transpose_tiled ?(tile=32) m =
  let t = make_matrix m.cols m.rows (fun _ -> 0.0) in
  let r = ref 0 in
  while !r < m.rows do
    let c = ref 0 in
    while !c < m.cols do
      for rr = !r to min (!r + tile - 1) (m.rows - 1) do
        for cc = !c to min (!c + tile - 1) (m.cols - 1) do
          set t cc rr (get m rr cc)
        done
      done;
      c := !c + tile
    done;
    r := !r + tile
  done;
  t

let time_it label f =
  let t0 = Sys.time () in
  let r = f () in
  Printf.printf "%s: %.6fs\n" label (Sys.time () -. t0);
  r

let () =
  let n = 1024 in
  let m = make_matrix n n (fun i -> float_of_int i) in

  let _sr = time_it "Row-major sum" (fun () -> sum_row_major m) in
  let _sc = time_it "Col-major sum" (fun () -> sum_col_major m) in
  let _ = time_it "Naive transpose" (fun () -> transpose_naive m) in
  let _ = time_it "Tiled transpose (tile=32)" (fun () -> transpose_tiled m) in
  Printf.printf "Matrix %dx%d: %d elements\n" n n (n * n)
