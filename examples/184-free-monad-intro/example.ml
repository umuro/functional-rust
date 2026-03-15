(* Example 184: Introduction to Free Monads *)
(* Separate program description from interpretation *)

(* Approach 1: Basic Free monad *)
type ('f, 'a) free =
  | Pure of 'a
  | Free of ('f, 'a) free 'f
(* OCaml can't directly express higher-kinded types for Free,
   so we specialize for a concrete functor *)

(* Approach 1: Free monad specialized for a simple DSL *)
type 'next console_f =
  | Print of string * 'next
  | GetLine of (string -> 'next)

type 'a console =
  | CPure of 'a
  | CFree of 'a console console_f

let print msg = CFree (Print (msg, CPure ()))
let get_line () = CFree (GetLine (fun s -> CPure s))

let rec bind : type a b. a console -> (a -> b console) -> b console =
  fun m f -> match m with
  | CPure a -> f a
  | CFree (Print (msg, next)) -> CFree (Print (msg, bind next f))
  | CFree (GetLine k) -> CFree (GetLine (fun s -> bind (k s) f))

let (>>=) = bind
let return_ x = CPure x

(* Approach 2: Interpret to string list (pure) *)
let rec interpret_pure (inputs : string list) (prog : 'a console) : string list * 'a =
  match prog with
  | CPure a -> ([], a)
  | CFree (Print (msg, next)) ->
    let (outputs, result) = interpret_pure inputs next in
    (msg :: outputs, result)
  | CFree (GetLine k) ->
    (match inputs with
     | [] -> failwith "No more input"
     | x :: rest -> interpret_pure rest (k x))

(* Approach 3: Chain operations *)
let program =
  print "What is your name?" >>= fun () ->
  get_line () >>= fun name ->
  print ("Hello, " ^ name ^ "!") >>= fun () ->
  return_ name

let () =
  (* Test pure interpretation *)
  let (outputs, result) = interpret_pure ["Alice"] program in
  assert (outputs = ["What is your name?"; "Hello, Alice!"]);
  assert (result = "Alice");

  let (outputs2, _) = interpret_pure ["Bob"] program in
  assert (outputs2 = ["What is your name?"; "Hello, Bob!"]);

  (* Test simple programs *)
  let (out, _) = interpret_pure [] (print "hi") in
  assert (out = ["hi"]);

  let (_, v) = interpret_pure ["test"] (get_line ()) in
  assert (v = "test");

  print_endline "✓ All tests passed"
