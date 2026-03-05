(* OCaml: manual future-like with continuations *)

type 'a state = Pending of (unit -> 'a state) | Ready of 'a

let rec run = function
  | Ready v -> v
  | Pending f -> run (f ())

let delayed_value n steps =
  let rec loop i =
    if i = 0 then Ready n
    else Pending (fun () -> loop (i-1))
  in loop steps

let () =
  Printf.printf "Got: %d\n" (run (delayed_value 42 3))
