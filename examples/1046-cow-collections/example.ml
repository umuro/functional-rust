(* 1046: Clone-on-Write Collections
   Rust's Cow<'_, [T]> avoids cloning until mutation is needed.
   OCaml's persistent data structures give copy-on-write semantics for free:
   sharing is structural. We show lazy-clone with a ref-counted variant,
   and also OCaml's natural sharing behavior. *)

(* Approach 1: OCaml's natural sharing — lists share tails without copying *)
let sharing_demo () =
  let original = [1; 2; 3; 4; 5] in
  (* Prepending is O(1) and shares the tail — no copy *)
  let extended = 0 :: original in
  assert (List.tl extended == original)  (* physical equality: same pointer *)

(* Approach 2: Lazy clone — wrap data in a variant that defers copy *)
type 'a cow =
  | Borrowed of 'a     (* no clone, share original *)
  | Owned of 'a        (* we own this copy *)

let borrow x = Borrowed x
let is_borrowed = function Borrowed _ -> true | Owned _ -> false

(* Process data: borrow if no modification needed, clone otherwise *)
let process_data data threshold =
  if List.for_all (fun x -> x <= threshold) data then
    Borrowed data
  else
    Owned (List.map (fun x -> min x threshold) data)

let deref = function Borrowed x | Owned x -> x

(* Normalize a string: borrow if already lowercase, clone otherwise *)
let normalize_name name =
  if String.equal name (String.lowercase_ascii name) then
    Borrowed name
  else
    Owned (String.lowercase_ascii name)

(* to_mut: trigger clone only on first mutation *)
let to_mut cow =
  match cow with
  | Owned x -> (Owned x, x)         (* already owned, no clone *)
  | Borrowed x -> (Owned x, x)      (* clone now *)

let () =
  sharing_demo ();

  let data = [1; 2; 3; 4; 5] in

  (* No change needed — borrow original *)
  let r1 = process_data data 10 in
  assert (is_borrowed r1);
  assert (deref r1 = [1; 2; 3; 4; 5]);

  (* Needs clamp — produces owned copy *)
  let r2 = process_data data 3 in
  assert (not (is_borrowed r2));
  assert (deref r2 = [1; 2; 3; 3; 3]);

  (* Original unchanged *)
  assert (data = [1; 2; 3; 4; 5]);

  (* String normalization *)
  let r3 = normalize_name "alice" in
  assert (is_borrowed r3);
  assert (deref r3 = "alice");

  let r4 = normalize_name "Alice" in
  assert (not (is_borrowed r4));
  assert (deref r4 = "alice");

  (* to_mut: clone on first mutation *)
  let cow = borrow data in
  let (owned, _) = to_mut cow in
  assert (not (is_borrowed owned));

  Printf.printf "All CoW collection tests passed.\n"
