(* 185: Free Monad DSL — Console operations as a composable data structure.
   The Free monad lifts any functor into a monad, letting us describe
   effectful programs as pure values that can be interpreted later. *)

(* ── DSL functor ──────────────────────────────────────────────────────────── *)

(* One step of a console program; 'k is the continuation type *)
type 'k console_f =
  | Print   of string * (unit -> 'k)
  | ReadLine of (string -> 'k)
  | Exit    of int

(* ── Free monad ───────────────────────────────────────────────────────────── *)

type 'a console =
  | Pure  of 'a
  | Free  of ('a console) console_f

(* functor map over console_f *)
let map_f (f : 'a -> 'b) : 'a console_f -> 'b console_f = function
  | Print   (msg, k) -> Print (msg, fun () -> f (k ()))
  | ReadLine k       -> ReadLine (fun s -> f (k s))
  | Exit n           -> Exit n

(* monadic bind — the key Free monad operation *)
let rec bind (ma : 'a console) (f : 'a -> 'b console) : 'b console =
  match ma with
  | Pure a -> f a
  | Free step -> Free (map_f (fun k -> bind k f) step)

(* ── DSL smart constructors ───────────────────────────────────────────────── *)

let print_line msg : unit console =
  Free (Print (msg, fun () -> Pure ()))

let read_line_dsl : string console =
  Free (ReadLine (fun s -> Pure s))

let exit_prog code : 'a console =
  Free (Exit code)

(* Infix bind for sequencing: ma >>= f *)
let ( >>= ) = bind

(* Sequence two actions, discarding the first result *)
let ( >> ) ma mb = ma >>= fun _ -> mb

(* ── Example program: interactive menu ───────────────────────────────────── *)

let menu_program : string console =
  print_line "=== Menu ===" >>
  print_line "1. Greet"     >>
  print_line "2. Exit"      >>
  print_line "Choose: "     >>
  read_line_dsl >>= fun choice ->
  match choice with
  | "1" ->
    print_line "Enter name: " >>
    read_line_dsl >>= fun name ->
    print_line (Printf.sprintf "Hello, %s!" name) >>
    Pure (Printf.sprintf "greeted %s" name)
  | "2" -> exit_prog 0
  | _   -> print_line "Invalid choice" >> Pure "error"

(* ── Pure interpreter (for testing) ─────────────────────────────────────────
   Runs the program against a list of canned inputs, collecting outputs.
   No I/O side effects — the program is just data. *)

type 'a program_result = Ok of 'a | Exited of int

let run_pure (inputs : string list) (prog : string console)
    : string list * string program_result =
  let outputs = ref [] in
  let input_q = Queue.of_seq (List.to_seq inputs) in
  let rec step = function
    | Pure a -> Ok a
    | Free (Print (msg, k)) ->
      outputs := msg :: !outputs;
      step (k ())
    | Free (ReadLine k) ->
      let s = if Queue.is_empty input_q then "" else Queue.pop input_q in
      step (k s)
    | Free (Exit n) -> Exited n
  in
  let result = step prog in
  (List.rev !outputs, result)

(* ── Demo ─────────────────────────────────────────────────────────────────── *)

let () =
  (* Greet path *)
  let (out, result) = run_pure ["1"; "Alice"] menu_program in
  Printf.printf "--- greet path ---\n";
  List.iter (Printf.printf "  %s\n") out;
  (match result with
   | Ok s     -> Printf.printf "  result: %s\n" s
   | Exited n -> Printf.printf "  exited: %d\n" n);

  (* Exit path *)
  let (out2, result2) = run_pure ["2"] menu_program in
  Printf.printf "--- exit path ---\n";
  List.iter (Printf.printf "  %s\n") out2;
  (match result2 with
   | Ok s     -> Printf.printf "  result: %s\n" s
   | Exited n -> Printf.printf "  exited: %d\n" n)
