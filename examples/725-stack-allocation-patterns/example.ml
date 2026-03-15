(* OCaml: Stack allocation concepts
   OCaml's GC allocates most values on the heap. Integers and floats
   in local variables are unboxed on the stack. Float arrays have a
   special optimisation path. *)

(* Integers and small tuples: unboxed, stack-allocated *)
let stack_int_ops () =
  let a = 42 in         (* unboxed int — stack *)
  let b = 100 in        (* unboxed int — stack *)
  let sum = a + b in    (* no allocation *)
  Printf.printf "sum=%d (unboxed stack ints)\n" sum

(* Float local variables: unboxed when not in a record or variant *)
let stack_float_ops () =
  let x = 3.14 in
  let y = 2.71 in
  Printf.printf "x+y=%.4f (unboxed floats)\n" (x +. y)

(* Float arrays: OCaml optimises these to unboxed flat arrays *)
let unboxed_float_array () =
  let arr = Array.init 8 (fun i -> float_of_int i *. 1.5) in
  let sum = Array.fold_left (+.) 0.0 arr in
  Printf.printf "float array sum=%.2f (unboxed flat)\n" sum

(* Closest to a stack-allocated byte buffer: local Bytes *)
(* Note: Bytes is heap-allocated in OCaml, but it's the idiom *)
let local_buffer () =
  let buf = Bytes.create 64 in   (* heap, but small and short-lived *)
  let len = ref 0 in
  let push b = Bytes.set buf !len b; incr len in
  List.iter push [Char.chr 72; Char.chr 105];  (* 'H', 'i' *)
  Printf.printf "Buffer: %s\n" (Bytes.sub_string buf 0 !len)

(* Simulate ArrayVec: array with explicit length *)
type 'a arrayvec = { mutable data: 'a array; mutable len: int; cap: int }

let make_arrayvec cap default =
  { data = Array.make cap default; len = 0; cap }

let push_av av x =
  if av.len >= av.cap then failwith "ArrayVec full"
  else begin av.data.(av.len) <- x; av.len <- av.len + 1 end

let slice_av av = Array.sub av.data 0 av.len

let () =
  stack_int_ops ();
  stack_float_ops ();
  unboxed_float_array ();
  local_buffer ();

  let av = make_arrayvec 8 0 in
  List.iter (push_av av) [1; 2; 3; 4; 5];
  Printf.printf "ArrayVec: [%s] len=%d cap=%d\n"
    (String.concat "; " (Array.to_list (Array.map string_of_int (slice_av av))))
    av.len av.cap
