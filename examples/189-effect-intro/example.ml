(* OCaml 5 algebraic effects: define effects, perform them, handle them.
   Like resumable exceptions with a continuation. *)

(* Note: requires OCaml 5.0+ *)

effect Print : string -> unit
effect Readline : string

(* A program that uses effects — pure in itself *)
let interactive_program () =
  perform (Print "What is your name?");
  let name = perform Readline in
  perform (Print ("Hello, " ^ name ^ "!"));
  name

(* Handler 1: real I/O *)
let run_io program =
  match program () with
  | result -> result
  | effect (Print msg) k ->
    print_endline msg;
    continue k ()
  | effect Readline k ->
    let line = input_line stdin in
    continue k line

(* Handler 2: pure simulation with list of inputs *)
let run_pure inputs program =
  let buf = Buffer.create 64 in
  let inputs = ref inputs in
  match program () with
  | result -> (result, Buffer.contents buf)
  | effect (Print msg) k ->
    Buffer.add_string buf (msg ^ "\n");
    continue k ()
  | effect Readline k ->
    let line = List.hd !inputs in
    inputs := List.tl !inputs;
    continue k line

let () =
  let (result, output) = run_pure ["OCaml 5"] interactive_program in
  Printf.printf "Output:\n%s" output;
  Printf.printf "Got name: %s\n" result
