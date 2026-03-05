(* Index trait concept in OCaml *)

(* Matrix with 2D indexing *)
type 'a matrix = {
  rows: int;
  cols: int;
  data: 'a array;
}

let create rows cols default =
  { rows; cols; data = Array.make (rows * cols) default }

let get m r c =
  if r >= m.rows || c >= m.cols then failwith "Index out of bounds"
  else m.data.(r * m.cols + c)

let set m r c v =
  if r >= m.rows || c >= m.cols then failwith "Index out of bounds"
  else m.data.(r * m.cols + c) <- v

let print_matrix m to_str =
  for r = 0 to m.rows - 1 do
    for c = 0 to m.cols - 1 do
      Printf.printf "%s " (to_str (get m r c))
    done;
    print_newline ()
  done

let () =
  let m = create 3 3 0 in
  for i = 0 to 2 do
    for j = 0 to 2 do
      set m i j ((i * 3) + j + 1)
    done
  done;
  Printf.printf "Matrix:\n";
  print_matrix m string_of_int;
  Printf.printf "m[1][2] = %d\n" (get m 1 2)
