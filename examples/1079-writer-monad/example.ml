(* Writer Monad — Logging Computation *)

type 'a writer = { value: 'a; log: string list }

let return x = { value = x; log = [] }
let bind w f =
  let w' = f w.value in
  { value = w'.value; log = w.log @ w'.log }

let tell msg = { value = (); log = [msg] }
let ( >>= ) = bind

let half x =
  { value = x / 2; log = [Printf.sprintf "halved %d to %d" x (x / 2)] }

let compute x =
  return x >>= fun n ->
  half n >>= fun n ->
  tell (Printf.sprintf "result is %d" n) >>= fun () ->
  return n

let () =
  let result = compute 100 in
  assert (result.value = 50);
  assert (result.log = ["halved 100 to 50"; "result is 50"]);
  Printf.printf "Value: %d\n" result.value;
  List.iter (Printf.printf "  Log: %s\n") result.log;
  print_endline "ok"
