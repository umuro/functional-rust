(* Match guards with 'when' in OCaml *)
let grade n =
  match n with
  | n when n >= 90 -> "A"
  | n when n >= 80 -> "B"
  | n when n >= 70 -> "C"
  | n when n >= 60 -> "D"
  | _              -> "F"

type shape = Circle of float | Rect of float * float

let describe = function
  | Circle r when r <= 0.0 -> "invalid"
  | Circle r when r < 1.0  -> "tiny circle"
  | Circle _               -> "circle"
  | Rect (w, h) when w = h -> "square"
  | Rect (w, h) when w > h -> "wide"
  | Rect _                 -> "tall"

let () =
  List.iter (fun n -> Printf.printf "%d->%s " n (grade n)) [95;82;74;61;45];
  print_newline ();
  List.iter (fun s -> Printf.printf "%s " (describe s))
    [Circle (-1.0); Circle 0.5; Circle 2.0; Rect(3.,3.); Rect(5.,2.); Rect(2.,5.)];
  print_newline ()
