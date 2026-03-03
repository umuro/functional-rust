(* Operational monad: instructions are a GADT; programs build the sequence.
   Similar to free monad but more explicit about the instruction set. *)

(* Instruction set for a simple IO DSL *)
type _ instr =
  | Read  : string instr
  | Write : string -> unit instr

(* Programs: sequence of instructions *)
type _ prog =
  | Return : 'a -> 'a prog
  | Instr  : 'a instr * ('a -> 'b prog) -> 'b prog

let return x     = Return x
let read ()      = Instr (Read,    return)
let write s      = Instr (Write s, fun () -> Return ())

let rec bind m f = match m with
  | Return x         -> f x
  | Instr (i, cont)  -> Instr (i, fun x -> bind (cont x) f)

(* Pure interpreter using a list as stdin *)
let run_pure inputs prog =
  let buf = Buffer.create 64 in
  let inputs = ref inputs in
  let rec go : type a. a prog -> a = function
    | Return x -> x
    | Instr (Read, cont) ->
      let line = List.hd !inputs in
      inputs := List.tl !inputs;
      go (cont line)
    | Instr (Write s, cont) ->
      Buffer.add_string buf (s ^ "\n");
      go (cont ())
  in
  (go prog, Buffer.contents buf)

let () =
  let prog =
    bind (write "Enter name:") (fun () ->
    bind (read ())              (fun name ->
    bind (write ("Hello, " ^ name ^ "!")) (fun () ->
    return name)))
  in
  let (result, output) = run_pure ["Alice"] prog in
  Printf.printf "Output:\n%s" output;
  Printf.printf "Result: %s\n" result
