(* Mutable Records and Arrays *)
(* Mutable record fields for stateful objects *)

type counter = {
  mutable count : int;
  name : string;
}

let make_counter name = { count = 0; name }

let increment c = c.count <- c.count + 1
let reset c = c.count <- 0
let value c = c.count

let c = make_counter "clicks"
let () =
  for _ = 1 to 10 do increment c done;
  Printf.printf "%s: %d\n" c.name (value c);
  reset c;
  Printf.printf "After reset: %d\n" (value c)
