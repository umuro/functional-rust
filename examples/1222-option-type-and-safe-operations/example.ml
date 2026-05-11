let safe_div a b =
  if b = 0 then None
  else Some (a / b)

let safe_head lst =
  match lst with
  | [] -> None
  | hd :: _ -> Some hd

let get_or_default opt default =
  match opt with
  | Some x -> x
  | None -> default

let () =
  let result1 = safe_div 10 2 in
  let result2 = safe_div 10 0 in
  Printf.printf "10/2 = %s\n"
    (match result1 with Some x -> string_of_int x | None -> "None");
  Printf.printf "10/0 = %s\n"
    (match result2 with Some x -> string_of_int x | None -> "None");
  Printf.printf "safe_head [] = %d\n" (get_or_default (safe_head []) 0)