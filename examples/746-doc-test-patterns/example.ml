(* 746: Documentation Tests — OCaml odoc examples
   ODDoc supports @example annotations but they are NOT executed.
   This is a key advantage Rust has over OCaml. *)

(**
  Clamp a value within [lo, hi].

  @example
  {[
    let _ = clamp 0 10 (-5)  (* = 0 *)
    let _ = clamp 0 10 5     (* = 5 *)
    let _ = clamp 0 10 15    (* = 10 *)
  ]}
*)
let clamp lo hi x = max lo (min hi x)

(**
  Repeat a string n times.

  @example
  {[
    let _ = repeat "ab" 3  (* = "ababab" *)
    let _ = repeat "x" 0   (* = "" *)
  ]}
*)
let repeat s n =
  let buf = Buffer.create (String.length s * n) in
  for _ = 1 to n do Buffer.add_string buf s done;
  Buffer.contents buf

(**
  Split a string by delimiter (first occurrence).

  @example
  {[
    let _ = split_once ':' "key:value"  (* = Some ("key", "value") *)
    let _ = split_once ':' "nodelim"    (* = None *)
  ]}
*)
let split_once delim s =
  match String.index_opt s delim with
  | None -> None
  | Some i ->
    let left  = String.sub s 0 i in
    let right = String.sub s (i + 1) (String.length s - i - 1) in
    Some (left, right)

let () =
  assert (clamp 0 10 (-5) = 0);
  assert (repeat "ab" 3 = "ababab");
  assert (split_once ':' "key:value" = Some ("key", "value"));
  Printf.printf "OCaml examples verified (manually — odoc doesn't run them)\n"
