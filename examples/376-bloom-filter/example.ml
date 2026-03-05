(* OCaml: bloom filter with bitarray *)

let m = 1024  (* bits *)
let bits = Bytes.make (m/8) '\000'

let set_bit i = Bytes.set bits (i/8) (Char.chr (Char.code (Bytes.get bits (i/8)) lor (1 lsl (i mod 8))))
let get_bit i = (Char.code (Bytes.get bits (i/8)) lsr (i mod 8)) land 1 = 1

let hash1 s = (Hashtbl.hash s) mod m
let hash2 s = (Hashtbl.hash (s ^ "salt")) mod m
let hash3 s = (Hashtbl.hash ("x" ^ s)) mod m

let add s = set_bit (hash1 s); set_bit (hash2 s); set_bit (hash3 s)
let might_contain s = get_bit (hash1 s) && get_bit (hash2 s) && get_bit (hash3 s)

let () =
  List.iter add ["alice";"bob";"charlie"];
  Printf.printf "alice: %b (should be true)\n" (might_contain "alice");
  Printf.printf "dave: %b (should be false, maybe fp)\n" (might_contain "dave")
