(* 1038: Adjacency Matrix Graph
   Dense representation with O(1) edge lookup.
   Warshall transitive closure in idiomatic OCaml. *)

type matrix_graph = {
  matrix : bool array array;
  size   : int;
}

let make_matrix n =
  { matrix = Array.init n (fun _ -> Array.make n false); size = n }

let add_edge g from_ to_ =
  g.matrix.(from_).(to_) <- true

let has_edge g from_ to_ =
  g.matrix.(from_).(to_)

let neighbors g node =
  Array.to_list (Array.mapi (fun i c -> (i, c)) g.matrix.(node))
  |> List.filter_map (fun (i, c) -> if c then Some i else None)

let out_degree g node =
  Array.fold_left (fun acc c -> if c then acc + 1 else acc) 0 g.matrix.(node)

let in_degree g node =
  let count = ref 0 in
  for i = 0 to g.size - 1 do
    if g.matrix.(i).(node) then incr count
  done;
  !count

(* Warshall transitive closure — O(n^3) *)
let transitive_closure g =
  let n = g.size in
  (* Copy matrix *)
  let tc = Array.init n (fun i -> Array.copy g.matrix.(i)) in
  for k = 0 to n - 1 do
    for i = 0 to n - 1 do
      for j = 0 to n - 1 do
        if tc.(i).(k) && tc.(k).(j) then
          tc.(i).(j) <- true
      done
    done
  done;
  tc

let () =
  let g = make_matrix 4 in
  add_edge g 0 1;
  add_edge g 0 2;
  add_edge g 1 2;
  add_edge g 2 3;

  assert (has_edge g 0 1);
  assert (not (has_edge g 0 3));
  assert (has_edge g 2 3);

  assert (out_degree g 0 = 2);
  assert (out_degree g 2 = 1);
  assert (in_degree g 2 = 2);
  assert (neighbors g 0 = [1; 2]);

  (* Transitive closure *)
  let tc = transitive_closure g in
  assert (not (has_edge g 0 3));  (* no direct edge *)
  assert (tc.(0).(3));            (* reachable transitively *)
  assert (tc.(0).(2));
  assert (tc.(1).(3));

  (* Undirected: add both directions *)
  let ug = make_matrix 3 in
  add_edge ug 0 1;
  add_edge ug 1 0;
  assert (has_edge ug 0 1);
  assert (has_edge ug 1 0);

  (* Self-loop *)
  let sg = make_matrix 3 in
  add_edge sg 0 0;
  assert (has_edge sg 0 0);
  assert (out_degree sg 0 = 1);

  Printf.printf "All adjacency-matrix tests passed.\n"
