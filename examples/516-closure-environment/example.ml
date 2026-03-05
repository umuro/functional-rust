(* Complex closure environments in OCaml *)

(* Closure capturing multiple fields *)
type config = {
  prefix: string;
  max_len: int;
  transform: string -> string;
}

let make_formatter cfg =
  fun s ->
    let truncated = if String.length s > cfg.max_len
      then String.sub s 0 cfg.max_len ^ "..."
      else s
    in
    cfg.transform (cfg.prefix ^ truncated)

(* Closure over a list and a counter *)
let make_cycler items =
  let arr = Array.of_list items in
  let i = ref 0 in
  fun () ->
    let v = arr.(!i) in
    i := (!i + 1) mod Array.length arr;
    v

(* Nested closure capturing outer environment *)
let make_multiplier_factory base =
  fun factor ->
    let combined = base * factor in
    fun x -> x * combined

let () =
  let fmt = make_formatter {
    prefix = "[INFO] ";
    max_len = 10;
    transform = String.uppercase_ascii;
  } in
  Printf.printf "%s\n" (fmt "hello world this is long");
  Printf.printf "%s\n" (fmt "hi");

  let cycle = make_cycler ["red"; "green"; "blue"] in
  for _ = 1 to 7 do
    Printf.printf "%s " (cycle ())
  done;
  print_newline ();

  let times10 = make_multiplier_factory 5 2 in
  Printf.printf "times10(3) = %d\n" (times10 3)
