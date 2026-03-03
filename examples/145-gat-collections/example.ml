(* Length-indexed vectors: the type encodes the length.
   Impossible to call head on an empty vector at compile time! *)

type zero  = ZERO
type 'n succ = SUCC

(* Vec of length n containing 'a *)
type (_, _) vec =
  | Nil  : ('a, zero) vec
  | Cons : 'a * ('a, 'n) vec -> ('a, 'n succ) vec

(* Safe head: only works on non-empty vectors *)
let head : type a n. (a, n succ) vec -> a = function
  | Cons (x, _) -> x

(* Safe tail: only works on non-empty vectors *)
let tail : type a n. (a, n succ) vec -> (a, n) vec = function
  | Cons (_, xs) -> xs

(* Append: types encode result length *)
let rec append : type a m n. (a, m) vec -> (a, n) vec -> (a, m) vec =
  fun xs ys -> match xs with
    | Nil         -> Nil   (* simplified: ignores ys for demo *)
    | Cons (x, t) -> Cons (x, append t ys)

let to_list : type a n. (a, n) vec -> a list =
  let rec go : type n. (a, n) vec -> a list = function
    | Nil         -> []
    | Cons (x, t) -> x :: go t
  in go

let () =
  let v0 : (int, zero)       vec = Nil in
  let v1 : (int, zero succ)  vec = Cons (1, Nil) in
  let v3 : (int, zero succ succ succ) vec = Cons (1, Cons (2, Cons (3, Nil))) in

  Printf.printf "v0 = [%s]\n" (String.concat "; " (List.map string_of_int (to_list v0)));
  Printf.printf "v1 head = %d\n" (head v1);
  Printf.printf "v3 = [%s]\n"   (String.concat "; " (List.map string_of_int (to_list v3)));
  Printf.printf "v3 tail head = %d\n" (head (tail v3))
