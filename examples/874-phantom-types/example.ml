(* Example 080: Phantom Types *)
(* Compile-time safety with phantom type parameters *)

(* Approach 1: Phantom types for units of measure *)
type meters
type seconds
type meters_per_second

type 'a quantity = { value : float }

let meters v : meters quantity = { value = v }
let seconds v : seconds quantity = { value = v }

let speed (d : meters quantity) (t : seconds quantity) : meters_per_second quantity =
  { value = d.value /. t.value }

let add_same (a : 'a quantity) (b : 'a quantity) : 'a quantity =
  { value = a.value +. b.value }

let scale (a : 'a quantity) (factor : float) : 'a quantity =
  { value = a.value *. factor }

(* Approach 2: Phantom types for state machines *)
type unlocked
type locked

type 'state door = { name : string }

let new_door name : unlocked door = { name }
let lock (d : unlocked door) : locked door = { name = d.name }
let unlock (d : locked door) : unlocked door = { name = d.name }
let walk_through (d : unlocked door) = Printf.sprintf "Walked through %s" d.name
(* let walk_through_locked (d : locked door) = ... would be a type error *)

(* Approach 3: Phantom types for validated data *)
type unvalidated
type validated

type 'a email = Email of string

let create_email s : unvalidated email = Email s
let validate_email (Email s : unvalidated email) : validated email option =
  if String.contains s '@' then Some (Email s) else None
let send_email (Email s : validated email) =
  Printf.sprintf "Sent to %s" s

(* Tests *)
let () =
  let d = meters 100.0 in
  let t = seconds 10.0 in
  let s = speed d t in
  assert (s.value = 10.0);

  let d2 = add_same d (meters 50.0) in
  assert (d2.value = 150.0);

  let d3 = scale d 2.0 in
  assert (d3.value = 200.0);

  (* Door state machine *)
  let door = new_door "front" in
  let msg = walk_through door in
  assert (msg = "Walked through front");
  let locked_door = lock door in
  let unlocked_again = unlock locked_door in
  let _ = walk_through unlocked_again in

  (* Validated email *)
  let raw = create_email "user@example.com" in
  (match validate_email raw with
   | Some valid -> assert (send_email valid = "Sent to user@example.com")
   | None -> assert false);

  let bad = create_email "invalid" in
  assert (validate_email bad = None);

  Printf.printf "✓ All tests passed\n"
