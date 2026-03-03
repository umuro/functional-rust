(* Effect handlers: intercept performed effects and decide how to resume.
   The handler has access to the delimited continuation 'k'. *)

(* A logging effect *)
effect Log : string -> unit

(* A nondeterminism effect: choose one of two values *)
effect Choose : 'a list -> 'a

(* Run with logging to a buffer *)
let collect_logs program =
  let log = ref [] in
  let result = match program () with
    | v -> v
    | effect (Log msg) k ->
      log := msg :: !log;
      continue k ()
  in
  (result, List.rev !log)

(* Run nondeterministically: return ALL results *)
let run_all program =
  match program () with
  | v -> [v]
  | effect (Choose xs) k ->
    List.concat_map (fun x -> continue k x) xs

let () =
  (* Logging handler *)
  let program1 () =
    perform (Log "starting");
    let x = 42 in
    perform (Log (Printf.sprintf "x = %d" x));
    perform (Log "done");
    x * 2
  in
  let (result, logs) = collect_logs program1 in
  Printf.printf "result=%d, logs=[%s]\n" result (String.concat "; " logs);

  (* Nondeterminism handler *)
  let program2 () =
    let x = perform (Choose [1; 2; 3]) in
    let y = perform (Choose [10; 20]) in
    x + y
  in
  let results = run_all program2 in
  Printf.printf "all results: [%s]\n" (String.concat "; " (List.map string_of_int results))
