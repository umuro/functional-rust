(* OCaml: Closest analog to MaybeUninit is using option types or Obj.magic.
   OCaml's GC always initialises heap values, so "uninit" is not a normal concept.
   We show the two patterns: optional deferred init, and unsafe Obj.magic. *)

(* --- Pattern 1: Deferred initialisation via option --- *)

(* This is the safe, idiomatic OCaml approach *)
type 'a maybe_uninit = Uninit | Init of 'a

let write mu v = match mu with
  | Uninit | Init _ -> Init v

let assume_init = function
  | Init v -> v
  | Uninit -> failwith "assume_init called on uninitialised value!"

let () =
  let slot : int maybe_uninit ref = ref Uninit in
  (* Later, initialise it *)
  slot := write !slot 42;
  let value = assume_init !slot in
  Printf.printf "Initialised value: %d\n" value

(* --- Pattern 2: Fixed-size buffer initialisation --- *)

(* Build a length-5 array by writing elements one by one *)
let init_array_stepwise () =
  let arr = Array.make 5 0 in  (* OCaml requires an init value *)
  (* Simulate writing each element *)
  for i = 0 to 4 do
    arr.(i) <- i * i
  done;
  arr

let () =
  let a = init_array_stepwise () in
  Printf.printf "Squares: %s\n"
    (String.concat " " (Array.to_list (Array.map string_of_int a)))

(* --- Pattern 3: FFI output parameter (simulated) --- *)

(* C functions often take a pointer and write the result through it.
   In OCaml, we pass a ref and let the function mutate it. *)
let c_like_get_value (out : int ref) : bool =
  out := 99;  (* "write through pointer" *)
  true        (* success *)

let () =
  let out = ref 0 in
  let ok = c_like_get_value out in
  if ok then Printf.printf "Got value from C: %d\n" !out

(* --- Pattern 4: Obj.magic (the dangerous uninit equivalent) --- *)
(* Never do this in real code! Shown only for comparison. *)
let _dangerous_uninit () : int =
  Obj.magic (Obj.repr 0)  (* "uninit" by casting — UB if not filled before read *)

(* The safe lesson: in OCaml you can't easily get genuinely uninit memory.
   In Rust, MaybeUninit makes this explicit and safe to reason about. *)
let () = Printf.printf "MaybeUninit concept demo complete.\n"
