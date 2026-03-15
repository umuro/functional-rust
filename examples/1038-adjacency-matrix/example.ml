(* 1038: Adjacency Matrix — Dense Graph Operations *)
(* 2D boolean array for graph connectivity *)

(* Approach 1: Basic adjacency matrix *)
let basic_matrix () =
  let n = 4 in
  let matrix = Array.make_matrix n n false in
  (* Add edges: 0->1, 0->2, 1->2, 2->3 *)
  matrix.(0).(1) <- true;
  matrix.(0).(2) <- true;
  matrix.(1).(2) <- true;
  matrix.(2).(3) <- true;
  assert (matrix.(0).(1) = true);
  assert (matrix.(0).(3) = false);
  assert (matrix.(2).(3) = true)

(* Approach 2: Degree counting and neighbor listing *)
let degree_and_neighbors () =
  let n = 4 in
  let matrix = Array.make_matrix n n false in
  matrix.(0).(1) <- true;
  matrix.(0).(2) <- true;
  matrix.(1).(2) <- true;
  matrix.(2).(3) <- true;
  (* Out-degree of node 0 *)
  let out_degree node =
    Array.fold_left (fun acc connected -> if connected then acc + 1 else acc)
      0 matrix.(node)
  in
  assert (out_degree 0 = 2);
  assert (out_degree 2 = 1);
  (* List neighbors *)
  let neighbors node =
    Array.to_list matrix.(node)
    |> List.mapi (fun i c -> (i, c))
    |> List.filter_map (fun (i, c) -> if c then Some i else None)
  in
  assert (neighbors 0 = [1; 2])

(* Approach 3: Transitive closure (Warshall's algorithm) *)
let transitive_closure () =
  let n = 4 in
  let m = Array.make_matrix n n false in
  m.(0).(1) <- true;
  m.(1).(2) <- true;
  m.(2).(3) <- true;
  (* Copy matrix *)
  let tc = Array.init n (fun i -> Array.copy m.(i)) in
  (* Warshall's algorithm *)
  for k = 0 to n - 1 do
    for i = 0 to n - 1 do
      for j = 0 to n - 1 do
        if tc.(i).(k) && tc.(k).(j) then
          tc.(i).(j) <- true
      done
    done
  done;
  (* Now 0 can reach 3 transitively *)
  assert (not m.(0).(3));  (* Direct: no *)
  assert (tc.(0).(3));     (* Transitive: yes *)
  assert (tc.(0).(2));     (* 0 -> 1 -> 2 *)

let () =
  basic_matrix ();
  degree_and_neighbors ();
  transitive_closure ();
  Printf.printf "✓ All tests passed\n"
