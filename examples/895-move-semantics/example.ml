(* Example 101: Move Semantics — OCaml GC vs Rust Ownership Transfer *)

(* In OCaml, all values are garbage-collected. "Moving" doesn't exist —
   you just share references and the GC cleans up. *)

(* Approach 1: Simple value passing — OCaml shares freely *)
let use_string s =
  Printf.printf "Using: %s\n" s;
  String.length s

let approach1 () =
  let greeting = "Hello, ownership!" in
  let len1 = use_string greeting in
  (* In OCaml, we can use greeting again — no move happened *)
  let len2 = use_string greeting in
  assert (len1 = len2);
  Printf.printf "Used greeting twice, lengths: %d, %d\n" len1 len2

(* Approach 2: Passing structured data — still no move *)
type person = { name : string; age : int }

let greet p =
  Printf.printf "Hello, %s (age %d)!\n" p.name p.age

let approach2 () =
  let p = { name = "Alice"; age = 30 } in
  greet p;
  greet p;  (* No problem — p is still accessible *)
  Printf.printf "Person %s is still here\n" p.name

(* Approach 3: Simulating ownership transfer with option ref *)
let take_ownership (slot : string option ref) =
  match !slot with
  | Some s ->
    slot := None;  (* "consume" the value *)
    Printf.printf "Took ownership of: %s\n" s;
    s
  | None ->
    failwith "Value already moved!"

let approach3 () =
  let data = ref (Some "precious data") in
  let _taken = take_ownership data in
  (* Trying again would fail — simulating Rust's move *)
  (try
     let _ = take_ownership data in ()
   with Failure msg ->
     Printf.printf "Caught: %s\n" msg);
  Printf.printf "Simulated move semantics in OCaml\n"

(* Tests *)
let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
