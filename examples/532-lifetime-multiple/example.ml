(* Multiple independent references in OCaml — GC handles automatically *)
let pick_by_length (s1 : string) (sep : string) (s2 : string) =
  (* All three inputs can have independent lifetimes — GC traces them *)
  if String.length s1 > String.length s2
  then s1 ^ sep
  else s2 ^ sep

let first_of (container : string) (_key : string) =
  (* output from container, not key *)
  String.sub container 0 (min 5 (String.length container))

let () =
  let result = pick_by_length "hello world" " :: " "hi" in
  Printf.printf "%s\n" result;

  let r = first_of "abcdefgh" "ignored_key" in
  Printf.printf "first_of: %s\n" r
