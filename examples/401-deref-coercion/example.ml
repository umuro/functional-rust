(* Deref coercion in OCaml — via modules and abstraction *)

(* OCaml doesn't have deref coercions, but we can show the concept via modules *)

(* A smart-pointer-like wrapper *)
module Box = struct
  type 'a t = { value: 'a }
  let create v = { value = v }
  let deref { value } = value
  let map f b = create (f (deref b))
end

let use_string (s : string) =
  Printf.printf "String: %s (len=%d)\n" s (String.length s)

let () =
  let boxed_str = Box.create "hello world" in
  use_string (Box.deref boxed_str);  (* Manual deref — no auto coercion *)

  (* String to bytes-like access *)
  let s = "Rust programming" in
  Printf.printf "First char: %c\n" s.[0];
  Printf.printf "Substring: %s\n" (String.sub s 0 4)
