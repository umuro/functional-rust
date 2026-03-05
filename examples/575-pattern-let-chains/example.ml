(* OCaml: flat chaining with let* *)
let (let*) = Option.bind

let process s =
  let* n = (try Some(int_of_string s) with _->None) in
  let* p = (if n > 0 then Some n else None) in
  let* e = (if p mod 2 = 0 then Some p else None) in
  Some (e * 2)

let () =
  List.iter (fun s ->
    match process s with
    | Some v -> Printf.printf "%s -> %d\n" s v
    | None   -> Printf.printf "%s -> invalid\n" s
  ) ["4";"-2";"3";"abc";"8"]
