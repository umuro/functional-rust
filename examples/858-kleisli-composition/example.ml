(* Example 059: Kleisli Composition *)
(* Kleisli arrow: a -> m b, composed via >=> *)

let bind m f = match m with None -> None | Some x -> f x
let ( >>= ) = bind

(* Kleisli composition: (>=>) *)
let ( >=> ) f g x = f x >>= g

(* Approach 1: Composing Option-returning functions *)
let parse_int s = int_of_string_opt s
let check_positive n = if n > 0 then Some n else None
let safe_half n = if n mod 2 = 0 then Some (n / 2) else None

let validate = parse_int >=> check_positive >=> safe_half

(* Approach 2: Kleisli for Result *)
let rbind r f = match r with Error e -> Error e | Ok x -> f x
let ( >>=. ) = rbind
let kleisli_r f g x = f x >>=. g
let ( >==>. ) = kleisli_r

let parse_r s = match int_of_string_opt s with
  | Some n -> Ok n | None -> Error "parse failed"
let positive_r n = if n > 0 then Ok n else Error "not positive"
let even_r n = if n mod 2 = 0 then Ok n else Error "not even"

let validate_r = (fun s -> parse_r s) >==>. positive_r >==>. even_r

(* Approach 3: Building pipelines from Kleisli arrows *)
let pipeline steps x =
  List.fold_left (fun acc step -> acc >>= step) (Some x) steps

let steps = [check_positive; safe_half; (fun n -> if n < 100 then Some n else None)]

let () =
  assert (validate "42" = Some 21);
  assert (validate "0" = None);
  assert (validate "7" = None);
  assert (validate "bad" = None);

  assert (validate_r "42" = Ok 42);
  assert (validate_r "bad" = Error "parse failed");
  assert (validate_r "-1" = Error "not positive");
  assert (validate_r "7" = Error "not even");

  assert (pipeline steps 50 = Some 25);
  assert (pipeline steps (-1) = None);
  assert (pipeline steps 300 = None);

  Printf.printf "✓ All tests passed\n"
