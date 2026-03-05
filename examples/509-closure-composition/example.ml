(* Function composition in OCaml *)

(* compose: (b -> c) -> (a -> b) -> (a -> c) *)
let compose f g x = f (g x)

(* Pipe forward operator *)
let ( |>> ) x f = f x

(* Compose operator: f << g = compose f g *)
let ( << ) f g x = f (g x)
let ( >> ) g f x = f (g x)  (* pipe through: g then f *)

(* Build a processing pipeline from a list of transforms *)
let pipeline transforms x =
  List.fold_left (fun acc f -> f acc) x transforms

let () =
  let double = fun x -> x * 2 in
  let inc = fun x -> x + 1 in
  let square = fun x -> x * x in
  let to_str = string_of_int in

  (* compose: apply right first *)
  let double_then_inc = compose inc double in
  Printf.printf "double_then_inc(5) = %d\n" (double_then_inc 5); (* 11 *)

  (* using operators *)
  let pipeline_fn = double >> inc >> square in
  Printf.printf "double|inc|square(3) = %d\n" (pipeline_fn 3); (* ((3*2)+1)^2=49 *)

  (* pipe forward for readability *)
  let result = 5 |>> double |>> inc |>> square in
  Printf.printf "5 |>> double |>> inc |>> square = %d\n" result;

  (* point-free style *)
  let process = compose to_str (compose square inc) in
  Printf.printf "process(4) = %s\n" (process 4); (* "(4+1)^2" = "25" *)

  (* pipeline from list *)
  let transforms = [double; inc; square; double] in
  Printf.printf "pipeline(2) = %d\n" (pipeline transforms 2)
