(* Option Map *)
(* OCaml 99 Problems #42 *)

let double opt = Option.map (fun x -> x * 2) opt
let stringify opt = Option.map string_of_int opt

(* Tests *)
let () =
  assert (double (Some 21) = Some 42);
  assert (double None = None);
  assert (stringify (Some 7) = Some "7");
  assert (stringify None = None);

  (* functor law: map id = id *)
  let opt = Some 5 in
  assert (Option.map (fun x -> x) opt = opt);

  (* functor law: map f . map g = map (g . f) *)
  let opt3 = Some 3 in
  let f x = x + 1 in
  let g x = x * 2 in
  assert (Option.map g (Option.map f opt3) = Option.map (fun x -> g (f x)) opt3);

  print_endline "✓ OCaml tests passed"
