(* 963: Bloom Filter *)
(* Probabilistic set membership: may have false positives, no false negatives *)

(* Simple hash functions using djb2 and sdbm *)

let hash1 s =
  String.fold_left (fun h c -> h * 31 + Char.code c) 5381 s

let hash2 s =
  String.fold_left (fun h c -> (Char.code c) + (h lsl 6) + (h lsl 16) - h) 0 s

let hash3 s =
  String.fold_left (fun h c -> h * 33 lxor Char.code c) 0 s

(* Approach 1: Bloom filter with fixed bit array *)

type bloom_filter = {
  bits: bool array;
  size: int;
}

let create size = { bits = Array.make size false; size }

let add bf s =
  let h1 = abs (hash1 s) mod bf.size in
  let h2 = abs (hash2 s) mod bf.size in
  let h3 = abs (hash3 s) mod bf.size in
  bf.bits.(h1) <- true;
  bf.bits.(h2) <- true;
  bf.bits.(h3) <- true

let might_contain bf s =
  let h1 = abs (hash1 s) mod bf.size in
  let h2 = abs (hash2 s) mod bf.size in
  let h3 = abs (hash3 s) mod bf.size in
  bf.bits.(h1) && bf.bits.(h2) && bf.bits.(h3)

(* Approach 2: Compact bit representation using int array *)

let create_compact num_bits =
  let words = (num_bits + 62) / 63 in
  Array.make words 0

let set_bit bits i =
  let word = i / 63 in
  let bit = i mod 63 in
  bits.(word) <- bits.(word) lor (1 lsl bit)

let get_bit bits i =
  let word = i / 63 in
  let bit = i mod 63 in
  (bits.(word) lsr bit) land 1 = 1

let () =
  let bf = create 1024 in

  add bf "apple";
  add bf "banana";
  add bf "cherry";

  (* All added items must be found *)
  assert (might_contain bf "apple");
  assert (might_contain bf "banana");
  assert (might_contain bf "cherry");

  (* These might or might not be found (false positives possible) *)
  (* but very unlikely with 1024 bits and 3 hashes for these strings *)
  let likely_absent = not (might_contain bf "dragon") in
  let _ = likely_absent in (* suppressing: could be false positive *)

  (* Compact bit array test *)
  let bits = create_compact 128 in
  set_bit bits 0;
  set_bit bits 63;
  set_bit bits 127;
  assert (get_bit bits 0);
  assert (get_bit bits 63);
  assert (get_bit bits 127);
  assert (not (get_bit bits 1));
  assert (not (get_bit bits 64));

  Printf.printf "✓ All tests passed\n"
