(* 970: Rope String *)
(* Tree-based string structure for O(log n) concatenation *)
(* Naive concat is O(1), but split/index are O(log n) *)

type rope =
  | Leaf of string
  | Node of rope * rope * int  (* left, right, total_length *)

let length = function
  | Leaf s -> String.length s
  | Node (_, _, n) -> n

let concat r1 r2 =
  Node (r1, r2, length r1 + length r2)

let of_string s = Leaf s

(* Convert rope back to string *)
let rec to_string = function
  | Leaf s -> s
  | Node (l, r, _) -> to_string l ^ to_string r

(* Index: get character at position i *)
let rec index rope i =
  match rope with
  | Leaf s ->
    if i >= 0 && i < String.length s then Some s.[i]
    else None
  | Node (l, r, _) ->
    let ln = length l in
    if i < ln then index l i
    else index r (i - ln)

(* Split rope into (left, right) at position i *)
let rec split rope i =
  match rope with
  | Leaf s ->
    let n = String.length s in
    let i = max 0 (min i n) in
    (Leaf (String.sub s 0 i), Leaf (String.sub s i (n - i)))
  | Node (l, r, _) ->
    let ln = length l in
    if i <= ln then
      let (ll, lr) = split l i in
      (ll, concat lr r)
    else
      let (rl, rr) = split r (i - ln) in
      (concat l rl, rr)

(* Substring extraction *)
let sub rope start len =
  let (_, right) = split rope start in
  let (mid, _) = split right len in
  to_string mid

let () =
  let r1 = of_string "Hello" in
  let r2 = of_string ", " in
  let r3 = of_string "World" in
  let r4 = of_string "!" in

  let rope = concat (concat r1 r2) (concat r3 r4) in
  assert (length rope = 13);
  assert (to_string rope = "Hello, World!");

  assert (index rope 0 = Some 'H');
  assert (index rope 7 = Some 'W');
  assert (index rope 12 = Some '!');
  assert (index rope 13 = None);

  assert (sub rope 7 5 = "World");
  assert (sub rope 0 5 = "Hello");

  let (left, right) = split rope 7 in
  assert (to_string left = "Hello, ");
  assert (to_string right = "World!");

  Printf.printf "✓ All tests passed\n"
