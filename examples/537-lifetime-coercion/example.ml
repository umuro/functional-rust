(* Lifetime subtyping is implicit in OCaml — GC handles it *)
(* Demonstrating the concept with value semantics *)
let use_short_lived s =
  Printf.printf "Short-lived: %s\n" s

let use_long_lived s =
  Printf.printf "Long-lived: %s\n" s

let () =
  (* In OCaml, any string can be used where a "shorter-lived" one is needed *)
  let long_lived = "This string lives long" in
  use_short_lived long_lived;   (* using longer-lived value in shorter context *)
  use_long_lived long_lived;

  (* The concept: you can always use something that lives LONGER *)
  let create_and_use () =
    let s = "created here" in
    use_short_lived s;  (* s goes out of scope after this block *)
    s (* but OCaml keeps it alive via GC *)
  in
  let result = create_and_use () in
  Printf.printf "Still valid: %s\n" result
