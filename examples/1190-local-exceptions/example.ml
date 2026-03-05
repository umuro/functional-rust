(* Local Exceptions *)
(* Use local exceptions for control flow *)

(* Local exception for early return *)
let find_first pred lst =
  let exception Found of int in
  try
    List.iteri (fun i x -> if pred x then raise (Found i)) lst;
    None
  with Found i -> Some i

let idx = find_first (fun x -> x > 10) [3; 7; 12; 5; 20]
let () = match idx with
  | Some i -> Printf.printf "First > 10 at index %d\n" i
  | None -> print_endline "Not found"

(* Local exception for loop break *)
let sum_until_negative lst =
  let exception Stop in
  let total = ref 0 in
  (try List.iter (fun x ->
    if x < 0 then raise Stop;
    total := !total + x
  ) lst with Stop -> ());
  !total

let () = Printf.printf "Sum: %d\n" (sum_until_negative [1; 2; 3; -1; 5])
