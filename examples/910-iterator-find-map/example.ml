(* 271. Transform-and-find with find_map() - OCaml *)

let find_map f lst =
  let rec aux = function
    | [] -> None
    | x :: xs -> (match f x with Some _ as r -> r | None -> aux xs)
  in aux lst

let () =
  let strings = ["hello"; "42"; "world"; "17"; "foo"] in
  let first_int = find_map int_of_string_opt strings in
  Printf.printf "First int: %s\n"
    (match first_int with Some n -> string_of_int n | None -> "None");

  let first_long_len = find_map (fun w ->
    let l = String.length w in if l > 4 then Some l else None
  ) strings in
  Printf.printf "First long word length: %s\n"
    (match first_long_len with Some n -> string_of_int n | None -> "None");

  let env_vars = ["PATH=/usr/bin"; "HOME=/root"; "BAD"; "USER=alice"] in
  let first_kv = find_map (fun s ->
    match String.split_on_char '=' s with
    | [k; v] -> Some (k, v)
    | _ -> None
  ) env_vars in
  match first_kv with
  | Some (k, v) -> Printf.printf "First valid var: %s=%s\n" k v
  | None -> print_endline "None"
