(* Record destructuring in OCaml *)
type point = { x: float; y: float }
type person = { name: string; age: int; email: string }

let dist_from_origin { x; y } = sqrt (x*.x +. y*.y)
let greet { name; age; _ } = Printf.sprintf "Hello %s, age %d" name age

let classify_person = function
  | { age; _ } when age < 18 -> "minor"
  | { age; _ } when age < 65 -> "adult"
  | _                        -> "senior"

let () =
  Printf.printf "%.1f\n" (dist_from_origin { x=3.; y=4. });
  let alice = { name="Alice"; age=30; email="a@b.com" } in
  Printf.printf "%s\n" (greet alice);
  Printf.printf "%s\n" (classify_person alice)
