(* 1014: Recover from Panic
   Rust uses std::panic::catch_unwind to convert panics into Result.
   OCaml uses try...with to catch exceptions — the idiomatic equivalent.
   Exceptions in OCaml are lightweight and pervasive; catching them is normal. *)

(* Approach 1: catch_unwind equivalent — wrap any thunk in try/with *)
let safe_divide a b =
  if b = 0 then Error "division by zero"
  else
    (* Demonstrate catching a failwith *)
    match
      (try
         if b = 0 then failwith "division by zero";
         Ok (a / b)
       with Failure msg -> Error msg)
    with
    | r -> r

(* Generic: run a thunk and convert any exception to Result *)
let catch_exn f =
  try Ok (f ())
  with
  | Failure msg -> Error msg
  | Invalid_argument msg -> Error msg
  | exn -> Error (Printexc.to_string exn)

(* Approach 2: catch_unwind with state — works naturally in OCaml *)
let catch_with_state data =
  let data = ref data in
  catch_exn (fun () ->
    data := !data @ [42];
    if List.length !data > 5 then
      failwith "too many elements";
    List.fold_left (+) 0 !data)
  |> (fun result -> (result, !data))

(* Approach 3: Quiet panic suppression — OCaml does not print exceptions
   unless they bubble to the top; no hook needed *)
let with_quiet_exn f =
  try Ok (f ())
  with exn -> Error (Printexc.to_string exn)

let () =
  assert (safe_divide 10 2 = Ok 5);
  assert (safe_divide 10 0 = Error "division by zero");

  (* catch_exn *)
  let ok = catch_exn (fun () -> 42) in
  assert (ok = Ok 42);

  let err = catch_exn (fun () -> failwith "boom"; 0) in
  assert (Result.is_error err);

  (* catch_with_state *)
  let (result, final_data) = catch_with_state [1; 2; 3] in
  assert (Result.is_ok result);
  assert (List.length final_data = 4);  (* 42 was appended *)

  let (result2, _) = catch_with_state [1; 2; 3; 4; 5] in
  assert (Result.is_error result2);

  (* quiet exn *)
  let quiet = with_quiet_exn (fun () -> failwith "silent failure"; 0) in
  assert (Result.is_error quiet);

  let quiet_ok = with_quiet_exn (fun () -> 42) in
  assert (quiet_ok = Ok 42);

  Printf.printf "safe_divide 10/2: %s\n"
    (match safe_divide 10 2 with Ok n -> string_of_int n | Error e -> e)
