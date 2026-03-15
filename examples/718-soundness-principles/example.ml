(* OCaml: Soundness through module abstraction and type encapsulation.
   OCaml has no "unsafe" keyword — soundness is enforced by the module system
   and the GC. We show the analogous patterns. *)

(* --- Pattern 1: Invariant through private type --- *)

(* A "sorted list" that maintains the sorted invariant via its interface. *)
module SortedList : sig
  type t
  val empty  : t
  val insert : int -> t -> t
  val to_list : t -> int list
  (* There is no public constructor that bypasses the invariant *)
end = struct
  type t = int list (* Invariant: always sorted ascending *)

  let empty = []

  let rec insert x = function
    | []               -> [x]
    | h :: tl when x <= h -> x :: h :: tl
    | h :: tl          -> h :: insert x tl

  let to_list xs = xs
end

let () =
  let s = SortedList.empty
    |> SortedList.insert 5
    |> SortedList.insert 2
    |> SortedList.insert 8
    |> SortedList.insert 1
  in
  Printf.printf "Sorted: %s\n"
    (String.concat ", " (List.map string_of_int (SortedList.to_list s)))

(* --- Pattern 2: Use-after-free is impossible in OCaml --- *)

(* In OCaml, the GC keeps objects alive as long as any reference exists.
   There is no analogue to Rust's dangling pointer. *)
let dangling_demo () =
  let r = ref (Array.make 10 0) in
  let alias = !r in        (* Both point to the same heap object *)
  r := Array.make 10 1;   (* `r` now points to a new array *)
  (* `alias` still safely references the old array — GC won't collect it *)
  Printf.printf "alias[0] = %d (safe, GC keeps it alive)\n" alias.(0)

let () = dangling_demo ()

(* --- Pattern 3: Aliasing in OCaml --- *)

(* OCaml's mutable references can alias — unlike Rust's &mut T — but the GC
   ensures this doesn't lead to UB. Rust's borrow checker prevents aliasing
   &mut references entirely in safe code. *)
let aliasing_demo () =
  let a = ref 42 in
  let b = a in     (* `a` and `b` alias *)
  b := 100;        (* Mutate through `b` *)
  Printf.printf "a = %d (aliased mutation is safe in OCaml, forbidden in Rust)\n" !a

let () = aliasing_demo ()

(* --- Pattern 4: "Safety contract" via documentation --- *)

(* OCaml has no unsafe blocks, but complex C FFI bindings carry the same
   obligation — you must document what the caller must guarantee. *)

(** [unsafe_index arr i] returns [arr.(i)] without bounds checking.
    @raise Undefined_behavior if [i < 0] or [i >= Array.length arr].
    Caller must ensure [0 <= i < Array.length arr]. *)
external unsafe_get : 'a array -> int -> 'a = "%array_unsafe_get"

let () =
  let arr = [|10; 20; 30|] in
  (* We verify the precondition before calling *)
  let i = 1 in
  if i >= 0 && i < Array.length arr then
    Printf.printf "unsafe_get[%d] = %d\n" i (unsafe_get arr i)
