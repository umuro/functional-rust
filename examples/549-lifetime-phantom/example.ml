(* Phantom types in OCaml — type-level markers *)
(* Used to add type constraints without runtime cost *)

type 'a validated = Validated of 'a

let validate (s : string) : string validated =
  if String.length s > 0 then Validated s
  else failwith "empty string"

let use_validated (Validated s) =
  Printf.printf "Using validated: %s\n" s

(* Phantom state machine *)
type locked = Locked
type unlocked = Unlocked
type 'state door = Door of string

let make_door name : locked door = Door name
let unlock (Door name : locked door) : unlocked door = Door name
let open_door (Door name : unlocked door) = Printf.printf "Opening %s\n" name

let () =
  let v = validate "hello" in
  use_validated v;
  let door = make_door "front" in
  let open_door_ = unlock door in
  open_door open_door_
