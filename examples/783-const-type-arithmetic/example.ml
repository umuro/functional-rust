(* Type-level arithmetic in OCaml with GADTs *)

(* Peano naturals at the type level *)
type zero = Zero
type 'n succ = Succ

(* Length-indexed vectors *)
type ('a, 'n) vec =
  | Nil  : ('a, zero) vec
  | Cons : 'a * ('a, 'n) vec -> ('a, 'n succ) vec

(* Type-safe head: only works on non-empty vectors *)
let head (Cons (x, _)) = x

(* Type-safe tail *)
let tail (Cons (_, xs)) = xs

(* Append: length is sum of input lengths *)
let rec append : type a n m. (a, n) vec -> (a, m) vec -> (a, (* n+m *) _) vec =
  fun xs ys ->
  match xs with
  | Nil -> ys
  | Cons (x, rest) -> Cons (x, append rest ys)

(* Replicate: create a vec of length n *)
let rec replicate : type n. int -> (int, n) vec -> (int, n) vec =
  fun _default v -> v  (* simplified — real impl needs type witness *)

(* Length via type-level computation *)
let rec length : type a n. (a, n) vec -> int = function
  | Nil -> 0
  | Cons (_, rest) -> 1 + length rest

let v3 = Cons (1, Cons (2, Cons (3, Nil)))
let v2 = Cons (4, Cons (5, Nil))
let v5 = append v3 v2

let () =
  Printf.printf "head(v3) = %d\n" (head v3);
  Printf.printf "len(v3)  = %d\n" (length v3);
  Printf.printf "len(v2)  = %d\n" (length v2);
  Printf.printf "len(v3++v2) = %d\n" (length v5)
