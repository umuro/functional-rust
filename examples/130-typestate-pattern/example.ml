(* Example 130: Typestate Pattern — State Machines in Types *)

(* Approach 1: GADT-based state machine *)
type open_state = Open_s
type closed_state = Closed_s
type locked_state = Locked_s

type _ door =
  | OpenDoor : open_state door
  | ClosedDoor : closed_state door
  | LockedDoor : locked_state door

let close_door : open_state door -> closed_state door = fun _ -> ClosedDoor
let open_door : closed_state door -> open_state door = fun _ -> OpenDoor
let lock_door : closed_state door -> locked_state door = fun _ -> LockedDoor
let unlock_door : locked_state door -> closed_state door = fun _ -> ClosedDoor

(* Approach 2: Module-based state machine *)
module type DOOR_STATE = sig type t val name : string end
module Open : DOOR_STATE = struct type t = open_state let name = "open" end
module Closed : DOOR_STATE = struct type t = closed_state let name = "closed" end
module Locked : DOOR_STATE = struct type t = locked_state let name = "locked" end

(* Approach 3: Phantom type state *)
type 'state door_p = { material : string }

let make_open_door material : open_state door_p = { material }
let close_p (d : open_state door_p) : closed_state door_p = { material = d.material }
let open_p (d : closed_state door_p) : open_state door_p = { material = d.material }
let lock_p (d : closed_state door_p) : locked_state door_p = { material = d.material }
let unlock_p (d : locked_state door_p) : closed_state door_p = { material = d.material }

(* Tests *)
let () =
  let d = OpenDoor in
  let d = close_door d in
  let d = lock_door d in
  let d = unlock_door d in
  let _ = open_door d in

  let d = make_open_door "wood" in
  let d = close_p d in
  let d = lock_p d in
  let d = unlock_p d in
  let d = open_p d in
  assert (d.material = "wood");

  Printf.printf "✓ All tests passed\n"
