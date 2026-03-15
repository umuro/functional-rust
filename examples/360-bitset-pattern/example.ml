(* 360: BitSet Pattern
   Efficient set operations using a 64-bit integer as a bit vector.
   OCaml's int is 63 bits on 64-bit platforms; we use Int64 for a
   full 64-bit bitset, or native int for a 63-bit one. *)

(* 63-bit bitset using native int (sufficient for indices 0..62) *)
type bitset = { bits: int }

let empty  = { bits = 0 }
let insert b i = assert (i >= 0 && i < 63); { bits = b.bits lor (1 lsl i) }
let remove b i = if i < 0 || i >= 63 then b else { bits = b.bits land (lnot (1 lsl i)) }
let contains b i = i >= 0 && i < 63 && (b.bits lsr i) land 1 = 1
let union        a b = { bits = a.bits lor  b.bits }
let intersection a b = { bits = a.bits land b.bits }
let difference   a b = { bits = a.bits land (lnot b.bits) }

(* popcount via bit trick *)
let count b =
  let n = ref b.bits in
  let c = ref 0 in
  while !n <> 0 do
    n := !n land (!n - 1);  (* clear lowest set bit *)
    incr c
  done;
  !c

let is_empty b = b.bits = 0

let to_list b =
  List.init 63 (fun i -> i)
  |> List.filter (contains b)

let () =
  (* Insert and contains *)
  let b = insert empty 5 in
  assert (contains b 5);
  assert (not (contains b 4));
  Printf.printf "bitset contains 5: %b, contains 4: %b\n%!" (contains b 5) (contains b 4);

  (* Set operations *)
  let a = List.fold_left insert empty [1;2;3] in
  let b = List.fold_left insert empty [2;3;4] in
  let inter = intersection a b |> to_list in
  assert (inter = [2;3]);
  Printf.printf "intersection: %s\n%!"
    (inter |> List.map string_of_int |> String.concat ", ");

  let diff = difference a b |> to_list in
  assert (diff = [1]);
  Printf.printf "difference: %s\n%!"
    (diff |> List.map string_of_int |> String.concat ", ");

  (* Count (popcount) *)
  let c = List.fold_left insert empty (List.init 10 Fun.id) in
  assert (count c = 10);
  Printf.printf "count of 0..9: %d\n%!" (count c);

  (* Remove *)
  let b2 = remove b 5 in
  assert (not (contains b2 5));
  Printf.printf "after remove 5: %b\n%!" (contains b2 5)
