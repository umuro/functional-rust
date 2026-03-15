(* 139: Heterogeneous List (HList)
   OCaml's GADTs let us build type-safe heterogeneous lists natively.
   The full element type is preserved in the list's type signature. *)

(* ── Core GADT ────────────────────────────────────────────────────────────── *)

(* 'a hlist encodes the types of all elements at the type level:
   - hnil has type unit hlist
   - hcons h t has type (h * t_shape) hlist  *)
type _ hlist =
  | HNil  : unit hlist
  | HCons : 'h * 't hlist -> ('h * 't) hlist

(* ── Smart constructors ───────────────────────────────────────────────────── *)

let hnil = HNil
let hcons h t = HCons (h, t)

(* ── Accessors ────────────────────────────────────────────────────────────── *)

let hhead (HCons (h, _)) = h
let htail (HCons (_, t)) = t

(* ── Compile-time length via type class ──────────────────────────────────── *)

(* We compute length at runtime, but the type guarantees correctness *)
let rec hlength : type a. a hlist -> int = function
  | HNil       -> 0
  | HCons (_, t) -> 1 + hlength t

(* ── Map collecting results into a list (uniform output type) ─────────────── *)

(* Because OCaml lists must be uniform, we map to a common type R.
   The function receives each element as a polymorphic argument via a record. *)
type 'r mapper = { f : 'a. 'a -> 'r }

let rec hmap_collect : type a r. r mapper -> a hlist -> r list =
  fun m -> function
  | HNil         -> []
  | HCons (h, t) -> m.f h :: hmap_collect m t

(* ── Fold over elements that share a class via a type constraint ──────────── *)

(* Fold where every element must satisfy a constraint recorded in the list.
   We use a constraint list that mirrors the hlist structure. *)
type 'a to_int = ToInt : ('a -> int) -> 'a to_int

type _ int_witnesses =
  | WNil  : unit int_witnesses
  | WCons : 'h to_int * 't int_witnesses -> ('h * 't) int_witnesses

let rec hfold_int : type a. int -> (int -> int -> int) -> a int_witnesses -> a hlist -> int =
  fun init f witnesses lst ->
  match witnesses, lst with
  | WNil, HNil -> init
  | WCons (ToInt conv, wt), HCons (h, t) ->
    hfold_int (f init (conv h)) f wt t

(* ── Demo ─────────────────────────────────────────────────────────────────── *)

let () =
  (* Build a heterogeneous list: int * (string * (bool * unit)) hlist *)
  let lst = hcons 42 (hcons "hello" (hcons true hnil)) in

  Printf.printf "length = %d\n" (hlength lst);
  Printf.printf "head   = %d\n" (hhead lst);
  Printf.printf "tail.head = %s\n" (hhead (htail lst));
  Printf.printf "tail.tail.head = %b\n" (hhead (htail (htail lst)));

  (* Map every element to its string representation *)
  let strings = hmap_collect { f = fun x -> Printf.sprintf "%s" (Obj.repr x |> ignore; "?") } lst in
  (* More practical: use format_any via show functions *)
  let show_lst = hcons 1 (hcons 2 (hcons 3 hnil)) in
  let strs = hmap_collect { f = (fun x -> string_of_int (Obj.magic x)) } show_lst in
  Printf.printf "mapped ints: [%s]\n" (String.concat "; " strs);
  ignore strings;

  (* Fold an int hlist to sum *)
  let int_lst = hcons 10 (hcons 20 (hcons 30 hnil)) in
  let witnesses = WCons (ToInt Fun.id, WCons (ToInt Fun.id, WCons (ToInt Fun.id, WNil))) in
  let sum = hfold_int 0 ( + ) witnesses int_lst in
  Printf.printf "sum [10;20;30] = %d\n" sum;

  (* Compile-time type distinctness:
     int * (string * unit) hlist  ≠  string * (int * unit) hlist
     Each is a different OCaml type — swapping is a type error. *)
  let _a : (int * (string * unit)) hlist = hcons 1 (hcons "a" hnil) in
  let _b : (string * (int * unit)) hlist = hcons "a" (hcons 1 hnil) in
  Printf.printf "type distinctness: a.head=%d  b.head=%s\n"
    (hhead _a) (hhead _b)
