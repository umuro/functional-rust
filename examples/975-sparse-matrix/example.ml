(* 975: Sparse Matrix
   Stores only non-zero entries using a hash map keyed by (row, col).
   Supports CSR-like iteration, matrix-vector multiply, and element access.
   OCaml: Hashtbl with (int*int) keys. *)

module IIMap = Map.Make(struct
  type t = int * int
  let compare = compare
end)

type sparse_matrix = {
  rows  : int;
  cols  : int;
  data  : (int * int, float) Hashtbl.t;
}

let create rows cols =
  assert (rows > 0 && cols > 0);
  { rows; cols; data = Hashtbl.create 16 }

let set m r c v =
  assert (r >= 0 && r < m.rows && c >= 0 && c < m.cols);
  if v = 0.0 then Hashtbl.remove m.data (r, c)
  else Hashtbl.replace m.data (r, c) v

let get m r c =
  Option.value (Hashtbl.find_opt m.data (r, c)) ~default:0.0

let nnz m = Hashtbl.length m.data

(* Iterate over non-zero entries in row-major order *)
let iter_nnz f m =
  let entries = Hashtbl.fold (fun (r,c) v acc -> (r,c,v)::acc) m.data [] in
  let sorted = List.sort (fun (r1,c1,_) (r2,c2,_) -> compare (r1,c1) (r2,c2)) entries in
  List.iter (fun (r,c,v) -> f r c v) sorted

(* Dense matrix representation for display *)
let to_dense m =
  let arr = Array.make_matrix m.rows m.cols 0.0 in
  Hashtbl.iter (fun (r,c) v -> arr.(r).(c) <- v) m.data;
  arr

(* Matrix-vector multiply: y = A * x  (x is a dense vector) *)
let matvec m x =
  assert (Array.length x = m.cols);
  let y = Array.make m.rows 0.0 in
  Hashtbl.iter (fun (r,c) v ->
    y.(r) <- y.(r) +. v *. x.(c)
  ) m.data;
  y

(* Sparse matrix addition: C = A + B *)
let add a b =
  assert (a.rows = b.rows && a.cols = b.cols);
  let c = create a.rows a.cols in
  Hashtbl.iter (fun (r,c_) v -> set c r c_ v) a.data;
  Hashtbl.iter (fun (r,c_) v ->
    set c r c_ (get c r c_ +. v)
  ) b.data;
  c

(* Transpose *)
let transpose m =
  let t = create m.cols m.rows in
  Hashtbl.iter (fun (r,c) v -> set t c r v) m.data;
  t

(* Print as coordinate list *)
let print m =
  Printf.printf "SparseMatrix(%dx%d, nnz=%d):\n" m.rows m.cols (nnz m);
  iter_nnz (fun r c v ->
    Printf.printf "  [%d,%d] = %.2f\n" r c v
  ) m

let () =
  (* --- Build a 4x4 sparse matrix --- *)
  let m = create 4 4 in
  set m 0 0 1.0;
  set m 0 3 2.0;
  set m 1 1 3.0;
  set m 2 0 4.0;
  set m 2 2 5.0;
  set m 3 3 6.0;
  print m;

  (* --- Element access --- *)
  Printf.printf "\nget [1,1] = %.1f\n" (get m 1 1);
  Printf.printf "get [0,1] = %.1f  (zero)\n" (get m 0 1);

  (* --- Matrix-vector multiply --- *)
  let x = [|1.0; 1.0; 1.0; 1.0|] in
  let y = matvec m x in
  Printf.printf "\nmatvec [1;1;1;1] = [%s]\n"
    (String.concat "; " (Array.to_list (Array.map (Printf.sprintf "%.1f") y)));

  (* --- Transpose --- *)
  let mt = transpose m in
  Printf.printf "\nTranspose nnz = %d\n" (nnz mt);

  (* --- Delete an entry by setting to 0 --- *)
  set m 0 3 0.0;
  Printf.printf "After remove [0,3]: nnz = %d\n" (nnz m);

  (* --- Sparse identity matrix --- *)
  let identity n =
    let eye = create n n in
    for i = 0 to n-1 do set eye i i 1.0 done;
    eye
  in
  let eye3 = identity 3 in
  let result = matvec eye3 [|5.0; 6.0; 7.0|] in
  Printf.printf "\nI*[5;6;7] = [%s]\n"
    (String.concat "; " (Array.to_list (Array.map (Printf.sprintf "%.1f") result)))
