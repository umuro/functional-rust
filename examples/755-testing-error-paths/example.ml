(* 755: Testing Error Paths — OCaml *)

type parse_error =
  | Empty
  | InvalidChar of char
  | TooLong
  | OutOfRange of int

let parse_positive (s : string) : (int, parse_error) result =
  if String.length s = 0 then Error Empty
  else if String.length s > 10 then Error TooLong
  else
    match String.to_seq s |> Seq.find (fun c -> c < '0' || c > '9') with
    | Some c -> Error (InvalidChar c)
    | None ->
      let n = int_of_string s in
      if n <= 0 then Error (OutOfRange n)
      else Ok n

(* Test helpers for errors *)
let assert_error ?(expected=None) result =
  match result, expected with
  | Error _, None -> ()
  | Error e, Some exp ->
    if e <> exp then
      failwith (Printf.sprintf "wrong error variant")
  | Ok _, _ -> failwith "expected Error, got Ok"

let () =
  (* Happy path *)
  assert (parse_positive "42" = Ok 42);

  (* Error paths *)
  assert_error (parse_positive "");
  assert_error ~expected:(Some Empty) (parse_positive "");
  assert_error ~expected:(Some TooLong) (parse_positive "12345678901");
  assert_error (parse_positive "12a45");
  assert_error ~expected:(Some (OutOfRange (-5))) (parse_positive "-5");

  (* expect-style: assert with message *)
  let result = parse_positive "100" in
  let n = match result with
    | Ok n -> n
    | Error _ -> failwith "parse_positive '100' should succeed"
  in
  assert (n = 100);

  Printf.printf "Error path tests passed!\n"
