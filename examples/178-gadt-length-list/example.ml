(* Example 178: Length-Indexed Lists (Vectors) *)
(* Lists whose length is tracked at the type level *)

(* Approach 1: Peano-encoded length-indexed vector *)
type zero = Zero
type 'n succ = Succ

type ('a, 'n) vec =
  | Nil  : ('a, zero) vec
  | Cons : 'a * ('a, 'n) vec -> ('a, 'n succ) vec

let head : type a n. (a, n succ) vec -> a = function
  | Cons (x, _) -> x
  (* Nil case is impossible — the type prevents it! *)

let tail : type a n. (a, n succ) vec -> (a, n) vec = function
  | Cons (_, xs) -> xs

let rec to_list : type a n. (a, n) vec -> a list = function
  | Nil -> []
  | Cons (x, xs) -> x :: to_list xs

let rec map : type a b n. (a -> b) -> (a, n) vec -> (b, n) vec =
  fun f -> function
    | Nil -> Nil
    | Cons (x, xs) -> Cons (f x, map f xs)

let rec append : type a m n. (a, m) vec -> (a, n) vec -> a list =
  fun v1 v2 -> match v1 with
    | Nil -> to_list v2
    | Cons (x, xs) -> x :: append xs v2

(* Approach 2: Using GADTs for safe zip *)
let rec zip : type a b n. (a, n) vec -> (b, n) vec -> (a * b, n) vec =
  fun v1 v2 -> match v1, v2 with
    | Nil, Nil -> Nil
    | Cons (x, xs), Cons (y, ys) -> Cons ((x, y), zip xs ys)

(* Approach 3: Length witness for runtime operations *)
type _ length =
  | LZ : zero length
  | LS : 'n length -> 'n succ length

let rec replicate : type a n. n length -> a -> (a, n) vec =
  fun n x -> match n with
    | LZ -> Nil
    | LS n' -> Cons (x, replicate n' x)

let () =
  let v1 = Cons (1, Cons (2, Cons (3, Nil))) in
  let v2 = Cons (10, Cons (20, Cons (30, Nil))) in

  assert (head v1 = 1);
  assert (to_list (tail v1) = [2; 3]);
  assert (to_list (map (fun x -> x * 2) v1) = [2; 4; 6]);
  assert (to_list (zip v1 v2) = [(1,10); (2,20); (3,30)]);

  let v3 = replicate (LS (LS (LS LZ))) 42 in
  assert (to_list v3 = [42; 42; 42]);

  print_endline "✓ All tests passed"
