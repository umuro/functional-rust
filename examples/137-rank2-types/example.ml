(* Example 137: Rank-2 Types *)
(* A rank-2 function takes a polymorphic function as argument *)

(* Approach 1: Using records to encode rank-2 polymorphism *)
type id_fn = { id : 'a. 'a -> 'a }

let apply_id (f : id_fn) =
  let x = f.id 42 in
  let y = f.id "hello" in
  (x, y)

(* Approach 2: Rank-2 with first-class modules *)
module type TRANSFORM = sig
  val transform : 'a -> 'a
end

let apply_transform (module T : TRANSFORM) =
  (T.transform 42, T.transform "hello")

module Identity : TRANSFORM = struct
  let transform x = x
end

(* Approach 3: ST monad simulation — rank-2 prevents leaking *)
type ('s, 'a) st_ref = { mutable st_contents : 'a }

type 'a st_action = { run : 's. unit -> 'a }

let new_ref v = { st_contents = v }
let read_ref r = r.st_contents
let write_ref r v = r.st_contents <- v

let run_st (action : 'a st_action) : 'a = action.run ()

(* Tests *)
let () =
  let result = apply_id { id = fun x -> x } in
  assert (fst result = 42);
  assert (snd result = "hello");

  let (i, s) = apply_transform (module Identity) in
  assert (i = 42);
  assert (s = "hello");

  let result = run_st { run = fun () ->
    let r = new_ref 0 in
    write_ref r 42;
    read_ref r
  } in
  assert (result = 42);

  Printf.printf "✓ All tests passed\n"
