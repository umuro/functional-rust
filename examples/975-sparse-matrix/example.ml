(* 975: Sparse Matrix *)
(* Only store non-zero elements using a hash map *)

module IntPair = struct
  type t = int * int
  let equal (r1, c1) (r2, c2) = r1 = r2 && c1 = c2
  let hash (r, c) = Hashtbl.hash (r, c)
end

module PairHash = Hashtbl.Make(IntPair)

type sparse_matrix = {
  rows: int;
  cols: int;
  data: float PairHash.t;
}

let create rows cols =
  { rows; cols; data = PairHash.create 16 }

let set m r c v =
  if r < 0 || r >= m.rows || c < 0 || c >= m.cols then
    failwith "index out of bounds";
  if v = 0.0 then PairHash.remove m.data (r, c)
  else PairHash.replace m.data (r, c) v

let get m r c =
  PairHash.find_opt m.data (r, c) |> Option.value ~default:0.0

let nnz m = PairHash.length m.data

(* Matrix-vector multiply: result[i] = sum_j m[i,j] * v[j] *)
let matvec m v =
  assert (Array.length v = m.cols);
  let result = Array.make m.rows 0.0 in
  PairHash.iter (fun (r, c) value ->
    result.(r) <- result.(r) +. value *. v.(c)
  ) m.data;
  result

(* Transpose *)
let transpose m =
  let t = create m.cols m.rows in
  PairHash.iter (fun (r, c) v ->
    PairHash.replace t.data (c, r) v
  ) m.data;
  t

(* Add two sparse matrices *)
let add m1 m2 =
  assert (m1.rows = m2.rows && m1.cols = m2.cols);
  let result = create m1.rows m1.cols in
  PairHash.iter (fun k v -> PairHash.replace result.data k v) m1.data;
  PairHash.iter (fun (r, c) v ->
    let existing = PairHash.find_opt result.data (r, c) |> Option.value ~default:0.0 in
    let sum = existing +. v in
    if sum = 0.0 then PairHash.remove result.data (r, c)
    else PairHash.replace result.data (r, c) sum
  ) m2.data;
  result

let () =
  let m = create 4 4 in
  set m 0 0 1.0;
  set m 0 2 2.0;
  set m 1 1 3.0;
  set m 2 0 4.0;
  set m 2 3 5.0;
  set m 3 3 6.0;

  assert (nnz m = 6);
  assert (get m 0 0 = 1.0);
  assert (get m 0 1 = 0.0);  (* zero element *)
  assert (get m 1 1 = 3.0);

  (* Setting to zero removes entry *)
  set m 1 1 0.0;
  assert (nnz m = 5);
  assert (get m 1 1 = 0.0);

  (* Matrix-vector multiply *)
  let v = [| 1.0; 0.0; 1.0; 0.0 |] in
  let result = matvec m v in
  assert (result.(0) = 3.0);  (* 1*1 + 2*1 *)
  assert (result.(1) = 0.0);  (* 3 was removed *)
  assert (result.(2) = 4.0);  (* 4*1 *)

  (* Transpose *)
  let mt = transpose m in
  assert (get mt 0 0 = 1.0);
  assert (get mt 2 0 = 2.0);
  assert (get mt 0 2 = 4.0);
  assert (get mt 3 2 = 5.0);
  assert (get mt 3 3 = 6.0);

  Printf.printf "✓ All tests passed\n"
