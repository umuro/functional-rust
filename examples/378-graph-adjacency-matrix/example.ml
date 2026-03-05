(* Graph as adjacency matrix in OCaml *)

type graph = {
  vertices: int;
  matrix: bool array array;
}

let create_graph n =
  { vertices = n; matrix = Array.make_matrix n n false }

let add_edge g u v =
  g.matrix.(u).(v) <- true;
  g.matrix.(v).(u) <- true  (* undirected *)

let has_edge g u v = g.matrix.(u).(v)

let neighbors g u =
  Array.to_list (Array.init g.vertices (fun v ->
    if g.matrix.(u).(v) then Some v else None))
  |> List.filter_map (fun x -> x)

let print_matrix g =
  Printf.printf "  ";
  for i = 0 to g.vertices - 1 do
    Printf.printf "%d " i
  done;
  print_newline ();
  for i = 0 to g.vertices - 1 do
    Printf.printf "%d " i;
    for j = 0 to g.vertices - 1 do
      Printf.printf "%s " (if g.matrix.(i).(j) then "1" else "0")
    done;
    print_newline ()
  done

let degree g u =
  List.length (neighbors g u)

let () =
  let g = create_graph 5 in
  add_edge g 0 1;
  add_edge g 0 2;
  add_edge g 1 3;
  add_edge g 2 3;
  add_edge g 3 4;
  Printf.printf "Graph adjacency matrix (5 vertices):\n";
  print_matrix g;
  Printf.printf "\nNeighbors of vertex 3: ";
  List.iter (fun v -> Printf.printf "%d " v) (neighbors g 3);
  print_newline ();
  Printf.printf "Has edge 0-1: %b\n" (has_edge g 0 1);
  Printf.printf "Has edge 0-4: %b\n" (has_edge g 0 4);
  Printf.printf "Degree of vertex 3: %d\n" (degree g 3)
