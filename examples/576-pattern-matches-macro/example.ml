(* OCaml: pattern-to-bool *)
type status = Active | Inactive | Pending | Banned

let is_active  = function Active -> true | _ -> false
let is_usable  = function Active | Pending -> true | _ -> false

let () =
  let users = [Active;Inactive;Pending;Banned;Active] in
  Printf.printf "active=%d usable=%d\n"
    (List.length (List.filter is_active  users))
    (List.length (List.filter is_usable  users));
  let s = "abc123" in
  let n = String.to_seq s |> Seq.filter (fun c -> c>='0' && c<='9') |> Seq.fold_left (fun a _ -> a+1) 0 in
  Printf.printf "digits in '%s': %d\n" s n
