(* Darts *)
(* Floating-point distance calculation with conditional scoring *)

let score (x: float) (y: float): int =
  let distance = sqrt (x *. x +. y *. y) in
  if distance <= 1.0 then 10
  else if distance <= 5.0 then 5
  else if distance <= 10.0 then 1
  else 0
