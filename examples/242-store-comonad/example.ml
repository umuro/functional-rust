(* Store comonad: models a mutable store.
   Store s a = (s -> a) * s
   extract: read current value
   extend: transform the getter, keeping the store *)

type ('s, 'a) store = Store of ('s -> 'a) * 's

let pos   (Store (_, s)) = s
let peek  (Store (f, _)) s = f s
let extract (Store (f, s)) = f s

let extend (Store (f, s)) g =
  Store ((fun s' -> g (Store (f, s'))), s)

let duplicate store =
  extend store (fun s -> s)

(* A 1D cellular automaton via Store comonad! *)
(* The store is: int (position) -> bool (alive/dead), with current focus *)

let make_store arr pos =
  Store ((fun i ->
    let len = Array.length arr in
    let i' = ((i mod len) + len) mod len in
    arr.(i')
  ), pos)

let rule30 store =
  let left  = peek store (pos store - 1) in
  let cur   = extract store in
  let right = peek store (pos store + 1) in
  match (left, cur, right) with
  | (true,  true,  true)  -> false
  | (true,  true,  false) -> false
  | (true,  false, true)  -> false
  | (true,  false, false) -> true
  | (false, true,  true)  -> true
  | (false, true,  false) -> true
  | (false, false, true)  -> true
  | (false, false, false) -> false

let step arr =
  let len = Array.length arr in
  let store = make_store arr 0 in
  Array.init len (fun i ->
    let s = extend store (fun _ -> ()) in
    let s2 = Store ((fun i' ->
      let (Store (f, _)) = store in f i'), i) in
    rule30 s2
  )

let print_gen arr =
  Array.iter (fun b -> print_char (if b then '#' else ' ')) arr;
  print_newline ()

let () =
  let size = 21 in
  let gen = Array.make size false in
  gen.(size / 2) <- true;
  print_gen gen;
  let g1 = step gen in print_gen g1;
  let g2 = step g1  in print_gen g2;
  Printf.printf "Store comonad: cellular automaton\n"
