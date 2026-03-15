(* 1046: Clone-on-Write for Collections *)
(* OCaml's immutable data structures are inherently "copy-on-write" *)
(* via structural sharing — no explicit Cow needed *)

(* Approach 1: OCaml's natural structural sharing *)
let structural_sharing () =
  let original = [1; 2; 3; 4; 5] in
  (* Prepending shares the tail — no copy needed *)
  let extended = 0 :: original in
  assert (extended = [0; 1; 2; 3; 4; 5]);
  assert (original = [1; 2; 3; 4; 5]);  (* unchanged *)
  (* Both lists share the [1;2;3;4;5] tail in memory *)
  ()

(* Approach 2: Explicit copy-on-write with ref *)
type 'a cow = {
  mutable data: 'a array;
  mutable is_owned: bool;
}

let cow_borrow arr = { data = arr; is_owned = false }

let cow_to_owned cow =
  if cow.is_owned then cow
  else { data = Array.copy cow.data; is_owned = true }

let cow_modify cow idx value =
  let cow = cow_to_owned cow in
  cow.data.(idx) <- value;
  cow

let cow_demo () =
  let shared = [|1; 2; 3; 4; 5|] in
  let c1 = cow_borrow shared in
  let c2 = cow_borrow shared in
  assert (not c1.is_owned);
  assert (not c2.is_owned);
  (* Modify c1 — triggers copy *)
  let c1 = cow_modify c1 0 99 in
  assert (c1.is_owned);
  assert (c1.data.(0) = 99);
  (* c2 and shared are unaffected *)
  assert (c2.data.(0) = 1);
  assert (shared.(0) = 1)

(* Approach 3: Functional update pattern (natural CoW) *)
let functional_update () =
  let data = [1; 2; 3; 4; 5] in
  (* "Modify" by creating new structure *)
  let modified = List.map (fun x -> if x = 3 then 99 else x) data in
  assert (data = [1; 2; 3; 4; 5]);    (* original unchanged *)
  assert (modified = [1; 2; 99; 4; 5])

let () =
  structural_sharing ();
  cow_demo ();
  functional_update ();
  Printf.printf "✓ All tests passed\n"
