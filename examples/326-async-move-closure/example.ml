(* OCaml: capture by value in closures *)

let make_greeter name = fun () -> Printf.printf "Hello, %s!\n" name

let make_counter start =
  let count = ref start in
  fun () -> let v = !count in incr count; v

let () =
  let ga = make_greeter "Alice" in
  let gb = make_greeter "Bob" in
  ga (); gb ();
  let c = make_counter 10 in
  Printf.printf "%d\n" (c ()); Printf.printf "%d\n" (c ())
