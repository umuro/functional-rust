(* Example 139: HList — Heterogeneous List *)

(* Approach 1: GADT heterogeneous list *)
type hnil = HNil_t
type ('h, 't) hcons = HCons_t

type _ hlist =
  | HNil : hnil hlist
  | HCons : 'a * 'b hlist -> ('a, 'b) hcons hlist

let empty = HNil
let ex1 = HCons (42, HCons ("hello", HCons (true, HNil)))

let hd : type a b. (a, b) hcons hlist -> a = function
  | HCons (x, _) -> x

let tl : type a b. (a, b) hcons hlist -> b hlist = function
  | HCons (_, rest) -> rest

(* Approach 2: Existential list (loses type info) *)
type any = Any : 'a -> any

let any_list = [Any 42; Any "hello"; Any true]

(* Approach 3: Tuple-based HList via nesting *)
type ('a, 'b) pair = { fst : 'a; snd : 'b }

let hlist3 a b c = { fst = a; snd = { fst = b; snd = c } }
let get_first p = p.fst
let get_second p = p.snd.fst
let get_third p = p.snd.snd

(* Tests *)
let () =
  let h = HCons (42, HCons ("hello", HCons (3.14, HNil))) in
  assert (hd h = 42);
  assert (hd (tl h) = "hello");
  assert (hd (tl (tl h)) = 3.14);

  let p = hlist3 1 "two" 3.0 in
  assert (get_first p = 1);
  assert (get_second p = "two");
  assert (get_third p = 3.0);

  Printf.printf "✓ All tests passed\n"
