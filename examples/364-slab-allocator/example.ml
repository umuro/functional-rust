(* OCaml: slab-like pool with indices *)

type 'a slab = {
  mutable data: 'a option array;
  mutable free: int list;
  mutable next_id: int;
}

let make cap = { data=Array.make cap None; free=[]; next_id=0 }

let insert s v =
  match s.free with
  | i::rest -> s.data.(i) <- Some v; s.free <- rest; i
  | [] ->
    let i = s.next_id in
    s.data.(i) <- Some v; s.next_id <- i+1; i

let get s i = s.data.(i)
let remove s i = s.data.(i) <- None; s.free <- i :: s.free

let () =
  let s = make 8 in
  let k1 = insert s "hello" in
  let k2 = insert s "world" in
  Printf.printf "k1=%d: %s\n" k1 (Option.get (get s k1));
  remove s k1;
  let k3 = insert s "reused" in
  Printf.printf "k3=%d (reused slot): %s\n" k3 (Option.get (get s k3))
