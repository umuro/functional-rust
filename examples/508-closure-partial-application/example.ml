(* Partial application in OCaml — natural via currying *)

(* All multi-arg functions are curried by default *)
let add x y = x + y
let multiply x y = x * y
let clamp lo hi x = max lo (min hi x)
let between lo hi x = x >= lo && x <= hi

(* Partial application — just apply some args *)
let add5 = add 5         (* int -> int *)
let double = multiply 2  (* int -> int *)
let clamp_0_100 = clamp 0 100   (* int -> int *)
let in_teens = between 13 19    (* int -> bool *)

(* Partial application in pipelines *)
let process items =
  items
  |> List.map add5
  |> List.map double
  |> List.filter in_teens

(* Generic partial: fix first argument *)
let partial f a = fun b -> f a b

let () =
  Printf.printf "add5(10) = %d\n" (add5 10);
  Printf.printf "double(7) = %d\n" (double 7);
  Printf.printf "clamp(150) = %d\n" (clamp_0_100 150);
  Printf.printf "in_teens(15) = %b\n" (in_teens 15);
  Printf.printf "in_teens(20) = %b\n" (in_teens 20);

  let results = process [1; 2; 3; 4; 5; 6] in
  Printf.printf "processed: [%s]\n" (String.concat ";" (List.map string_of_int results));

  let times3 = partial multiply 3 in
  Printf.printf "times3(8) = %d\n" (times3 8)
