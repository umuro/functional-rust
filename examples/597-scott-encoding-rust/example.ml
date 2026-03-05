(* Scott encoding in OCaml *)
(* Scott Bool: same as Church Bool *)
(* Scott List: cons h t nil_case cons_case = cons_case h t *)

(* Scott Maybe *)
let scott_none none_case _some_case = none_case
let scott_some v _none_case some_case = some_case v

(* Usage: match on encoded Maybe *)
let scott_match_maybe m on_none on_some = m on_none on_some

let () =
  let none = scott_none in
  let some42 = scott_some 42 in
  Printf.printf "none: %d\n" (scott_match_maybe none 0 (fun v -> v));
  Printf.printf "some 42: %d\n" (scott_match_maybe some42 0 (fun v -> v));

  (* Scott List *)
  let scott_nil  nil_f _cons_f = nil_f () in
  let scott_cons h t nil_f cons_f = cons_f h t in

  let lst = scott_cons 1 (scott_cons 2 (scott_cons 3 scott_nil)) in

  let rec to_list encoded =
    encoded (fun () -> []) (fun h t -> h :: to_list t) in
  Printf.printf "list: %s\n"
    (String.concat "," (List.map string_of_int (to_list lst)))
