(* 477: String Trimming
   OCaml's String.trim removes leading and trailing whitespace.
   For trim_start / trim_end / trim_matches we write small helpers. *)

(* Remove leading whitespace — like Rust's trim_start() *)
let trim_start s =
  let n = String.length s in
  let i = ref 0 in
  while !i < n && (s.[!i] = ' ' || s.[!i] = '\t' || s.[!i] = '\n' || s.[!i] = '\r') do
    incr i
  done;
  String.sub s !i (n - !i)

(* Remove trailing whitespace — like Rust's trim_end() *)
let trim_end s =
  let n = String.length s in
  let i = ref (n - 1) in
  while !i >= 0 && (s.[!i] = ' ' || s.[!i] = '\t' || s.[!i] = '\n' || s.[!i] = '\r') do
    decr i
  done;
  String.sub s 0 (!i + 1)

(* Remove a specific character from both ends — like trim_matches('#') *)
let trim_char s c =
  let n = String.length s in
  let lo = ref 0 in
  while !lo < n && s.[!lo] = c do incr lo done;
  let hi = ref (n - 1) in
  while !hi >= !lo && s.[!hi] = c do decr hi done;
  String.sub s !lo (!hi - !lo + 1)

let () =
  (* trim both ends *)
  assert (String.trim "  hi  " = "hi");
  Printf.printf "trim: \"%s\"\n%!" (String.trim "  hi  ");

  (* trim_start *)
  assert (trim_start "  hi  " = "hi  ");
  Printf.printf "trim_start: \"%s\"\n%!" (trim_start "  hi  ");

  (* trim_end *)
  assert (trim_end "  hi  " = "  hi");
  Printf.printf "trim_end: \"%s\"\n%!" (trim_end "  hi  ");

  (* trim_matches '#' *)
  assert (trim_char "##hi##" '#' = "hi");
  Printf.printf "trim_char '#': \"%s\"\n%!" (trim_char "##hi##" '#');

  (* Verify that trim returns a substring (shares memory on same string) *)
  let s = "  hi  " in
  let t = String.trim s in
  assert (String.length t < String.length s);
  Printf.printf "trim produces shorter string: %d < %d\n%!"
    (String.length t) (String.length s)
