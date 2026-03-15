(* 977: Bitset Operations *)
(* Set, clear, toggle, union, intersection using u64 word arrays *)

let words_for_bits n = (n + 62) / 63  (* OCaml int is 63-bit on 64-bit systems *)

type bitset = {
  bits: int array;
  size: int;
}

let create size = { bits = Array.make (words_for_bits size) 0; size }

let word_index i = i / 63
let bit_index i = i mod 63

let set bs i =
  if i >= bs.size then failwith "out of range";
  bs.bits.(word_index i) <- bs.bits.(word_index i) lor (1 lsl bit_index i)

let clear bs i =
  if i >= bs.size then failwith "out of range";
  bs.bits.(word_index i) <- bs.bits.(word_index i) land (lnot (1 lsl bit_index i))

let toggle bs i =
  if i >= bs.size then failwith "out of range";
  bs.bits.(word_index i) <- bs.bits.(word_index i) lxor (1 lsl bit_index i)

let test bs i =
  if i >= bs.size then false
  else (bs.bits.(word_index i) lsr bit_index i) land 1 = 1

let popcount w =
  let w = ref w in
  let count = ref 0 in
  while !w <> 0 do
    w := !w land (!w - 1);  (* clear lowest set bit *)
    incr count
  done;
  !count

let count bs =
  Array.fold_left (fun acc w -> acc + popcount w) 0 bs.bits

let union a b =
  assert (a.size = b.size);
  let result = create a.size in
  for i = 0 to Array.length a.bits - 1 do
    result.bits.(i) <- a.bits.(i) lor b.bits.(i)
  done;
  result

let intersect a b =
  assert (a.size = b.size);
  let result = create a.size in
  for i = 0 to Array.length a.bits - 1 do
    result.bits.(i) <- a.bits.(i) land b.bits.(i)
  done;
  result

let difference a b =
  assert (a.size = b.size);
  let result = create a.size in
  for i = 0 to Array.length a.bits - 1 do
    result.bits.(i) <- a.bits.(i) land (lnot b.bits.(i))
  done;
  result

let () =
  let bs = create 64 in
  set bs 0;
  set bs 5;
  set bs 10;
  set bs 63;

  assert (test bs 0);
  assert (test bs 5);
  assert (test bs 10);
  assert (test bs 63);
  assert (not (test bs 1));
  assert (count bs = 4);

  clear bs 5;
  assert (not (test bs 5));
  assert (count bs = 3);

  toggle bs 5;
  assert (test bs 5);
  toggle bs 5;
  assert (not (test bs 5));

  let a = create 64 in
  let b = create 64 in
  List.iter (set a) [0; 1; 2; 3];
  List.iter (set b) [2; 3; 4; 5];

  let u = union a b in
  assert (count u = 6);

  let i = intersect a b in
  assert (count i = 2);
  assert (test i 2);
  assert (test i 3);

  let d = difference a b in
  assert (count d = 2);
  assert (test d 0);
  assert (test d 1);

  Printf.printf "✓ All tests passed\n"
