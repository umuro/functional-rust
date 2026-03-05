(* Option — Chaining Multiple Lookups *)
(* Chain Option.bind for multi-step lookups *)

let users = [("alice", 1); ("bob", 2)]
let profiles = [(1, "Engineer"); (2, "Designer")]
let salaries = [("Engineer", 90000); ("Designer", 85000)]

let get_salary name =
  List.assoc_opt name users
  |> Option.bind (fun id -> List.assoc_opt id profiles)
  |> Option.bind (fun role -> List.assoc_opt role salaries)

let () = List.iter (fun name ->
  match get_salary name with
  | Some s -> Printf.printf "%s earns %d\n" name s
  | None -> Printf.printf "%s: unknown\n" name
) ["alice"; "bob"; "charlie"]
