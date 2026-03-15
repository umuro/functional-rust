(* 977: Bitset
   A compact set of non-negative integers backed by an integer array.
   Each word holds 63 bits (on 64-bit OCaml, int is 63 bits due to the tag bit).
   Supports O(1) member/add/remove and O(n/w) union/intersection. *)

let bits_per_word = Sys.int_size - 1   (* 63 on 64-bit *)

type bitset = {
  mutable words : int array;
  mutable cap   : int;   (* number of bits we can hold *)
}

let create ?(capacity=64) () =
  let words = (capacity + bits_per_word - 1) / bits_per_word in
  { words = Array.make words 0; cap = words * bits_per_word }

(* Grow if needed *)
let ensure_capacity bs i =
  if i >= bs.cap then begin
    let need_words = (i + bits_per_word) / bits_per_word in
    let new_words = Array.make need_words 0 in
    Array.blit bs.words 0 new_words 0 (Array.length bs.words);
    bs.words <- new_words;
    bs.cap   <- need_words * bits_per_word
  end

let add bs i =
  ensure_capacity bs i;
  bs.words.(i / bits_per_word) <- bs.words.(i / bits_per_word) lor (1 lsl (i mod bits_per_word))

let remove bs i =
  if i < bs.cap then
    bs.words.(i / bits_per_word) <- bs.words.(i / bits_per_word) land (lnot (1 lsl (i mod bits_per_word)))

let mem bs i =
  if i >= bs.cap then false
  else bs.words.(i / bits_per_word) land (1 lsl (i mod bits_per_word)) <> 0

(* Count set bits (Kernighan's method per word) *)
let popcount w =
  let w = ref w and c = ref 0 in
  while !w <> 0 do incr c; w := !w land (!w - 1) done;
  !c

let count bs = Array.fold_left (fun acc w -> acc + popcount w) 0 bs.words

(* Iterate over all set bits *)
let iter f bs =
  Array.iteri (fun wi word ->
    let w = ref word and base = wi * bits_per_word in
    let bit = ref 0 in
    while !w <> 0 do
      (* find lowest set bit position *)
      while !w land (1 lsl !bit) = 0 do incr bit done;
      f (base + !bit);
      w := !w land (!w - 1);
      incr bit
    done
  ) bs.words

let to_list bs =
  let acc = ref [] in
  iter (fun i -> acc := i :: !acc) bs;
  List.rev !acc

(* Set operations — pairwise on words *)
let pairwise_op op a b =
  let n = max (Array.length a.words) (Array.length b.words) in
  let r = { words = Array.make n 0; cap = n * bits_per_word } in
  for i = 0 to n - 1 do
    let wa = if i < Array.length a.words then a.words.(i) else 0 in
    let wb = if i < Array.length b.words then b.words.(i) else 0 in
    r.words.(i) <- op wa wb
  done;
  r

let union        a b = pairwise_op ( lor  ) a b
let inter        a b = pairwise_op ( land ) a b
let difference   a b = pairwise_op (fun x y -> x land (lnot y)) a b
let sym_diff     a b = pairwise_op ( lxor ) a b

let subset a b =
  (* a ⊆ b iff a AND NOT b = 0 *)
  let d = difference a b in
  count d = 0

let of_list lst =
  let bs = create () in
  List.iter (add bs) lst;
  bs

let () =
  let a = of_list [0; 1; 3; 5; 7; 9] in
  let b = of_list [0; 2; 4; 6; 8; 9] in

  Printf.printf "a = {%s}\n" (String.concat "," (List.map string_of_int (to_list a)));
  Printf.printf "b = {%s}\n" (String.concat "," (List.map string_of_int (to_list b)));

  Printf.printf "mem a 3 = %b, mem a 4 = %b\n" (mem a 3) (mem a 4);
  Printf.printf "|a| = %d, |b| = %d\n" (count a) (count b);

  let u = union a b in
  Printf.printf "a ∪ b = {%s}\n" (String.concat "," (List.map string_of_int (to_list u)));

  let i = inter a b in
  Printf.printf "a ∩ b = {%s}\n" (String.concat "," (List.map string_of_int (to_list i)));

  let d = difference a b in
  Printf.printf "a \\ b = {%s}\n" (String.concat "," (List.map string_of_int (to_list d)));

  let sd = sym_diff a b in
  Printf.printf "a △ b = {%s}\n" (String.concat "," (List.map string_of_int (to_list sd)));

  let c = of_list [1; 3] in
  Printf.printf "subset {1,3} ⊆ a: %b\n" (subset c a);
  Printf.printf "subset {1,3} ⊆ b: %b\n" (subset c b);

  remove a 3;
  Printf.printf "after remove 3: {%s}\n" (String.concat "," (List.map string_of_int (to_list a)))
