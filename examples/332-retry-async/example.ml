(* OCaml: retry with exponential backoff *)

type ('a, 'e) retry_error = Transient of 'e | Permanent of 'e

let retry ~max_attempts ~base_delay ~multiplier f =
  let rec loop attempt delay =
    match f () with
    | Ok v -> Ok v
    | Error (Permanent e) -> Error e
    | Error (Transient e) ->
      if attempt >= max_attempts then Error e
      else begin
        Printf.printf "Attempt %d failed, retrying in %.0fms\n" attempt (delay *. 1000.);
        Thread.delay delay;
        loop (attempt + 1) (delay *. multiplier)
      end
  in
  loop 1 base_delay

let () =
  let counter = ref 0 in
  let result = retry ~max_attempts:3 ~base_delay:0.01 ~multiplier:2.0 (fun () ->
    incr counter;
    if !counter < 3 then Error (Transient "not ready")
    else Ok 42
  ) in
  match result with
  | Ok v -> Printf.printf "Success: %d\n" v
  | Error e -> Printf.printf "Failed: %s\n" e
