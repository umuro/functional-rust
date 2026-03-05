(* Effect Handlers Preview (OCaml 5) *)
(* Algebraic effects for controlled side effects (OCaml 5+) *)

(* Note: requires OCaml 5.0+ *)
(* This shows the concept; run on OCaml 5 *)
type _ Effect.t += Ask : string Effect.t
type _ Effect.t += Log : string -> unit Effect.t

let program () =
  let name = Effect.perform Ask in
  Effect.perform (Log ("Got name: " ^ name));
  Printf.printf "Hello, %s!\n" name

(* In OCaml 5 you'd install handlers:
   Effect.Deep.try_with program ()
   { effc = fun (type a) (eff : a Effect.t) ->
     match eff with
     | Ask -> Some (fun k -> Effect.Deep.continue k "World")
     | Log msg -> Some (fun k -> print_endline msg; Effect.Deep.continue k ())
     | _ -> None } *)

(* Simulated version for pre-5: *)
let () = Printf.printf "Hello, World! (effect simulation)\n"
