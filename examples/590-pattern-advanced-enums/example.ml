(* Advanced enum cookbook in OCaml *)
type json =
  | Null
  | Bool   of bool
  | Num    of float
  | Str    of string
  | Array  of json list
  | Object of (string * json) list

let rec show = function
  | Null       -> "null"
  | Bool b     -> string_of_bool b
  | Num n      -> string_of_float n
  | Str s      -> Printf.sprintf "%S" s
  | Array xs   -> Printf.sprintf "[%s]" (String.concat "," (List.map show xs))
  | Object kvs ->
    let pairs = List.map (fun (k,v)->Printf.sprintf "%S:%s" k (show v)) kvs in
    Printf.sprintf "{%s}" (String.concat "," pairs)

let rec depth = function
  | Array xs   -> 1 + List.fold_left (fun a x->max a (depth x)) 0 xs
  | Object kvs -> 1 + List.fold_left (fun a (_,v)->max a (depth v)) 0 kvs
  | _          -> 0

let () =
  let j = Object ["name",Str"Alice"; "age",Num 30.0;
    "scores",Array[Num 95.0;Num 87.0];
    "active",Bool true; "notes",Null] in
  Printf.printf "%s\n" (show j);
  Printf.printf "depth=%d\n" (depth j)
