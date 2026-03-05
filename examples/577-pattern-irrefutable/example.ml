(* Irrefutable vs refutable in OCaml *)
let () =
  (* Irrefutable let bindings *)
  let (a, b) = (1, 2) in
  Printf.printf "a=%d b=%d\n" a b;

  (* Irrefutable function param *)
  let add (x,y) = x+y in
  Printf.printf "add=%d\n" (add (3,4));

  (* Refutable — requires match *)
  let opt = Some 42 in
  (match opt with Some v->Printf.printf "got %d\n" v | None->());

  (* OCaml warns if you do: let Some v = opt *)
  (* That pattern is "non-exhaustive" — use match instead *)
  ()
