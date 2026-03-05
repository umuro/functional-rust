(* From/Into conversion traits in OCaml *)

(* Infallible conversion *)
module type From = sig
  type source
  type target
  val from : source -> target
end

(* Fallible conversion *)
module type TryFrom = sig
  type source
  type target
  type error
  val try_from : source -> (target, error) result
end

(* Example: string to positive int *)
module StringToPositiveInt : TryFrom
  with type source = string
   and type target = int
   and type error = string = struct
  type source = string
  type target = int
  type error = string
  let try_from s =
    match int_of_string_opt s with
    | Some n when n > 0 -> Ok n
    | Some _ -> Error "Must be positive"
    | None -> Error ("Not a number: " ^ s)
end

let () =
  let test s =
    match StringToPositiveInt.try_from s with
    | Ok n -> Printf.printf "Ok: %d\n" n
    | Error e -> Printf.printf "Error: %s\n" e
  in
  test "42";
  test "-5";
  test "abc";
  (* Infallible *)
  let n : int = 42 in
  let f : float = Float.of_int n in
  Printf.printf "int->float: %g\n" f
