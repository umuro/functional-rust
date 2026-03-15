(* 943: Result Type — Railway-Oriented Error Handling

   Using Result with combinators (bind/map) for chaining fallible operations.
   Errors short-circuit the pipeline automatically.

   OCaml's Result module (4.08+) provides map, bind, and join.
   The let* syntax with Result.bind gives the same ergonomics as Rust's `?`. *)

(* ── Fallible operations ─────────────────────────────────────────────────── *)

(* Parse a string to int *)
let parse_int s =
  match int_of_string_opt s with
  | None -> Error (Printf.sprintf "not an integer: %S" s)
  | Some n -> Ok n

(* Validate that a number is positive *)
let positive x =
  if x > 0 then Ok x
  else Error (Printf.sprintf "%d is not positive" x)

(* Safe square root *)
let sqrt_safe x =
  match positive x with
  | Error e -> Error e
  | Ok n -> Ok (sqrt (float_of_int n))

(* ── Pipeline using Result.bind (>>=) ────────────────────────────────────── *)

let ( >>= ) = Result.bind

let process_bind s =
  parse_int s >>= positive >>= sqrt_safe

(* ── Pipeline using let* syntax (analogous to Rust's ?) ─────────────────── *)

let ( let* ) = Result.bind

let process s =
  let* n = parse_int s in
  let* n = positive n in
  sqrt_safe n

(* Map over the Ok value *)
let process_doubled s =
  Result.map (fun v -> v *. 2.0) (process s)

(* ── Collecting multiple results ─────────────────────────────────────────── *)

(* sequence: [Ok 1; Ok 2; Ok 3] → Ok [1;2;3], stops at first Error *)
let sequence results =
  List.fold_right
    (fun r acc ->
      match (r, acc) with
      | (Ok x, Ok xs) -> Ok (x :: xs)
      | (Error e, _) | (_, Error e) -> Error e)
    results
    (Ok [])

let () =
  (* valid input: sqrt(16) = 4.0 *)
  (match process "16" with
   | Ok r -> assert (abs_float (r -. 4.0) < Float.epsilon)
   | Error _ -> failwith "expected Ok");

  (match process "25" with
   | Ok r -> assert (abs_float (r -. 5.0) < Float.epsilon)
   | Error _ -> failwith "expected Ok");

  (* negative: process "-4" should fail with "is not positive" *)
  assert (process "-4" = Error "-4 is not positive");

  (* not an integer *)
  assert (match process "hello" with Error _ -> true | Ok _ -> false);

  (* zero *)
  assert (process "0" = Error "0 is not positive");

  (* bind and ? versions give same results *)
  let cases = ["16"; "25"; "-4"; "hello"; "0"] in
  List.iter (fun s ->
    assert (process s = process_bind s)
  ) cases;

  (* map *)
  (match process_doubled "16" with
   | Ok r -> assert (abs_float (r -. 8.0) < Float.epsilon)
   | Error _ -> failwith "expected Ok");

  (* sequence *)
  assert (sequence [Ok 1; Ok 2; Ok 3] = Ok [1; 2; 3]);
  assert (match sequence [Ok 1; Error "oops"; Ok 3] with
          | Error "oops" -> true | _ -> false);

  (* Result.is_ok / is_error *)
  assert (Result.is_ok (process "16"));
  assert (Result.is_error (process "0"));

  print_endline "943-result-railway: all tests passed"
