(* OCaml: no lifetime annotations needed ever *)
let first_word s =
  match String.split_on_char ' ' s with
  | [] -> ""
  | w :: _ -> w

let get_or_default opt default = match opt with
  | None -> default
  | Some v -> v

let () =
  Printf.printf "%s\n" (first_word "hello world");
  Printf.printf "%s\n" (get_or_default None "default");
  Printf.printf "%s\n" (get_or_default (Some "value") "default")
