(* Free monad in OCaml *)
type ('f, 'a) free =
  | Pure of 'a
  | Free of ('f * (('f,'a) free))  (* simplified *)

(* Console language *)
type 'a console_f =
  | Print of string * 'a
  | ReadLine of (string -> 'a)

type 'a console = ('a console_f, 'a) free

let print_line s k = Free (Print (s, Pure ()), (fun () -> k ()))
let read_line k    = Free (ReadLine k, (fun _ -> Pure ()))

(* Simplified: just describe the program *)
type program =
  | Print of string * program
  | Read  of (string -> program)
  | Done

let rec interpret log_buf = function
  | Done       -> ()
  | Print(s,k) -> Buffer.add_string log_buf (s^"\n"); interpret log_buf k
  | Read f     -> interpret log_buf (f "simulated-input")

let () =
  let prog =
    Print("What is your name?",
    Read(fun name ->
    Print(Printf.sprintf "Hello, %s!" name,
    Done))) in
  let buf = Buffer.create 64 in
  interpret buf prog;
  print_string (Buffer.contents buf)
