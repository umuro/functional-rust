(* 474. String formatting – OCaml *)
let () =
  let s = Printf.sprintf "Hello, %s! Age: %d" "Alice" 30 in
  print_string s; print_newline ();
  Printf.printf "|%-10s|%10s|\n" "left" "right";
  Printf.printf "|%010d|\n" 42;
  Printf.printf "pi=%.5f sci=%e\n" Float.pi 1234567.89;
  let buf = Buffer.create 32 in
  Buffer.add_string buf "start";
  Printf.bprintf buf " %d" 99;
  Printf.printf "buf=%s\n" (Buffer.contents buf)
