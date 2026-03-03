(* Robot Simulator — State with Immutable Records *)

type direction = North | East | South | West
type robot = { x: int; y: int; dir: direction }
type instruction = TurnLeft | TurnRight | Advance

let turn_right = function
  | North -> East | East -> South | South -> West | West -> North

let turn_left = function
  | North -> West | West -> South | South -> East | East -> North

let advance r = match r.dir with
  | North -> { r with y = r.y + 1 }
  | East -> { r with x = r.x + 1 }
  | South -> { r with y = r.y - 1 }
  | West -> { r with x = r.x - 1 }

let execute r = function
  | TurnLeft -> { r with dir = turn_left r.dir }
  | TurnRight -> { r with dir = turn_right r.dir }
  | Advance -> advance r

let run r instructions = List.fold_left execute r instructions

let () =
  let r = { x=0; y=0; dir=North } in
  let r = run r [Advance; TurnRight; Advance; Advance; TurnLeft; Advance] in
  assert (r.x = 2 && r.y = 2)
