(* Example 185: Console DSL with Free Monad *)
(* Print, ReadLine, Exit operations as a domain-specific language *)

(* Approach 1: Full console DSL *)
type 'a console =
  | Pure of 'a
  | Print of string * (unit -> 'a console)
  | ReadLine of (string -> 'a console)
  | Exit of int  (* exit code, no continuation *)

let pure x = Pure x
let print_line msg = Print (msg, fun () -> Pure ())
let read_line () = ReadLine (fun s -> Pure s)
let exit_prog code = Exit code

let rec bind : type a b. a console -> (a -> b console) -> b console =
  fun m f -> match m with
  | Pure a -> f a
  | Print (msg, k) -> Print (msg, fun () -> bind (k ()) f)
  | ReadLine k -> ReadLine (fun s -> bind (k s) f)
  | Exit code -> Exit code

let (>>=) = bind

(* Approach 2: Interactive menu program *)
let menu_program =
  print_line "=== Menu ===" >>= fun () ->
  print_line "1. Greet" >>= fun () ->
  print_line "2. Exit" >>= fun () ->
  print_line "Choose: " >>= fun () ->
  read_line () >>= fun choice ->
  match choice with
  | "1" ->
    print_line "Enter name: " >>= fun () ->
    read_line () >>= fun name ->
    print_line ("Hello, " ^ name ^ "!") >>= fun () ->
    pure ("greeted " ^ name)
  | "2" -> exit_prog 0
  | _ ->
    print_line "Invalid choice" >>= fun () ->
    pure "error"

(* Approach 3: Pure test interpreter *)
let interpret_pure inputs prog =
  let outputs = ref [] in
  let input_idx = ref 0 in
  let rec go : type a. a console -> (a, int) result = function
    | Pure a -> Ok a
    | Print (msg, k) ->
      outputs := msg :: !outputs;
      go (k ())
    | ReadLine k ->
      let s = List.nth inputs !input_idx in
      incr input_idx;
      go (k s)
    | Exit code -> Error code
  in
  let result = go prog in
  (List.rev !outputs, result)

let () =
  (* Test menu with greet *)
  let (out, result) = interpret_pure ["1"; "Alice"] menu_program in
  assert (out = ["=== Menu ==="; "1. Greet"; "2. Exit"; "Choose: "; "Enter name: "; "Hello, Alice!"]);
  assert (result = Ok "greeted Alice");

  (* Test menu with exit *)
  let (out2, result2) = interpret_pure ["2"] menu_program in
  assert (List.length out2 = 4);
  assert (result2 = Error 0);

  (* Test menu with invalid *)
  let (out3, result3) = interpret_pure ["x"] menu_program in
  assert (List.mem "Invalid choice" out3);
  assert (result3 = Ok "error");

  print_endline "✓ All tests passed"
