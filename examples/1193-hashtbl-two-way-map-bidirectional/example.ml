(* Hashtbl — Two-way Map (Bidirectional) *)
(* Maintain a bidirectional mapping with two hash tables *)

type ('a, 'b) bimap = {
  forward : ('a, 'b) Hashtbl.t;
  backward : ('b, 'a) Hashtbl.t;
}

let create n = { forward = Hashtbl.create n; backward = Hashtbl.create n }

let add bm k v =
  Hashtbl.replace bm.forward k v;
  Hashtbl.replace bm.backward v k

let find_forward bm k = Hashtbl.find bm.forward k
let find_backward bm v = Hashtbl.find bm.backward v

let bm = create 8
let () =
  add bm "one" 1; add bm "two" 2; add bm "three" 3;
  Printf.printf "two -> %d\n" (find_forward bm "two");
  Printf.printf "3 -> %s\n" (find_backward bm 3)
