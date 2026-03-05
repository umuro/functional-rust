(* Region inference in OCaml -- handled by GC, no inference needed *)
let borrow_and_use data transform =
  let result = transform data in
  result

let () =
  let data = "hello world" in
  let result = borrow_and_use data (fun s -> String.length s) in
  Printf.printf "length: %d\n" result;

  (* Inferring which "region" a value belongs to *)
  let get_first_word s =
    match String.split_on_char ' ' s with
    | [] -> ""
    | w :: _ -> w
  in
  Printf.printf "first: %s\n" (get_first_word "hello world rust")
