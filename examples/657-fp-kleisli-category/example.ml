(* Kleisli Category in OCaml *)

(* Kleisli composition (>=>) for Option *)
let ( >=> ) f g = fun x ->
  match f x with
  | None -> None
  | Some y -> g y

(* Identity arrow *)
let return_option x = Some x

(* Validators as Kleisli arrows *)
let validate_positive x = if x > 0 then Some x else None
let validate_even x = if x mod 2 = 0 then Some x else None
let double x = Some (x * 2)

(* Compose pipeline *)
let pipeline = validate_positive >=> validate_even >=> double

let () =
  let test x = match pipeline x with
    | None -> Printf.printf "%d: None\n" x
    | Some n -> Printf.printf "%d: Some %d\n" x n
  in
  test 4;   (* Some 8 *)
  test 3;   (* None - not even *)
  test (-2) (* None - not positive *)
