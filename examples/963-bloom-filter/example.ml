(* 963: Bloom Filter
   A probabilistic data structure: O(1) insert and membership test.
   False positives are possible; false negatives are not.
   Uses k independent hash functions mapping to m-bit array. *)

(* Simple bit array implemented as an int array *)
type bloom_filter = {
  bits : int array;   (* each element holds 63 bits on 64-bit OCaml *)
  m : int;            (* total number of bits *)
  k : int;            (* number of hash functions *)
}

let bits_per_word = Sys.int_size - 1  (* 63 on 64-bit *)

let create ~m ~k =
  assert (m > 0 && k > 0);
  let words = (m + bits_per_word - 1) / bits_per_word in
  { bits = Array.make words 0; m; k }

let set_bit bf i =
  let word = i / bits_per_word in
  let bit  = i mod bits_per_word in
  bf.bits.(word) <- bf.bits.(word) lor (1 lsl bit)

let test_bit bf i =
  let word = i / bits_per_word in
  let bit  = i mod bits_per_word in
  (bf.bits.(word) lsr bit) land 1 = 1

(* k hash positions for a string — uses double hashing: h1 + i*h2 mod m *)
let hash_positions bf s =
  (* Polynomial rolling hashes with two different bases *)
  let h1 = ref 0 and h2 = ref 0 in
  String.iter (fun c ->
    h1 := (!h1 * 31 + Char.code c) land max_int;
    h2 := (!h2 * 37 + Char.code c) land max_int;
  ) s;
  (* Ensure h2 is odd so it stays coprime to m *)
  let h2v = if !h2 = 0 then 1 else !h2 lor 1 in
  Array.init bf.k (fun i -> (!h1 + i * h2v) mod bf.m)

let add bf s =
  Array.iter (set_bit bf) (hash_positions bf s)

let might_contain bf s =
  Array.for_all (test_bit bf) (hash_positions bf s)

(* Estimate the current false-positive rate given n items inserted *)
let false_positive_rate bf n =
  (* (1 - e^(-k*n/m))^k *)
  let k = float_of_int bf.k in
  let m = float_of_int bf.m in
  let exponent = -. k *. float_of_int n /. m in
  (1.0 -. exp exponent) ** k

let () =
  (* m=1000, k=3 → ~1% FP rate at ~100 items *)
  let bf = create ~m:1000 ~k:3 in
  let words = ["apple"; "banana"; "cherry"; "date"; "elderberry"] in
  List.iter (add bf) words;

  Printf.printf "Membership tests (should all be true):\n";
  List.iter (fun w ->
    Printf.printf "  %s: %b\n" w (might_contain bf w)
  ) words;

  Printf.printf "\nNon-members (expect false, possibly true with low probability):\n";
  let non_members = ["fig"; "grape"; "kiwi"; "lemon"; "mango"] in
  List.iter (fun w ->
    Printf.printf "  %s: %b\n" w (might_contain bf w)
  ) non_members;

  Printf.printf "\nFalse positive rate at 5 items: %.4f\n"
    (false_positive_rate bf 5);

  (* Demonstrate with a larger set *)
  let bf2 = create ~m:10000 ~k:5 in
  for i = 0 to 999 do
    add bf2 (string_of_int i)
  done;
  Printf.printf "FP rate at 1000 items (m=10000, k=5): %.4f\n"
    (false_positive_rate bf2 1000)
