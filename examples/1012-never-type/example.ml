(* 1012: Never Type
   Rust's ! (never) type indicates a diverging computation.
   OCaml has the 'a (empty type / bottom) via 'raise' or infinite loops.
   - Functions that raise an exception never return normally
   - Pattern matches on empty types are exhaustive automatically
   - The 'assert false' expression has type 'a — the bottom type *)

(* Diverging function — raises, never returns a value *)
let diverge_panic () : 'a =
  failwith "this never returns"

(* Another diverging function: infinite loop *)
let diverge_loop () : 'a =
  let rec loop () = loop () in
  loop ()

(* In OCaml there is no Infallible type, but we can model it with an empty variant *)
(* type infallible = |   (* empty type — no constructors *) *)

(* classify: exhaustive pattern match — 'assert false' has type 'a *)
let classify n =
  if n > 0 then Printf.sprintf "positive: %d" n
  else if n < 0 then Printf.sprintf "negative: %d" n
  else "zero"

(* The 'assert false' trick — equivalent to Rust's unreachable!() *)
let safe_head = function
  | [] -> assert false  (* caller guarantees non-empty *)
  | x :: _ -> x

(* Values that can't fail — Option/Result where Error = never *)
let always_succeeds () : (int, [`Never]) result = Ok 42

let () =
  assert (classify 5 = "positive: 5");
  assert (classify (-3) = "negative: -3");
  assert (classify 0 = "zero");

  (* assert false has type 'a — can appear in any branch *)
  let _val : int = if true then 42 else assert false in
  assert (_val = 42);

  (* always_succeeds is always Ok *)
  assert (always_succeeds () = Ok 42);

  (* diverge_panic raises — test with exception catching *)
  (try
     ignore (diverge_panic ())
   with Failure msg ->
     assert (msg = "this never returns"));

  Printf.printf "classify 5: %s\n" (classify 5)
