(* 734: Typestate Basics — OCaml GADT approach
   OCaml doesn't have zero-cost phantom types like Rust, but GADTs let us
   encode state in the type safely. *)

(* State phantom types *)
type red = Red
type green = Green
type yellow = Yellow

(* Traffic light parameterized by phantom state *)
(* We use a simple record + witness value *)
type 'state light = {
  state_name: string;
  _phantom: 'state;  (* phantom field to carry state type *)
}

(* Type-safe constructors and transitions *)
let red_light () : red light = { state_name = "Red"; _phantom = Red }

let green_of_red (l : red light) : green light =
  let _ = l in
  { state_name = "Green"; _phantom = Green }

let yellow_of_green (l : green light) : yellow light =
  let _ = l in
  { state_name = "Yellow"; _phantom = Yellow }

let red_of_yellow (l : yellow light) : red light =
  let _ = l in
  { state_name = "Red"; _phantom = Red }

let print_light l = Printf.printf "Light is: %s\n" l.state_name

let () =
  let r = red_light () in
  print_light r;
  let g = green_of_red r in
  print_light g;
  let y = yellow_of_green g in
  print_light y;
  let r2 = red_of_yellow y in
  print_light r2;
  (* Invalid: green_of_red y  ← would be a type error in OCaml too! *)
  ignore r2
