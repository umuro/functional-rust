(* 476. String splitting – OCaml *)
let () =
  let csv = "alice,30,amsterdam" in
  List.iter (fun p -> Printf.printf "'%s'\n" p) (String.split_on_char ',' csv);

  (* split_once equivalent *)
  let split_once sep s =
    match String.split_on_char sep s with
    | [] | [_] -> None
    | h::t -> Some(h, String.concat (String.make 1 sep) t)
  in
  (match split_once '=' "key=value=extra" with
   | Some(k,v) -> Printf.printf "k=%s v=%s\n" k v | None->());

  (* split_whitespace *)
  let words = List.filter ((<>) "") (String.split_on_char ' ' "  a  b  c  ") in
  Printf.printf "words: %d\n" (List.length words)
