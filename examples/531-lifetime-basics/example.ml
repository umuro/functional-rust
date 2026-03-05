(* OCaml: GC makes lifetime management automatic *)
(* No annotations needed — GC ensures references are always valid *)

(* Return a reference to one of two strings (longer one) *)
let longer s1 s2 =
  if String.length s1 >= String.length s2 then s1 else s2

(* GC: both values stay alive as long as anything references them *)
let () =
  let result =
    let s1 = "hello" in
    let s2 = "world!" in
    longer s1 s2
  in
  Printf.printf "Longer: %s\n" result;

  (* Even with different scopes, GC handles it *)
  let make_string () = "allocated string" in
  let s = make_string () in
  Printf.printf "s = %s\n" s
