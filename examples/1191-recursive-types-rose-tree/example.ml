(* Recursive Types — Rose Tree *)
(* Multi-way tree (rose tree) structure *)

type 'a rose = Rose of 'a * 'a rose list

let leaf x = Rose (x, [])

let rec depth (Rose (_, children)) =
  1 + List.fold_left (fun acc c -> max acc (depth c)) 0 children

let rec size (Rose (_, children)) =
  1 + List.fold_left (fun acc c -> acc + size c) 0 children

let rec map f (Rose (x, children)) =
  Rose (f x, List.map (map f) children)

let tree = Rose ("root", [
  Rose ("a", [leaf "a1"; leaf "a2"]);
  Rose ("b", [leaf "b1"]);
  leaf "c"
])

let () = Printf.printf "Depth: %d, Size: %d\n" (depth tree) (size tree)
