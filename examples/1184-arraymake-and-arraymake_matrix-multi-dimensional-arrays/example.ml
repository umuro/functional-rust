(* Idiomatic OCaml: Array.make and Array.make_matrix *)
let zeros = Array.make 5 0
let matrix = Array.make_matrix 3 4 0.0
let () = matrix.(1).(2) <- 42.0

(* Recursive: build a matrix using Array.init *)
let identity n =
  Array.init n (fun i ->
    Array.init n (fun j -> if i = j then 1.0 else 0.0))

let () =
  assert (Array.length zeros = 5);
  assert (zeros.(0) = 0);
  assert (matrix.(1).(2) = 42.0);
  assert (matrix.(0).(0) = 0.0);
  let id = identity 3 in
  assert (id.(0).(0) = 1.0);
  assert (id.(0).(1) = 0.0);
  assert (id.(1).(1) = 1.0);
  print_endline "ok"
