(* 081: Newtype Pattern
   Wrap primitives in single-field records for compile-time type safety *)

(* --- Approach 1: Physical units as distinct types --- *)

type meters = { m : float }
type seconds = { s : float }
type meters_per_second = { mps : float }

let speed dist time =
  if time.s = 0.0 then None
  else Some { mps = dist.m /. time.s }

(* --- Approach 2: Distinct ID types that cannot be accidentally mixed --- *)

type user_id  = { uid : int }
type order_id = { oid : int }

let find_user  id    = Printf.sprintf "User #%d"  id.uid
let find_order id    = Printf.sprintf "Order #%d" id.oid
(* find_user { oid = 1 } would be a type error — different record types *)

(* --- Approach 3: Newtypes with conversion functions --- *)

type celsius    = { c : float }
type fahrenheit = { f : float }

let celsius_to_fahrenheit { c } = { f = c *. 9.0 /. 5.0 +. 32.0 }
let fahrenheit_to_celsius { f } = { c = (f -. 32.0) *. 5.0 /. 9.0 }

let () =
  (match speed { m = 100.0 } { s = 10.0 } with
   | Some v -> Printf.printf "speed = %.2f m/s\n" v.mps
   | None   -> print_endline "zero time");
  Printf.printf "divide by zero -> %s\n"
    (match speed { m = 100.0 } { s = 0.0 } with None -> "None" | Some _ -> "Some");

  Printf.printf "%s\n" (find_user  { uid = 42 });
  Printf.printf "%s\n" (find_order { oid = 7  });

  let boiling_f = celsius_to_fahrenheit { c = 100.0 } in
  Printf.printf "100°C = %.1f°F\n" boiling_f.f;
  let freezing_c = fahrenheit_to_celsius { f = 32.0 } in
  Printf.printf "32°F = %.1f°C\n" freezing_c.c
