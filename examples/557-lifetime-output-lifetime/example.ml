(* Output lifetimes in OCaml — all automatic *)
let longest s1 s2 = if String.length s1 >= String.length s2 then s1 else s2
let first_char s = if String.length s > 0 then Some (String.sub s 0 1) else None
let suffix s n = if n >= String.length s then "" else String.sub s n (String.length s - n)

let () =
  Printf.printf "%s\n" (longest "hello" "hi");
  Printf.printf "%s\n" (match first_char "rust" with Some s -> s | None -> "");
  Printf.printf "%s\n" (suffix "hello world" 6)
