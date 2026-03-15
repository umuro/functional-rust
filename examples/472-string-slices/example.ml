(* 472. String slices – OCaml *)
let () =
  let s = "Hello, World!" in
  Printf.printf "sub: %s\n" (String.sub s 7 5);
  let pos = String.index s ',' in
  Printf.printf "before comma: %s\n" (String.sub s 0 pos);
  let safe_sub s p l =
    if p>=0 && l>=0 && p+l<=String.length s then Some(String.sub s p l) else None
  in
  Printf.printf "safe: %s\n" (match safe_sub s 0 5 with Some v->v | None->"None");
  (* UTF-8: String.length counts bytes *)
  let cafe = "caf\xc3\xa9" in  (* café *)
  Printf.printf "byte_len=%d\n" (String.length cafe)
