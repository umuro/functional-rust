# Functional Rust — Example Queue

This file tracks OCaml examples waiting to be converted to Rust.

## Status legend
- `[ ]` pending — not yet processed by Claude Code
- `[x]` done — converted, quality-checked, on the site

## Sources being mined
- OCaml 99 Problems (examples 001-056 in examples/ dir)
- Real World OCaml (O'Reilly book by Minsky, Madhavapeddy, Hickey)
- Cornell CS3110 — Functional Programming in OCaml course
- OCaml.org official tutorials
- Exercism OCaml track
- Functional Programming in OCaml MOOC (France Université Numérique)
- OCaml Standard Library patterns (List, Map, Set, Hashtbl, Option, Result)
- Jane Street Core library patterns
- Lwt / Async concurrency patterns
- ppx metaprogramming examples

---

## Pending (to be added by research job)

<!-- Research job appends items here in format:
### NNN: [Title]
**Source:** [URL or book/chapter]
**Topic:** [brief description]
**OCaml:**
```ocaml
[code]
```
**Status:** [ ]
-->

---
<!-- Added by OCaml Hunt cron — 2026-03-02 -->

### 001: Applying a Function Twice
**Source:** https://cs3110.github.io/textbook/chapters/hop/higher_order.html
**Topic:** Higher-order function that takes a function and applies it twice to a value, demonstrating currying and partial application
**Difficulty:** Beginner
**Category:** Higher-Order Functions
**OCaml:**
```ocaml
let twice f x = f (f x)

let double x = 2 * x
let square x = x * x

let quad   = twice double   (* applies double twice *)
let fourth = twice square   (* applies square twice *)

let () =
  Printf.printf "quad 3   = %d\n" (quad 3);    (* 12 *)
  Printf.printf "fourth 2 = %d\n" (fourth 2)   (* 16 *)
```
**Status:** [x]

---

### 002: Function Composition
**Source:** https://cs3110.github.io/textbook/chapters/hop/higher_order.html
**Topic:** Composing two functions into a single pipeline; demonstrates how OCaml functions are first-class values
**Difficulty:** Beginner
**Category:** Higher-Order Functions
**OCaml:**
```ocaml
let compose f g x = f (g x)

let double x = 2 * x
let square x = x * x

(* square first, then double *)
let square_then_double = compose double square

let () =
  Printf.printf "square_then_double 3 = %d\n" (square_then_double 3);  (* 18 *)
  Printf.printf "square_then_double 4 = %d\n" (square_then_double 4)   (* 32 *)
```
**Status:** [x]

---

### 003: Pipeline Operator
**Source:** https://cs3110.github.io/textbook/chapters/hop/higher_order.html
**Topic:** Defining and using OCaml's |> (pipe-forward) operator to chain transformations left-to-right
**Difficulty:** Beginner
**Category:** Higher-Order Functions
**OCaml:**
```ocaml
(* The pipeline operator is just a higher-order function *)
let ( |> ) x f = f x

let double x = 2 * x
let add1   x = x + 1

(* Read: start with 5, double it, add 1 *)
let result = 5 |> double |> add1   (* 11 *)

(* Chaining string operations *)
let shout s = String.uppercase_ascii s
let exclaim s = s ^ "!"

let greeting = "hello" |> shout |> exclaim   (* "HELLO!" *)

let () =
  Printf.printf "%d\n" result;
  Printf.printf "%s\n" greeting
```
**Status:** [x]

---

### 004: List map from Scratch
**Source:** https://cs3110.github.io/textbook/chapters/hop/map.html
**Topic:** Deriving the List.map higher-order function step by step from two similar recursive functions; shows the Abstraction Principle
**Difficulty:** Beginner
**Category:** Higher-Order Functions
**OCaml:**
```ocaml
(* Build map by abstracting over the per-element operation *)
let rec map f = function
  | []     -> []
  | h :: t ->
    let h' = f h in          (* evaluate before recursing for left-to-right order *)
    h' :: map f t

(* Partial application creates specialised transformers *)
let add1      = map (fun x -> x + 1)
let to_string = map string_of_int
let double    = map (fun x -> x * 2)

let () =
  let nums = [1; 2; 3; 4; 5] in
  List.iter (Printf.printf "%d ") (add1   nums); print_newline ();
  List.iter (Printf.printf "%s ") (to_string nums); print_newline ();
  List.iter (Printf.printf "%d ") (double  nums); print_newline ()
```
**Status:** [x]

---

### 005: List filter from Scratch
**Source:** https://cs3110.github.io/textbook/chapters/hop/filter.html
**Topic:** Deriving the List.filter function; demonstrates predicate functions and how to remove elements while preserving order
**Difficulty:** Beginner
**Category:** Higher-Order Functions
**OCaml:**
```ocaml
let rec filter p = function
  | []     -> []
  | h :: t -> if p h then h :: filter p t else filter p t

let evens = filter (fun n -> n mod 2 = 0)
let odds  = filter (fun n -> n mod 2 <> 0)
let pos   = filter (fun n -> n > 0)

let () =
  let nums = [-3; -1; 0; 2; 4; 5; 7] in
  List.iter (Printf.printf "%d ") (evens nums); print_newline ();
  List.iter (Printf.printf "%d ") (odds  nums); print_newline ();
  List.iter (Printf.printf "%d ") (pos   nums); print_newline ()
```
**Status:** [x]

---

### 006: fold_right — Structural Recursion
**Source:** https://cs3110.github.io/textbook/chapters/hop/fold.html
**Topic:** Deriving List.fold_right by replacing [] with an accumulator and :: with an operator; the canonical "replace constructors" view of fold
**Difficulty:** Intermediate
**Category:** Higher-Order Functions
**OCaml:**
```ocaml
(* fold_right f [a; b; c] init  =  f a (f b (f c init)) *)
let rec fold_right f lst (acc : 'acc) =
  match lst with
  | []     -> acc
  | h :: t -> f h (fold_right f t acc)

(* Classic uses *)
let sum  lst = fold_right ( + ) lst 0
let prod lst = fold_right ( * ) lst 1
let cat  lst = fold_right ( ^ ) lst ""

(* Reconstructing a list — replacing :: with :: and [] with [] *)
let copy lst = fold_right (fun h t -> h :: t) lst []

let () =
  Printf.printf "sum  [1;2;3;4;5] = %d\n" (sum  [1;2;3;4;5]);
  Printf.printf "prod [1;2;3;4;5] = %d\n" (prod [1;2;3;4;5]);
  Printf.printf "cat  [\"a\";\"b\";\"c\"] = %s\n" (cat ["a";"b";"c"])
```
**Status:** [x]

---

### 007: fold_left — Tail-Recursive Accumulator
**Source:** https://cs3110.github.io/textbook/chapters/hop/fold.html
**Topic:** The tail-recursive sibling of fold_right — accumulates from left to right, avoids stack overflow on large lists
**Difficulty:** Intermediate
**Category:** Higher-Order Functions
**OCaml:**
```ocaml
(* fold_left f init [a; b; c]  =  f (f (f init a) b) c *)
let rec fold_left f acc = function
  | []     -> acc
  | h :: t -> fold_left f (f acc h) t

let sum     lst = fold_left ( + ) 0   lst
let product lst = fold_left ( * ) 1   lst
let maximum lst = fold_left max (List.hd lst) (List.tl lst)
let reverse lst = fold_left (fun acc x -> x :: acc) [] lst

let () =
  let nums = [3; 1; 4; 1; 5; 9; 2; 6] in
  Printf.printf "sum     = %d\n" (sum     nums);
  Printf.printf "product = %d\n" (product nums);
  Printf.printf "maximum = %d\n" (maximum nums);
  let rev = reverse nums in
  List.iter (Printf.printf "%d ") rev; print_newline ()
```
**Status:** [x]

---

### 008: Variants — Days of the Week
**Source:** https://cs3110.github.io/textbook/chapters/data/variants.html
**Topic:** Algebraic data type as an enumeration; pattern matching over variants to produce different outputs without explicit null checks
**Difficulty:** Beginner
**Category:** Algebraic Data Types
**OCaml:**
```ocaml
type day = Sun | Mon | Tue | Wed | Thu | Fri | Sat

let day_name = function
  | Sun -> "Sunday"    | Mon -> "Monday"  | Tue -> "Tuesday"
  | Wed -> "Wednesday" | Thu -> "Thursday"| Fri -> "Friday"
  | Sat -> "Saturday"

let is_weekend = function
  | Sun | Sat -> true
  | _         -> false

let next_day = function
  | Sun -> Mon | Mon -> Tue | Tue -> Wed | Wed -> Thu
  | Thu -> Fri | Fri -> Sat | Sat -> Sun

let () =
  let today = Wed in
  Printf.printf "Today is %s\n" (day_name today);
  Printf.printf "Is weekend? %b\n" (is_weekend today);
  Printf.printf "Tomorrow is %s\n" (day_name (next_day today))
```
**Status:** [x]

---

### 009: Recursive Variant — Expression Tree
**Source:** Cornell CS3110 — Data chapter (variants with payloads)
**Topic:** Algebraic data type with recursive constructors; evaluating an arithmetic expression tree via structural recursion and pattern matching
**Difficulty:** Intermediate
**Category:** Algebraic Data Types
**OCaml:**
```ocaml
type expr =
  | Num of float
  | Add of expr * expr
  | Sub of expr * expr
  | Mul of expr * expr
  | Div of expr * expr

let rec eval = function
  | Num n         -> n
  | Add (l, r)    -> eval l +. eval r
  | Sub (l, r)    -> eval l -. eval r
  | Mul (l, r)    -> eval l *. eval r
  | Div (l, r)    -> eval l /. eval r

let rec to_string = function
  | Num n         -> string_of_float n
  | Add (l, r)    -> Printf.sprintf "(%s + %s)" (to_string l) (to_string r)
  | Sub (l, r)    -> Printf.sprintf "(%s - %s)" (to_string l) (to_string r)
  | Mul (l, r)    -> Printf.sprintf "(%s * %s)" (to_string l) (to_string r)
  | Div (l, r)    -> Printf.sprintf "(%s / %s)" (to_string l) (to_string r)

(* (1 + 2) * (10 - 4) = 18 *)
let expr = Mul (Add (Num 1., Num 2.), Sub (Num 10., Num 4.))

let () =
  Printf.printf "%s = %.1f\n" (to_string expr) (eval expr)
```
**Status:** [x]

---

### 010: Option Type — Safe List Maximum
**Source:** https://cs3110.github.io/textbook/chapters/data/options.html
**Topic:** Using the option type to represent "no value" without exceptions or null; demonstrates pattern matching on Some/None
**Difficulty:** Beginner
**Category:** Error Handling
**OCaml:**
```ocaml
(* Returns None for empty list instead of raising an exception *)
let rec list_max = function
  | []     -> None
  | h :: t ->
    begin match list_max t with
    | None   -> Some h
    | Some m -> Some (max h m)
    end

(* Safe head — another classic option use *)
let safe_head = function
  | []    -> None
  | h :: _ -> Some h

(* Chaining options with map *)
let option_map f = function
  | None   -> None
  | Some x -> Some (f x)

let () =
  let nums = [3; 1; 4; 1; 5; 9; 2; 6] in
  (match list_max nums with
   | None   -> print_endline "empty list"
   | Some m -> Printf.printf "max = %d\n" m);
  (match list_max [] with
   | None   -> print_endline "max of [] = None"
   | Some m -> Printf.printf "max = %d\n" m);
  (* Double the maximum if it exists *)
  let doubled_max = option_map (fun x -> x * 2) (list_max nums) in
  match doubled_max with
  | None   -> ()
  | Some v -> Printf.printf "doubled max = %d\n" v
```
**Status:** [x]

---

### 011: Binary Tree — Size, Membership, Traversal
**Source:** https://cs3110.github.io/textbook/chapters/data/trees.html
**Topic:** Recursive algebraic data type for binary trees; writing size, membership, and preorder traversal via structural recursion
**Difficulty:** Intermediate
**Category:** Data Structures
**OCaml:**
```ocaml
type 'a tree =
  | Leaf
  | Node of 'a * 'a tree * 'a tree

let rec size = function
  | Leaf           -> 0
  | Node (_, l, r) -> 1 + size l + size r

let rec depth = function
  | Leaf           -> 0
  | Node (_, l, r) -> 1 + max (depth l) (depth r)

let rec mem x = function
  | Leaf           -> false
  | Node (v, l, r) -> v = x || mem x l || mem x r

(* Linear-time preorder using accumulator *)
let preorder t =
  let rec go acc = function
    | Leaf           -> acc
    | Node (v, l, r) -> v :: go (go acc r) l
  in go [] t

(*      4
       / \
      2   5
     / \
    1   3   *)
let t = Node (4, Node (2, Node (1, Leaf, Leaf), Node (3, Leaf, Leaf)), Node (5, Leaf, Leaf))

let () =
  Printf.printf "size     = %d\n" (size t);
  Printf.printf "depth    = %d\n" (depth t);
  Printf.printf "mem 3    = %b\n" (mem 3 t);
  Printf.printf "mem 99   = %b\n" (mem 99 t);
  Printf.printf "preorder = ";
  List.iter (Printf.printf "%d ") (preorder t);
  print_newline ()
```
**Status:** [ ]

---

### 012: Records — Immutable Update and Pattern Matching
**Source:** https://cs3110.github.io/textbook/chapters/data/records_tuples.html
**Topic:** OCaml records as named product types; functional update with the `with` syntax; pattern matching on record fields
**Difficulty:** Beginner
**Category:** Algebraic Data Types
**OCaml:**
```ocaml
type point = { x : float; y : float }

type rect = { origin : point; width : float; height : float }

let area { width; height; _ } = width *. height

let perimeter { width; height; _ } = 2.0 *. (width +. height)

(* Functional update: creates a new record, does not mutate *)
let translate dx dy r =
  { r with origin = { x = r.origin.x +. dx; y = r.origin.y +. dy } }

let contains_point r { x; y } =
  x >= r.origin.x && x <= r.origin.x +. r.width &&
  y >= r.origin.y && y <= r.origin.y +. r.height

let () =
  let r = { origin = { x = 0.0; y = 0.0 }; width = 10.0; height = 5.0 } in
  Printf.printf "area      = %.1f\n" (area r);
  Printf.printf "perimeter = %.1f\n" (perimeter r);
  let r2 = translate 3.0 4.0 r in
  Printf.printf "moved origin = (%.1f, %.1f)\n" r2.origin.x r2.origin.y;
  Printf.printf "contains (1,1)  = %b\n" (contains_point r { x = 1.0; y = 1.0 });
  Printf.printf "contains (11,1) = %b\n" (contains_point r { x = 11.0; y = 1.0 })
```
**Status:** [ ]


---
<!-- Added by OCaml Hunt cron — 2026-03-03 -->

### 013: Stack Module with Signature
**Source:** https://cs3110.github.io/textbook/chapters/modules/modules.html
**Topic:** Defining an OCaml module with an explicit signature; demonstrates how module types enforce abstraction and how ListStack encapsulates a LIFO data structure
**Difficulty:** Intermediate
**Category:** Modules
**OCaml:**
```ocaml
module type STACK = sig
  type 'a t
  exception Empty

  val empty    : 'a t
  val is_empty : 'a t -> bool
  val push     : 'a -> 'a t -> 'a t
  val peek     : 'a t -> 'a      (* raises Empty *)
  val pop      : 'a t -> 'a t    (* raises Empty *)
  val size     : 'a t -> int
end

module ListStack : STACK = struct
  type 'a t = 'a list
  exception Empty

  let empty        = []
  let is_empty s   = s = []
  let push x s     = x :: s
  let size s       = List.length s

  let peek = function
    | []     -> raise Empty
    | x :: _ -> x

  let pop = function
    | []      -> raise Empty
    | _ :: s  -> s
end

let () =
  let open ListStack in
  let s = empty |> push 1 |> push 2 |> push 3 in
  Printf.printf "size = %d\n" (size s);
  Printf.printf "peek = %d\n" (peek s);
  let s' = pop s in
  Printf.printf "after pop, peek = %d\n" (peek s')
```
**Status:** [ ]

---

### 014: Map.Make Functor — String→Int Dictionary
**Source:** https://ocaml.org/docs/maps
**Topic:** Using the Map.Make functor to create a type-safe, immutable balanced-BST dictionary; demonstrates functors as parametric module factories
**Difficulty:** Intermediate
**Category:** Modules / Functors
**OCaml:**
```ocaml
module StringMap = Map.Make(String)

let word_lengths words =
  List.fold_left
    (fun acc w -> StringMap.add w (String.length w) acc)
    StringMap.empty
    words

let () =
  let words = ["ocaml"; "rust"; "haskell"; "erlang"; "go"] in
  let m = word_lengths words in
  (match StringMap.find_opt "rust" m with
   | Some n -> Printf.printf "rust has %d chars\n" n
   | None   -> print_endline "not found");
  let long = StringMap.filter (fun _ v -> v > 4) m in
  Printf.printf "long words (%d):\n" (StringMap.cardinal long);
  StringMap.iter (fun k v -> Printf.printf "  %s -> %d\n" k v) long;
  let doubled = StringMap.map (fun v -> v * 2) m in
  Printf.printf "doubled rust length -> %d\n"
    (StringMap.find "rust" doubled)
```
**Status:** [ ]

---

### 015: Association List — Functional Key-Value Store
**Source:** https://cs3110.github.io/textbook/chapters/data/assoc_list.html
**Topic:** An association list (list of pairs) as the simplest possible map implementation; shows insertion shadowing and linear-time lookup
**Difficulty:** Beginner
**Category:** Data Structures
**OCaml:**
```ocaml
let insert k v lst = (k, v) :: lst

let rec lookup k = function
  | []            -> None
  | (k', v) :: t  -> if k = k' then Some v else lookup k t

let rec remove k = function
  | []                    -> []
  | (k', _) :: t when k = k' -> t
  | h :: t                -> h :: remove k t

let keys lst = List.map fst lst

let () =
  let d = [] in
  let d = insert "a" 1 d in
  let d = insert "b" 2 d in
  let d = insert "a" 99 d in   (* shadows earlier "a" *)
  Printf.printf "lookup a = %s\n"
    (match lookup "a" d with Some v -> string_of_int v | None -> "None");
  Printf.printf "lookup c = %s\n"
    (match lookup "c" d with Some v -> string_of_int v | None -> "None");
  let d' = remove "a" d in
  Printf.printf "after remove a, lookup a = %s\n"
    (match lookup "a" d' with Some v -> string_of_int v | None -> "None");
  Printf.printf "keys: %s\n" (String.concat ", " (keys d))
```
**Status:** [ ]

---

### 016: map and fold on Trees
**Source:** https://cs3110.github.io/textbook/chapters/hop/beyond_lists.html
**Topic:** Lifting map and fold from lists to binary trees; implementing size, depth, and traversals via a single fold_tree catamorphism — no explicit recursion needed after fold is defined
**Difficulty:** Intermediate
**Category:** Higher-Order Functions
**OCaml:**
```ocaml
type 'a tree =
  | Leaf
  | Node of 'a * 'a tree * 'a tree

let rec map_tree f = function
  | Leaf           -> Leaf
  | Node (v, l, r) -> Node (f v, map_tree f l, map_tree f r)

let rec fold_tree f acc = function
  | Leaf           -> acc
  | Node (v, l, r) -> f v (fold_tree f acc l) (fold_tree f acc r)

let size     t = fold_tree (fun _ l r -> 1 + l + r)    0  t
let depth    t = fold_tree (fun _ l r -> 1 + max l r)  0  t
let sum      t = fold_tree (fun v l r -> v + l + r)    0  t
let preorder t = fold_tree (fun v l r -> [v] @ l @ r) [] t
let inorder  t = fold_tree (fun v l r -> l @ [v] @ r) [] t

let t =
  Node (4, Node (2, Node (1, Leaf, Leaf), Node (3, Leaf, Leaf)),
           Node (6, Leaf, Leaf))

let () =
  Printf.printf "size     = %d\n" (size t);
  Printf.printf "depth    = %d\n" (depth t);
  Printf.printf "sum      = %d\n" (sum t);
  Printf.printf "preorder = %s\n"
    (String.concat " " (List.map string_of_int (preorder t)));
  Printf.printf "inorder  = %s\n"
    (String.concat " " (List.map string_of_int (inorder t)));
  let t2 = map_tree (fun v -> v * 2) t in
  Printf.printf "doubled sum = %d\n" (sum t2)
```
**Status:** [ ]

---

### 017: Mutual Recursion with `and`
**Source:** https://cs3110.github.io/textbook/chapters/modules/modules.html
**Topic:** Using OCaml's `let rec … and …` to define mutually recursive functions; even/odd as the canonical example, plus a mutually recursive ADT evaluator
**Difficulty:** Beginner
**Category:** Recursion
**OCaml:**
```ocaml
let rec is_even = function
  | 0 -> true
  | n -> is_odd (n - 1)
and     is_odd = function
  | 0 -> false
  | n -> is_even (n - 1)

(* Mutual recursion over an algebraic type *)
type expr =
  | Lit of int
  | Add of expr * expr
  | Mul of expr * expr

let rec eval_expr = function
  | Lit n      -> n
  | Add (l, r) -> eval_expr l + eval_expr r
  | Mul (l, r) -> eval_mul l r
and eval_mul l r =
  eval_expr l * eval_expr r

let () =
  Printf.printf "is_even 4  = %b\n" (is_even 4);
  Printf.printf "is_odd  7  = %b\n" (is_odd  7);
  Printf.printf "is_even 0  = %b\n" (is_even 0);
  let e = Mul (Add (Lit 2, Lit 3), Lit 4) in
  Printf.printf "(2+3)*4    = %d\n" (eval_expr e)
```
**Status:** [ ]

---

### 018: Tail-Recursive Accumulator Pattern
**Source:** Cornell CS3110 — Tail Recursion chapter
**Topic:** Transforming naive recursion into tail recursion using an accumulator; demonstrates stack safety for large inputs and the generalisation to any fold-like traversal
**Difficulty:** Intermediate
**Category:** Recursion
**OCaml:**
```ocaml
(* Naive — builds O(n) stack; crashes on big lists *)
let rec sum_naive = function
  | []     -> 0
  | h :: t -> h + sum_naive t

(* Tail-recursive — last action is the recursive call *)
let sum_tr lst =
  let rec go acc = function
    | []     -> acc
    | h :: t -> go (acc + h) t
  in go 0 lst

let rev_tr lst =
  let rec go acc = function
    | []     -> acc
    | h :: t -> go (h :: acc) t
  in go [] lst

let fib_tr n =
  let rec go a b = function
    | 0 -> a
    | n -> go b (a + b) (n - 1)
  in go 0 1 n

let () =
  let big = List.init 100_000 (fun i -> i + 1) in
  Printf.printf "sum 1..100000 = %d\n" (sum_tr big);
  Printf.printf "rev head      = %d\n" (List.hd (rev_tr big));
  Printf.printf "fib 10        = %d\n" (fib_tr 10);
  Printf.printf "fib 40        = %d\n" (fib_tr 40)
```
**Status:** [ ]

---

### 019: Sieve of Eratosthenes (Functional)
**Source:** https://rosettacode.org/wiki/Sieve_of_Eratosthenes#OCaml
**Topic:** A purely functional prime sieve using recursive filtering; demonstrates that the list acts as a lazy stream of candidates
**Difficulty:** Intermediate
**Category:** Algorithms
**OCaml:**
```ocaml
let rec sieve = function
  | []      -> []
  | p :: rest ->
    p :: sieve (List.filter (fun n -> n mod p <> 0) rest)

let primes_up_to n =
  if n < 2 then []
  else sieve (List.init (n - 1) (fun i -> i + 2))

let nth_prime n =
  (* generate enough candidates and take the n-th prime *)
  let limit = max 10 (n * n) in
  let ps = primes_up_to limit in
  List.nth ps (n - 1)

let () =
  let ps = primes_up_to 50 in
  Printf.printf "primes up to 50: %s\n"
    (String.concat " " (List.map string_of_int ps));
  Printf.printf "count up to 100: %d\n"
    (List.length (primes_up_to 100));
  Printf.printf "10th prime: %d\n" (nth_prime 10)
```
**Status:** [ ]

---

### 020: Hamming Distance
**Source:** https://exercism.org/tracks/ocaml/exercises/hamming
**Topic:** Counting differences between two equal-length strings using List.combine + filter + length; a clean pattern for pairwise character comparison
**Difficulty:** Beginner
**Category:** String Processing
**OCaml:**
```ocaml
exception Invalid_argument of string

let chars_of_string s =
  List.init (String.length s) (String.get s)

let hamming_distance s1 s2 =
  if String.length s1 <> String.length s2 then
    raise (Invalid_argument "strands must be of equal length");
  List.combine (chars_of_string s1) (chars_of_string s2)
  |> List.filter (fun (a, b) -> a <> b)
  |> List.length

let hamming_fold s1 s2 =
  if String.length s1 <> String.length s2 then
    raise (Invalid_argument "strands must be of equal length");
  List.fold_left2
    (fun acc c1 c2 -> if c1 <> c2 then acc + 1 else acc)
    0
    (chars_of_string s1)
    (chars_of_string s2)

let () =
  let s1 = "GAGCCTACTAACGGGAT" in
  let s2 = "CATCGTAATGACGGCCT" in
  Printf.printf "hamming %S %S = %d\n" s1 s2 (hamming_distance s1 s2);
  Printf.printf "identical: %d\n" (hamming_distance "AAAA" "AAAA");
  Printf.printf "complete:  %d\n" (hamming_fold "AAAA" "TTTT")
```
**Status:** [ ]

---

### 021: Collatz Conjecture
**Source:** https://exercism.org/tracks/ocaml/exercises/collatz-conjecture
**Topic:** Computing the Collatz (3n+1) sequence step count; demonstrates simple recursion, a Result-typed safe API, and then a tail-recursive accumulator variant
**Difficulty:** Beginner
**Category:** Recursion
**OCaml:**
```ocaml
let rec collatz_steps n =
  if n = 1 then 0
  else if n mod 2 = 0 then 1 + collatz_steps (n / 2)
  else 1 + collatz_steps (3 * n + 1)

let collatz n =
  if n <= 0 then Error "Only positive integers are allowed"
  else Ok (collatz_steps n)

let collatz_tr n =
  let rec go steps = function
    | 1 -> steps
    | n when n mod 2 = 0 -> go (steps + 1) (n / 2)
    | n                  -> go (steps + 1) (3 * n + 1)
  in
  if n <= 0 then Error "Only positive integers are allowed"
  else Ok (go 0 n)

let () =
  List.iter (fun n ->
    match collatz_tr n with
    | Ok s  -> Printf.printf "collatz(%3d) = %d steps\n" n s
    | Error e -> Printf.printf "collatz(%3d): %s\n" n e
  ) [1; 6; 11; 27; -1]
```
**Status:** [ ]

---

### 022: Result Type — Railway-Oriented Error Handling
**Source:** Cornell CS3110 — Error Handling chapter
**Topic:** Using the Result type with a bind combinator (>>=) for "railway-oriented programming"; errors short-circuit a chain of fallible operations without try/catch
**Difficulty:** Intermediate
**Category:** Error Handling
**OCaml:**
```ocaml
let ( >>= ) r f = match r with
  | Error e -> Error e
  | Ok v    -> f v

let ( >>| ) r f = match r with
  | Error e -> Error e
  | Ok v    -> Ok (f v)

let parse_int s =
  match int_of_string_opt s with
  | None   -> Error (Printf.sprintf "not an integer: %S" s)
  | Some n -> Ok n

let positive x =
  if x > 0 then Ok x
  else Error (Printf.sprintf "%d is not positive" x)

let sqrt_safe x =
  positive x >>| (fun n -> sqrt (float_of_int n))

(* Pipeline: each step can fail; errors propagate automatically *)
let process s =
  parse_int s >>= positive >>= sqrt_safe

let () =
  List.iter (fun s ->
    match process s with
    | Ok v    -> Printf.printf "process(%S) = %.4f\n" s v
    | Error e -> Printf.printf "process(%S) ERR: %s\n" s e
  ) ["16"; "25"; "-4"; "hello"; "0"]
```
**Status:** [ ]

---

### 023: Word Count with Map
**Source:** https://exercism.org/tracks/ocaml/exercises/word-count
**Topic:** Building a word-frequency map from a sentence using Map.Make; demonstrates string normalisation, splitting, and folding into an immutable ordered map
**Difficulty:** Intermediate
**Category:** Data Structures
**OCaml:**
```ocaml
module StringMap = Map.Make(String)

let tokenize s =
  let s = String.lowercase_ascii s in
  let words = ref [] and buf = Buffer.create 16 in
  let flush () =
    if Buffer.length buf > 0 then begin
      words := Buffer.contents buf :: !words;
      Buffer.clear buf
    end
  in
  String.iter (fun c ->
    if (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') then
      Buffer.add_char buf c
    else flush ()
  ) s;
  flush ();
  List.rev !words

let word_count sentence =
  tokenize sentence
  |> List.fold_left (fun m w ->
       let n = Option.value ~default:0 (StringMap.find_opt w m) in
       StringMap.add w (n + 1) m)
     StringMap.empty

let () =
  let s = "the cat sat on the mat, the cat sat" in
  let m = word_count s in
  Printf.printf "Counts for: %S\n" s;
  StringMap.iter (fun w n -> Printf.printf "  %-8s %d\n" w n) m
```
**Status:** [ ]

---

### 024: Currying, Partial Application, and Sections
**Source:** https://cs3110.github.io/textbook/chapters/hop/higher_order.html
**Topic:** Every OCaml function is curried by default; partial application produces specialised functions from general ones; labeled arguments enable keyword-style partial application
**Difficulty:** Beginner
**Category:** Higher-Order Functions
**OCaml:**
```ocaml
(* Curried: int -> int -> int *)
let add x y = x + y
let add5 = add 5             (* partial application *)

(* Tupled: NOT the OCaml default *)
let add_tup (x, y) = x + y

(* Converters between the two styles *)
let curry   f x y = f (x, y)
let uncurry f (x, y) = f x y

(* Operator sections via partial application *)
let double    = ( * ) 2
let increment = ( + ) 1
let halve     = Fun.flip ( / ) 2   (* flip swaps argument order *)

(* Labeled arguments allow any-order partial application *)
let scale_and_shift ~scale ~shift x = x * scale + shift
let celsius_of_fahrenheit = scale_and_shift ~scale:5 ~shift:(-160)

let () =
  Printf.printf "add5 10   = %d\n" (add5 10);
  Printf.printf "double 7  = %d\n" (double 7);
  Printf.printf "halve 20  = %d\n" (halve 20);
  let pipeline = [double; increment; halve] in
  let result = List.fold_left (fun acc f -> f acc) 6 pipeline in
  Printf.printf "6 |> *2 |> +1 |> /2 = %d\n" result;
  Printf.printf "212F in Celsius ≈ %d\n" (celsius_of_fahrenheit 212)
```
**Status:** [ ]

### 025: Merge Sort — Functional Divide and Conquer
**Source:** https://cs3110.github.io/textbook/chapters/ds/bst.html
**Topic:** Pure functional merge sort using list splitting and merging
**Difficulty:** Intermediate
**Category:** Sorting algorithms
**OCaml:**
```ocaml
let rec merge cmp l1 l2 = match l1, l2 with
  | [], l | l, [] -> l
  | h1 :: t1, h2 :: t2 ->
    if cmp h1 h2 <= 0 then h1 :: merge cmp t1 l2
    else h2 :: merge cmp l1 t2

let rec split = function
  | [] -> [], []
  | [x] -> [x], []
  | a :: b :: rest ->
    let l, r = split rest in
    a :: l, b :: r

let rec merge_sort cmp = function
  | ([] | [_]) as l -> l
  | l ->
    let left, right = split l in
    merge cmp (merge_sort cmp left) (merge_sort cmp right)

let () =
  let sorted = merge_sort compare [5; 2; 8; 1; 9; 3] in
  List.iter (Printf.printf "%d ") sorted
```
**Status:** [ ]

### 026: Quick Sort — Functional Partition
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Quicksort#OCaml
**Topic:** Quicksort using List.partition for elegant functional partitioning
**Difficulty:** Intermediate
**Category:** Sorting algorithms
**OCaml:**
```ocaml
let rec quicksort = function
  | [] -> []
  | pivot :: rest ->
    let left, right = List.partition (fun x -> x < pivot) rest in
    quicksort left @ [pivot] @ quicksort right

let () =
  let sorted = quicksort [3; 6; 8; 10; 1; 2; 1] in
  List.iter (Printf.printf "%d ") sorted
```
**Status:** [ ]

### 027: Insertion Sort — Building Sorted Lists
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Insertion_sort#OCaml
**Topic:** Insertion sort via recursive sorted insertion
**Difficulty:** Beginner
**Category:** Sorting algorithms
**OCaml:**
```ocaml
let rec insert x = function
  | [] -> [x]
  | h :: t as l ->
    if x <= h then x :: l
    else h :: insert x t

let insertion_sort lst =
  List.fold_left (fun acc x -> insert x acc) [] lst

let () =
  let sorted = insertion_sort [5; 3; 1; 4; 2] in
  List.iter (Printf.printf "%d ") sorted
```
**Status:** [ ]

### 028: Graph BFS — Breadth-First Search
**Source:** https://cs3110.github.io/textbook/chapters/ds/bst.html
**Topic:** BFS on adjacency list graph using a queue
**Difficulty:** Intermediate
**Category:** Graphs
**OCaml:**
```ocaml
let bfs graph start =
  let visited = Hashtbl.create 16 in
  let queue = Queue.create () in
  Queue.push start queue;
  Hashtbl.add visited start true;
  let result = ref [] in
  while not (Queue.is_empty queue) do
    let node = Queue.pop queue in
    result := node :: !result;
    List.iter (fun neighbor ->
      if not (Hashtbl.mem visited neighbor) then begin
        Hashtbl.add visited neighbor true;
        Queue.push neighbor queue
      end
    ) (List.assoc node graph)
  done;
  List.rev !result

let () =
  let graph = [("a", ["b";"c"]); ("b", ["d"]); ("c", ["d"]); ("d", [])] in
  List.iter (Printf.printf "%s ") (bfs graph "a")
```
**Status:** [ ]

### 029: Graph DFS — Depth-First Search (Functional)
**Source:** https://cs3110.github.io/textbook/chapters/ds/bst.html
**Topic:** Pure functional DFS using visited set
**Difficulty:** Intermediate
**Category:** Graphs
**OCaml:**
```ocaml
module SS = Set.Make(String)

let dfs graph start =
  let rec go visited node =
    if SS.mem node visited then (visited, [])
    else
      let visited = SS.add node visited in
      let neighbors = try List.assoc node graph with Not_found -> [] in
      let visited, paths = List.fold_left (fun (vis, acc) n ->
        let vis, path = go vis n in
        (vis, acc @ path)
      ) (visited, []) neighbors in
      (visited, node :: paths)
  in
  snd (go SS.empty start)

let () =
  let g = [("a",["b";"c"]); ("b",["d"]); ("c",["d"]); ("d",[])] in
  List.iter (Printf.printf "%s ") (dfs g "a")
```
**Status:** [ ]

### 030: Lazy Sequences — Infinite Fibonacci
**Source:** https://cs3110.github.io/textbook/chapters/ds/streams.html
**Topic:** Lazy streams for infinite sequences using thunks
**Difficulty:** Intermediate
**Category:** Lazy/infinite sequences
**OCaml:**
```ocaml
type 'a stream = Cons of 'a * (unit -> 'a stream)

let rec fibs a b = Cons (a, fun () -> fibs b (a + b))

let rec take n (Cons (x, rest)) =
  if n = 0 then []
  else x :: take (n - 1) (rest ())

let () =
  let fib_stream = fibs 0 1 in
  List.iter (Printf.printf "%d ") (take 10 fib_stream)
```
**Status:** [ ]

### 031: Memoization — Fibonacci with Hashtable Cache
**Source:** https://dev.realworldocaml.org/
**Topic:** Transparent memoization using a hash table wrapper
**Difficulty:** Intermediate
**Category:** Memoization
**OCaml:**
```ocaml
let memoize f =
  let cache = Hashtbl.create 16 in
  fun x ->
    match Hashtbl.find_opt cache x with
    | Some v -> v
    | None ->
      let v = f x in
      Hashtbl.add cache x v;
      v

let fib =
  let rec fib' n =
    if n <= 1 then n
    else memo_fib (n - 1) + memo_fib (n - 2)
  and memo_fib = memoize fib'
  in memo_fib

let () = Printf.printf "fib(35) = %d\n" (fib 35)
```
**Status:** [ ]

### 032: State Machine — Turnstile
**Source:** https://dev.realworldocaml.org/
**Topic:** Encoding state machines with variants and pattern matching
**Difficulty:** Intermediate
**Category:** State machines
**OCaml:**
```ocaml
type state = Locked | Unlocked
type event = Coin | Push

let transition state event = match state, event with
  | Locked, Coin -> Unlocked
  | Unlocked, Push -> Locked
  | Locked, Push -> Locked
  | Unlocked, Coin -> Unlocked

let state_name = function Locked -> "Locked" | Unlocked -> "Unlocked"

let () =
  let events = [Coin; Push; Push; Coin; Coin; Push] in
  let final = List.fold_left (fun s e ->
    let s' = transition s e in
    Printf.printf "%s -> %s\n" (state_name s) (state_name s');
    s'
  ) Locked events in
  Printf.printf "Final: %s\n" (state_name final)
```
**Status:** [ ]

### 033: Monadic Option Chaining
**Source:** https://cs3110.github.io/textbook/chapters/ds/monads.html
**Topic:** Option monad bind (>>=) for safe chaining without nested matches
**Difficulty:** Intermediate
**Category:** Monadic patterns
**OCaml:**
```ocaml
let ( >>= ) opt f = match opt with
  | None -> None
  | Some x -> f x

let ( >>| ) opt f = match opt with
  | None -> None
  | Some x -> Some (f x)

let safe_div x y = if y = 0 then None else Some (x / y)
let safe_head = function [] -> None | h :: _ -> Some h

let compute lst =
  safe_head lst >>= fun x ->
  safe_div 100 x >>| fun r ->
  r * 2

let () =
  let show = function None -> "None" | Some x -> string_of_int x in
  Printf.printf "%s\n" (show (compute [5; 3; 1]));
  Printf.printf "%s\n" (show (compute [0; 1]));
  Printf.printf "%s\n" (show (compute []))
```
**Status:** [ ]

### 034: Result Monad — Error Chaining
**Source:** https://dev.realworldocaml.org/error-handling.html
**Topic:** Result monad with bind for railway-oriented programming
**Difficulty:** Intermediate
**Category:** Monadic patterns
**OCaml:**
```ocaml
let ( >>= ) r f = match r with
  | Error _ as e -> e
  | Ok x -> f x

let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error ("Not an integer: " ^ s)

let check_positive n =
  if n > 0 then Ok n else Error "Must be positive"

let check_even n =
  if n mod 2 = 0 then Ok n else Error "Must be even"

let validate s =
  parse_int s >>= check_positive >>= check_even

let () =
  List.iter (fun s ->
    match validate s with
    | Ok n -> Printf.printf "%s -> Ok %d\n" s n
    | Error e -> Printf.printf "%s -> Error: %s\n" s e
  ) ["42"; "-3"; "abc"; "7"]
```
**Status:** [ ]

### 035: Functor — Comparable Set
**Source:** https://dev.realworldocaml.org/functors.html
**Topic:** Creating a custom Set using the Map.Make functor pattern
**Difficulty:** Advanced
**Category:** Functors and modules
**OCaml:**
```ocaml
module type COMPARABLE = sig
  type t
  val compare : t -> t -> int
end

module MakeSet (C : COMPARABLE) = struct
  type t = C.t list
  let empty = []
  let mem x = List.exists (fun y -> C.compare x y = 0)
  let add x s = if mem x s then s else x :: s
  let to_list s = List.sort C.compare s
end

module IntSet = MakeSet(Int)
module StringSet = MakeSet(String)

let () =
  let s = IntSet.(empty |> add 3 |> add 1 |> add 3 |> add 2) in
  List.iter (Printf.printf "%d ") (IntSet.to_list s);
  print_newline ()
```
**Status:** [ ]

### 036: Binary Search Tree — Insert and Search
**Source:** https://cs3110.github.io/textbook/chapters/ds/bst.html
**Topic:** Immutable BST with functional insert and membership
**Difficulty:** Intermediate
**Category:** Trees
**OCaml:**
```ocaml
type 'a bst = Leaf | Node of 'a bst * 'a * 'a bst

let rec insert x = function
  | Leaf -> Node (Leaf, x, Leaf)
  | Node (l, v, r) ->
    if x < v then Node (insert x l, v, r)
    else if x > v then Node (l, v, insert x r)
    else Node (l, v, r)

let rec mem x = function
  | Leaf -> false
  | Node (l, v, r) ->
    if x = v then true
    else if x < v then mem x l
    else mem x r

let rec inorder = function
  | Leaf -> []
  | Node (l, v, r) -> inorder l @ [v] @ inorder r

let () =
  let tree = List.fold_left (fun t x -> insert x t) Leaf [5;3;7;1;4;6;8] in
  List.iter (Printf.printf "%d ") (inorder tree);
  Printf.printf "\nmem 4 = %b, mem 9 = %b\n" (mem 4 tree) (mem 9 tree)
```
**Status:** [ ]

### 037: Rose Tree — Multi-Way Tree with Fold
**Source:** https://cs3110.github.io/textbook/chapters/ds/trees.html
**Topic:** Rose trees (n-ary) with recursive fold
**Difficulty:** Intermediate
**Category:** Trees
**OCaml:**
```ocaml
type 'a rose = Rose of 'a * 'a rose list

let rec fold f (Rose (x, children)) =
  f x (List.map (fold f) children)

let size = fold (fun _ sizes -> 1 + List.fold_left (+) 0 sizes)
let depth = fold (fun _ depths ->
  1 + List.fold_left max 0 depths)

let to_string = fold (fun x strs ->
  match strs with
  | [] -> x
  | _ -> x ^ "(" ^ String.concat "," strs ^ ")")

let () =
  let tree = Rose ("a", [
    Rose ("b", [Rose ("d", []); Rose ("e", [])]);
    Rose ("c", [Rose ("f", [])])
  ]) in
  Printf.printf "size=%d depth=%d repr=%s\n"
    (size tree) (depth tree) (to_string tree)
```
**Status:** [ ]

### 038: AVL Tree — Self-Balancing BST
**Source:** https://rosettacode.org/wiki/AVL_tree#OCaml
**Topic:** AVL tree with rotation and height-balanced insert
**Difficulty:** Advanced
**Category:** Trees
**OCaml:**
```ocaml
type 'a avl = Empty | Node of 'a avl * 'a * 'a avl * int

let height = function Empty -> 0 | Node (_, _, _, h) -> h
let node l v r = Node (l, v, r, 1 + max (height l) (height r))
let balance t = match t with Empty -> 0 | Node (l,_,r,_) -> height l - height r

let rotate_right = function
  | Node (Node (ll, lv, lr, _), v, r, _) -> node (node ll lv lr) v r
  | t -> t

let rotate_left = function
  | Node (l, v, Node (rl, rv, rr, _), _) -> node l v (node rl rv rr)
  | t -> t

let rebalance t = match balance t with
  | b when b > 1 -> rotate_right t
  | b when b < -1 -> rotate_left t
  | _ -> t

let rec insert x = function
  | Empty -> node Empty x Empty
  | Node (l, v, r, _) ->
    if x < v then rebalance (node (insert x l) v r)
    else if x > v then rebalance (node l v (insert x r))
    else node l v r

let rec inorder = function
  | Empty -> [] | Node (l,v,r,_) -> inorder l @ [v] @ inorder r

let () =
  let t = List.fold_left (fun t x -> insert x t) Empty [7;3;9;1;5;8;10;2] in
  List.iter (Printf.printf "%d ") (inorder t)
```
**Status:** [ ]

### 039: String Anagram Check
**Source:** https://exercism.org/tracks/ocaml/exercises/anagram
**Topic:** Checking if two strings are anagrams using sorting
**Difficulty:** Beginner
**Category:** String processing
**OCaml:**
```ocaml
let to_sorted_chars s =
  s |> String.lowercase_ascii
    |> String.to_seq |> List.of_seq
    |> List.sort Char.compare

let is_anagram s1 s2 =
  let s1' = String.lowercase_ascii s1 in
  let s2' = String.lowercase_ascii s2 in
  s1' <> s2' && to_sorted_chars s1 = to_sorted_chars s2

let find_anagrams word candidates =
  List.filter (is_anagram word) candidates

let () =
  let results = find_anagrams "listen" ["enlists";"google";"inlets";"silent"] in
  List.iter (Printf.printf "%s ") results
```
**Status:** [ ]

### 040: Caesar Cipher — Functional Encryption
**Source:** https://exercism.org/tracks/ocaml/exercises/rotational-cipher
**Topic:** String transformation using character mapping
**Difficulty:** Beginner
**Category:** String processing
**OCaml:**
```ocaml
let shift_char n c =
  if c >= 'a' && c <= 'z' then
    Char.chr ((Char.code c - Char.code 'a' + n) mod 26 + Char.code 'a')
  else if c >= 'A' && c <= 'Z' then
    Char.chr ((Char.code c - Char.code 'A' + n) mod 26 + Char.code 'A')
  else c

let caesar n s = String.map (shift_char n) s
let decrypt n = caesar (26 - n)

let () =
  let msg = "Hello World" in
  let enc = caesar 13 msg in
  Printf.printf "Encrypted: %s\nDecrypted: %s\n" enc (decrypt 13 enc)
```
**Status:** [ ]

### 041: Pangram Check
**Source:** https://exercism.org/tracks/ocaml/exercises/pangram
**Topic:** Set-based string analysis to check all letters present
**Difficulty:** Beginner
**Category:** String processing
**OCaml:**
```ocaml
module CS = Set.Make(Char)

let alphabet = 
  List.init 26 (fun i -> Char.chr (i + Char.code 'a'))
  |> CS.of_list

let is_pangram s =
  let chars = s |> String.lowercase_ascii |> String.to_seq
    |> Seq.filter (fun c -> c >= 'a' && c <= 'z')
    |> CS.of_seq in
  CS.subset alphabet chars

let () =
  Printf.printf "%b\n" (is_pangram "The quick brown fox jumps over the lazy dog");
  Printf.printf "%b\n" (is_pangram "Hello world")
```
**Status:** [x]

### 042: Isogram Check
**Source:** https://exercism.org/tracks/ocaml/exercises/isogram
**Topic:** Detecting duplicate characters using a set
**Difficulty:** Beginner
**Category:** String processing
**OCaml:**
```ocaml
let is_isogram s =
  let chars = s |> String.lowercase_ascii |> String.to_seq
    |> Seq.filter (fun c -> c >= 'a' && c <= 'z')
    |> List.of_seq in
  let unique = List.sort_uniq Char.compare chars in
  List.length chars = List.length unique

let () =
  List.iter (fun s ->
    Printf.printf "%s: %b\n" s (is_isogram s)
  ) ["lumberjacks"; "background"; "eleven"; "subdermatoglyphic"]
```
**Status:** [x]

### 043: Run-Length Encoding — String Compression
**Source:** https://exercism.org/tracks/ocaml/exercises/run-length-encoding
**Topic:** Encoding consecutive character runs into counts
**Difficulty:** Beginner
**Category:** String processing
**OCaml:**
```ocaml
let encode s =
  let n = String.length s in
  if n = 0 then "" else
  let buf = Buffer.create n in
  let rec go i c count =
    if i = n then begin
      if count > 1 then Buffer.add_string buf (string_of_int count);
      Buffer.add_char buf c
    end else if s.[i] = c then go (i+1) c (count+1)
    else begin
      if count > 1 then Buffer.add_string buf (string_of_int count);
      Buffer.add_char buf c;
      go (i+1) s.[i] 1
    end
  in
  go 1 s.[0] 1;
  Buffer.contents buf

let () =
  Printf.printf "%s\n" (encode "AABCCCDEEEE")
```
**Status:** [x]

### 044: Balanced Parentheses
**Source:** https://exercism.org/tracks/ocaml/exercises/matching-brackets
**Topic:** Stack-based bracket matching using a list as stack
**Difficulty:** Beginner
**Category:** Parsing
**OCaml:**
```ocaml
let is_balanced s =
  let matching = function ')' -> '(' | ']' -> '[' | '}' -> '{' | _ -> ' ' in
  let rec check stack i =
    if i = String.length s then stack = []
    else match s.[i] with
    | '(' | '[' | '{' as c -> check (c :: stack) (i + 1)
    | ')' | ']' | '}' as c ->
      (match stack with
       | top :: rest when top = matching c -> check rest (i + 1)
       | _ -> false)
    | _ -> check stack (i + 1)
  in
  check [] 0

let () =
  List.iter (fun s ->
    Printf.printf "%s: %b\n" s (is_balanced s)
  ) ["([]{})";"([)]";"((()))";"[{()}]";"("]
```
**Status:** [x]

### 045: Simple Recursive Descent Parser
**Source:** https://cs3110.github.io/textbook/chapters/interp/parsing.html
**Topic:** Parsing arithmetic expressions into an AST
**Difficulty:** Advanced
**Category:** Parsing
**OCaml:**
```ocaml
type expr = Num of int | Add of expr * expr | Mul of expr * expr

let rec parse_expr tokens = 
  let left, rest = parse_term tokens in
  match rest with
  | "+" :: rest' ->
    let right, rest'' = parse_expr rest' in
    (Add (left, right), rest'')
  | _ -> (left, rest)
and parse_term tokens =
  let left, rest = parse_atom tokens in
  match rest with
  | "*" :: rest' ->
    let right, rest'' = parse_term rest' in
    (Mul (left, right), rest'')
  | _ -> (left, rest)
and parse_atom = function
  | n :: rest -> (Num (int_of_string n), rest)
  | [] -> failwith "unexpected end"

let rec eval = function
  | Num n -> n | Add (a,b) -> eval a + eval b | Mul (a,b) -> eval a * eval b

let () =
  let tokens = ["2";"+";"3";"*";"4"] in
  let ast, _ = parse_expr tokens in
  Printf.printf "2+3*4 = %d\n" (eval ast)
```
**Status:** [ ]

### 046: Zipper — Functional List Cursor
**Source:** https://cs3110.github.io/textbook/chapters/ds/zippers.html
**Topic:** Zipper data structure for O(1) local list navigation
**Difficulty:** Intermediate
**Category:** Data structures
**OCaml:**
```ocaml
type 'a zipper = { left: 'a list; focus: 'a; right: 'a list }

let of_list = function
  | [] -> failwith "empty"
  | h :: t -> { left = []; focus = h; right = t }

let go_right z = match z.right with
  | [] -> None
  | h :: t -> Some { left = z.focus :: z.left; focus = h; right = t }

let go_left z = match z.left with
  | [] -> None
  | h :: t -> Some { left = t; focus = h; right = z.focus :: z.right }

let update f z = { z with focus = f z.focus }
let to_list z = List.rev z.left @ [z.focus] @ z.right

let () =
  let z = of_list [1;2;3;4;5] in
  let z = Option.get (go_right z) in
  let z = Option.get (go_right z) in
  let z = update (fun x -> x * 10) z in
  List.iter (Printf.printf "%d ") (to_list z)
```
**Status:** [ ]

### 047: Church Numerals — Functions as Numbers
**Source:** https://cs3110.github.io/textbook/chapters/hop/lambda.html
**Topic:** Lambda calculus encoding of natural numbers
**Difficulty:** Advanced
**Category:** Higher-order functions
**OCaml:**
```ocaml
let zero _f x = x
let one f x = f x
let succ n f x = f (n f x)
let add m n f x = m f (n f x)
let mul m n f = m (n f)
let to_int n = n (fun x -> x + 1) 0

let two = succ one
let three = add one two
let six = mul two three
let nine = mul three three

let () =
  Printf.printf "0=%d 1=%d 2=%d 3=%d 6=%d 9=%d\n"
    (to_int zero) (to_int one) (to_int two)
    (to_int three) (to_int six) (to_int nine)
```
**Status:** [x]

### 048: CPS — Continuation-Passing Style
**Source:** https://cs3110.github.io/textbook/chapters/hop/cps.html
**Topic:** Converting recursive functions to CPS for tail recursion
**Difficulty:** Advanced
**Category:** Higher-order functions
**OCaml:**
```ocaml
(* Direct style - not tail recursive *)
let rec factorial n =
  if n = 0 then 1 else n * factorial (n - 1)

(* CPS style - tail recursive *)
let factorial_cps n =
  let rec go n k =
    if n = 0 then k 1
    else go (n - 1) (fun result -> k (n * result))
  in
  go n Fun.id

(* CPS tree sum *)
type 'a tree = Leaf of 'a | Node of 'a tree * 'a tree

let sum_cps t =
  let rec go t k = match t with
    | Leaf x -> k x
    | Node (l, r) -> go l (fun sl -> go r (fun sr -> k (sl + sr)))
  in go t Fun.id

let () =
  Printf.printf "%d\n" (factorial_cps 10);
  let t = Node (Node (Leaf 1, Leaf 2), Node (Leaf 3, Leaf 4)) in
  Printf.printf "%d\n" (sum_cps t)
```
**Status:** [ ]

### 049: Phantom Types — Type-Safe Units
**Source:** https://dev.realworldocaml.org/
**Topic:** Using phantom type parameters for compile-time unit safety
**Difficulty:** Advanced
**Category:** Functors and modules
**OCaml:**
```ocaml
type meters
type seconds
type 'a quantity = Q of float

let meters x : meters quantity = Q x
let seconds x : seconds quantity = Q x

let add (Q a : 'a quantity) (Q b : 'a quantity) : 'a quantity = Q (a +. b)
let scale k (Q a : 'a quantity) : 'a quantity = Q (k *. a)
let value (Q x) = x

let () =
  let d1 = meters 100.0 in
  let d2 = meters 50.0 in
  let total = add d1 d2 in
  Printf.printf "Distance: %.1f m\n" (value total);
  (* add d1 (seconds 5.0) would be a type error! *)
  let doubled = scale 2.0 (seconds 3.0) in
  Printf.printf "Time: %.1f s\n" (value doubled)
```
**Status:** [x]

### 050: Seq Module — Lazy Sequences in OCaml 4.14+
**Source:** https://ocaml.org/docs/
**Topic:** Using the built-in Seq module for lazy computation
**Difficulty:** Intermediate
**Category:** Lazy/infinite sequences
**OCaml:**
```ocaml
let naturals = Seq.ints 0

let fibs =
  Seq.unfold (fun (a, b) -> Some (a, (b, a + b))) (0, 1)

let primes =
  let is_prime n =
    n >= 2 && Seq.ints 2
    |> Seq.take_while (fun i -> i * i <= n)
    |> Seq.for_all (fun i -> n mod i <> 0)
  in
  Seq.ints 2 |> Seq.filter is_prime

let () =
  Printf.printf "Naturals: ";
  Seq.take 5 naturals |> Seq.iter (Printf.printf "%d ");
  Printf.printf "\nFibs: ";
  Seq.take 10 fibs |> Seq.iter (Printf.printf "%d ");
  Printf.printf "\nPrimes: ";
  Seq.take 10 primes |> Seq.iter (Printf.printf "%d ");
  print_newline ()
```
**Status:** [x]

### 051: Map Module — Frequency Counter
**Source:** https://ocaml.org/docs/
**Topic:** Using Map.Make for word frequency counting
**Difficulty:** Beginner
**Category:** Data structures
**OCaml:**
```ocaml
module SMap = Map.Make(String)

let word_freq text =
  text |> String.split_on_char ' '
  |> List.map String.lowercase_ascii
  |> List.fold_left (fun acc w ->
    let count = try SMap.find w acc with Not_found -> 0 in
    SMap.add w (count + 1) acc
  ) SMap.empty

let () =
  let freq = word_freq "the cat sat on the mat the cat" in
  SMap.iter (Printf.printf "%s: %d\n") freq
```
**Status:** [x]

### 052: Unfold — Generating Sequences from Seeds
**Source:** https://cs3110.github.io/textbook/chapters/ds/streams.html
**Topic:** The dual of fold: building lists from a seed value
**Difficulty:** Intermediate
**Category:** Higher-order functions
**OCaml:**
```ocaml
let rec unfold f seed = match f seed with
  | None -> []
  | Some (value, next_seed) -> value :: unfold f next_seed

let range a b =
  unfold (fun i -> if i > b then None else Some (i, i + 1)) a

let countdown n =
  unfold (fun i -> if i < 0 then None else Some (i, i - 1)) n

let collatz n =
  unfold (fun x ->
    if x = 1 then Some (1, 0)
    else if x = 0 then None
    else Some (x, if x mod 2 = 0 then x / 2 else 3 * x + 1)
  ) n

let () =
  List.iter (Printf.printf "%d ") (range 1 5);
  print_newline ();
  List.iter (Printf.printf "%d ") (collatz 6);
  print_newline ()
```
**Status:** [x]

### 053: Scan Left — Running Accumulation
**Source:** https://ocaml.org/docs/
**Topic:** scan_left returns all intermediate fold results
**Difficulty:** Beginner
**Category:** Higher-order functions
**OCaml:**
```ocaml
let scan_left f init lst =
  let _, result = List.fold_left (fun (acc, res) x ->
    let acc' = f acc x in
    (acc', acc' :: res)
  ) (init, [init]) lst in
  List.rev result

let running_sum = scan_left (+) 0
let running_max = scan_left max min_int

let () =
  List.iter (Printf.printf "%d ") (running_sum [1;2;3;4;5]);
  print_newline ();
  List.iter (Printf.printf "%d ") (running_max [3;1;4;1;5;9;2;6])
```
**Status:** [x]

### 054: Trie — Prefix Tree for Strings
**Source:** https://rosettacode.org/wiki/Trie#OCaml
**Topic:** Functional trie data structure for string lookup
**Difficulty:** Advanced
**Category:** Trees
**OCaml:**
```ocaml
module CMap = Map.Make(Char)

type trie = { is_word: bool; children: trie CMap.t }

let empty = { is_word = false; children = CMap.empty }

let insert word trie =
  let rec go i node =
    if i = String.length word then { node with is_word = true }
    else
      let c = word.[i] in
      let child = try CMap.find c node.children with Not_found -> empty in
      { node with children = CMap.add c (go (i+1) child) node.children }
  in go 0 trie

let mem word trie =
  let rec go i node =
    if i = String.length word then node.is_word
    else match CMap.find_opt word.[i] node.children with
    | None -> false | Some child -> go (i+1) child
  in go 0 trie

let () =
  let t = List.fold_left (fun t w -> insert w t)
    empty ["cat";"car";"card";"care";"dare"] in
  List.iter (fun w ->
    Printf.printf "%s: %b\n" w (mem w t)
  ) ["cat";"ca";"card";"dare";"dog"]
```
**Status:** [ ]

### 055: GCD and LCM — Euclidean Algorithm
**Source:** https://exercism.org/tracks/ocaml/exercises/grains
**Topic:** Recursive GCD with Euclid's algorithm
**Difficulty:** Beginner
**Category:** Math/recursion
**OCaml:**
```ocaml
let rec gcd a b = if b = 0 then a else gcd b (a mod b)
let lcm a b = if a = 0 || b = 0 then 0 else abs (a * b) / gcd a b

let gcd_list = function
  | [] -> 0
  | h :: t -> List.fold_left gcd h t

let () =
  Printf.printf "gcd(48,18) = %d\n" (gcd 48 18);
  Printf.printf "lcm(12,18) = %d\n" (lcm 12 18);
  Printf.printf "gcd_list = %d\n" (gcd_list [48; 36; 60; 12])
```
**Status:** [x]

### 056: Matrix Operations — Functional 2D
**Source:** https://rosettacode.org/wiki/Matrix_transposition#OCaml
**Topic:** Matrix transpose and multiply using nested lists
**Difficulty:** Intermediate
**Category:** Math/recursion
**OCaml:**
```ocaml
let transpose matrix =
  match matrix with
  | [] -> []
  | _ -> List.init (List.length (List.hd matrix)) (fun i ->
    List.map (fun row -> List.nth row i) matrix)

let dot a b = List.fold_left2 (fun acc x y -> acc + x * y) 0 a b

let multiply a b =
  let bt = transpose b in
  List.map (fun row -> List.map (dot row) bt) a

let print_matrix m =
  List.iter (fun row ->
    List.iter (Printf.printf "%3d ") row;
    print_newline ()
  ) m

let () =
  let a = [[1;2;3];[4;5;6]] in
  let b = [[7;8];[9;10];[11;12]] in
  print_matrix (multiply a b)
```
**Status:** [x]

### 057: Topological Sort — DAG Ordering
**Source:** https://rosettacode.org/wiki/Topological_sort#OCaml
**Topic:** Topological sort on directed acyclic graph using DFS
**Difficulty:** Advanced
**Category:** Graphs
**OCaml:**
```ocaml
module SS = Set.Make(String)

let topo_sort edges =
  let neighbors node =
    List.filter_map (fun (a, b) -> if a = node then Some b else None) edges in
  let all_nodes = List.fold_left (fun s (a, b) -> SS.add a (SS.add b s)) SS.empty edges in
  let rec visit node (visited, order) =
    if SS.mem node visited then (visited, order)
    else
      let visited = SS.add node visited in
      let visited, order = List.fold_left (fun acc n ->
        visit n acc) (visited, order) (neighbors node) in
      (visited, node :: order)
  in
  let _, order = SS.fold (fun node acc -> visit node acc) all_nodes (SS.empty, []) in
  order

let () =
  let edges = [("a","b");("a","c");("b","d");("c","d");("d","e")] in
  List.iter (Printf.printf "%s ") (topo_sort edges)
```
**Status:** [x]

### 058: Interpreter — Simple Lambda Calculus
**Source:** https://cs3110.github.io/textbook/chapters/interp/substitution.html
**Topic:** Evaluating a tiny functional language with closures
**Difficulty:** Advanced
**Category:** Parsing
**OCaml:**
```ocaml
type expr =
  | Int of int | Var of string
  | Lam of string * expr | App of expr * expr
  | Add of expr * expr

type value = VInt of int | VClosure of string * expr * env
and env = (string * value) list

let rec eval env = function
  | Int n -> VInt n
  | Var x -> List.assoc x env
  | Lam (x, body) -> VClosure (x, body, env)
  | App (f, arg) ->
    let fv = eval env f in
    let av = eval env arg in
    (match fv with
     | VClosure (x, body, cenv) -> eval ((x, av) :: cenv) body
     | _ -> failwith "not a function")
  | Add (a, b) ->
    (match eval env a, eval env b with
     | VInt x, VInt y -> VInt (x + y)
     | _ -> failwith "type error")

let () =
  (* (\x -> x + 1) 41 *)
  let e = App (Lam ("x", Add (Var "x", Int 1)), Int 41) in
  match eval [] e with VInt n -> Printf.printf "%d\n" n | _ -> ()
```
**Status:** [x]

### 059: Catamorphism — Generalized Fold on ADTs
**Source:** https://cs3110.github.io/textbook/chapters/ds/algebraic.html
**Topic:** Catamorphism (generalized fold) pattern for any algebraic data type
**Difficulty:** Advanced
**Category:** Monadic patterns
**OCaml:**
```ocaml
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

(* The catamorphism replaces constructors with functions *)
let rec cata ~leaf ~node = function
  | Leaf -> leaf
  | Node (l, v, r) -> node (cata ~leaf ~node l) v (cata ~leaf ~node r)

let size = cata ~leaf:0 ~node:(fun l _ r -> 1 + l + r)
let sum = cata ~leaf:0 ~node:(fun l v r -> l + v + r)
let height = cata ~leaf:0 ~node:(fun l _ r -> 1 + max l r)
let mirror = cata ~leaf:Leaf ~node:(fun l v r -> Node (r, v, l))
let to_list = cata ~leaf:[] ~node:(fun l v r -> l @ [v] @ r)

let () =
  let t = Node (Node (Leaf, 1, Leaf), 2, Node (Leaf, 3, Leaf)) in
  Printf.printf "size=%d sum=%d height=%d\n" (size t) (sum t) (height t);
  List.iter (Printf.printf "%d ") (to_list (mirror t))
```
**Status:** [x]

### 060: Difference List — O(1) Append
**Source:** https://cs3110.github.io/textbook/chapters/ds/sequences.html
**Topic:** Difference lists for efficient concatenation
**Difficulty:** Intermediate
**Category:** Data structures
**OCaml:**
```ocaml
(* A difference list is a function from list to list *)
type 'a dlist = 'a list -> 'a list

let empty : 'a dlist = Fun.id
let singleton x : 'a dlist = fun rest -> x :: rest
let append (a : 'a dlist) (b : 'a dlist) : 'a dlist = fun rest -> a (b rest)
let of_list lst : 'a dlist = fun rest -> lst @ rest
let to_list (dl : 'a dlist) = dl []

let () =
  let a = of_list [1;2;3] in
  let b = of_list [4;5;6] in
  let c = singleton 7 in
  let result = append (append a b) c |> to_list in
  List.iter (Printf.printf "%d ") result
```
**Status:** [x]

### 061: Nucleotide Count — Bioinformatics
**Source:** https://exercism.org/tracks/ocaml/exercises/nucleotide-count
**Topic:** Counting character frequencies in DNA strings
**Difficulty:** Beginner
**Category:** String processing
**OCaml:**
```ocaml
module CMap = Map.Make(Char)

let nucleotide_count dna =
  let init = List.fold_left (fun m c -> CMap.add c 0 m)
    CMap.empty ['A';'C';'G';'T'] in
  String.fold_left (fun m c ->
    match CMap.find_opt c m with
    | Some n -> CMap.add c (n + 1) m
    | None -> failwith ("invalid nucleotide: " ^ String.make 1 c)
  ) init dna

let () =
  let counts = nucleotide_count "GATTACA" in
  CMap.iter (Printf.printf "%c: %d\n") counts
```
**Status:** [x]

### 062: Robot Simulator — State with Immutable Records
**Source:** https://exercism.org/tracks/ocaml/exercises/robot-simulator
**Topic:** Modeling state transitions with immutable records
**Difficulty:** Beginner
**Category:** Records and variants
**OCaml:**
```ocaml
type direction = North | East | South | West
type robot = { x: int; y: int; dir: direction }
type instruction = TurnLeft | TurnRight | Advance

let turn_right = function
  | North -> East | East -> South | South -> West | West -> North

let turn_left = function
  | North -> West | West -> South | South -> East | East -> North

let advance r = match r.dir with
  | North -> { r with y = r.y + 1 }
  | East -> { r with x = r.x + 1 }
  | South -> { r with y = r.y - 1 }
  | West -> { r with x = r.x - 1 }

let execute r = function
  | TurnLeft -> { r with dir = turn_left r.dir }
  | TurnRight -> { r with dir = turn_right r.dir }
  | Advance -> advance r

let run r instructions = List.fold_left execute r instructions

let () =
  let r = { x=0; y=0; dir=North } in
  let r = run r [Advance; TurnRight; Advance; Advance; TurnLeft; Advance] in
  Printf.printf "(%d, %d)\n" r.x r.y
```
**Status:** [x]

### 063: Phone Number Parser — Validation Pipeline
**Source:** https://exercism.org/tracks/ocaml/exercises/phone-number
**Topic:** Chaining validations with Result type
**Difficulty:** Intermediate
**Category:** Error handling
**OCaml:**
```ocaml
let digits_only s =
  String.to_seq s |> Seq.filter (fun c -> c >= '0' && c <= '9')
  |> String.of_seq

let validate s =
  let d = digits_only s in
  let n = String.length d in
  if n = 11 && d.[0] = '1' then Ok (String.sub d 1 10)
  else if n = 10 then Ok d
  else Error "wrong number of digits"
  |> Result.bind (fun d ->
    if d.[0] = '0' || d.[0] = '1' then Error "invalid area code"
    else Ok d)
  |> Result.bind (fun d ->
    if d.[3] = '0' || d.[3] = '1' then Error "invalid exchange"
    else Ok d)

let () =
  List.iter (fun s -> match validate s with
    | Ok n -> Printf.printf "%s -> %s\n" s n
    | Error e -> Printf.printf "%s -> Error: %s\n" s e
  ) ["(223) 456-7890"; "1-223-456-7890"; "(023) 456-7890"]
```
**Status:** [x]

### 064: Accumulate — Custom Map
**Source:** https://exercism.org/tracks/ocaml/exercises/accumulate
**Topic:** Implementing map without using List.map
**Difficulty:** Beginner
**Category:** Higher-order functions
**OCaml:**
```ocaml
let rec accumulate f = function
  | [] -> []
  | h :: t -> f h :: accumulate f t

(* Tail-recursive version *)
let accumulate_tr f lst =
  let rec go acc = function
    | [] -> List.rev acc
    | h :: t -> go (f h :: acc) t
  in go [] lst

let () =
  accumulate (fun x -> x * x) [1;2;3;4;5]
  |> List.iter (Printf.printf "%d ");
  print_newline ();
  accumulate String.uppercase_ascii ["hello";"world"]
  |> List.iter (Printf.printf "%s ")
```
**Status:** [x]

### 065: Space Age — Float Computation
**Source:** https://exercism.org/tracks/ocaml/exercises/space-age
**Topic:** Computation with named constants and pattern matching
**Difficulty:** Beginner
**Category:** Records and variants
**OCaml:**
```ocaml
type planet = Mercury | Venus | Earth | Mars | Jupiter | Saturn | Uranus | Neptune

let orbital_period = function
  | Mercury -> 0.2408467 | Venus -> 0.61519726 | Earth -> 1.0
  | Mars -> 1.8808158 | Jupiter -> 11.862615 | Saturn -> 29.447498
  | Uranus -> 84.016846 | Neptune -> 164.79132

let earth_year_seconds = 31557600.0

let age_on planet seconds =
  seconds /. (earth_year_seconds *. orbital_period planet)

let () =
  let seconds = 1_000_000_000.0 in
  let planets = [Mercury;Venus;Earth;Mars;Jupiter;Saturn;Uranus;Neptune] in
  List.iter (fun p ->
    Printf.printf "%.2f years on planet\n" (age_on p seconds)
  ) planets
```
**Status:** [x]

### 066: Difference of Squares
**Source:** https://exercism.org/tracks/ocaml/exercises/difference-of-squares
**Topic:** Mathematical computation with higher-order functions
**Difficulty:** Beginner
**Category:** Math/recursion
**OCaml:**
```ocaml
let square_of_sum n =
  let s = List.init n (fun i -> i + 1) |> List.fold_left (+) 0 in
  s * s

let sum_of_squares n =
  List.init n (fun i -> i + 1)
  |> List.fold_left (fun acc x -> acc + x * x) 0

let difference n = square_of_sum n - sum_of_squares n

let () =
  for n = 1 to 10 do
    Printf.printf "n=%d: sq_sum=%d sum_sq=%d diff=%d\n"
      n (square_of_sum n) (sum_of_squares n) (difference n)
  done
```
**Status:** [x]

### 067: Allergies — Bitflag Decoding
**Source:** https://exercism.org/tracks/ocaml/exercises/allergies
**Topic:** Bitwise operations with variant types
**Difficulty:** Intermediate
**Category:** Records and variants
**OCaml:**
```ocaml
type allergen = Eggs | Peanuts | Shellfish | Strawberries
  | Tomatoes | Chocolate | Pollen | Cats

let allergen_score = function
  | Eggs -> 1 | Peanuts -> 2 | Shellfish -> 4 | Strawberries -> 8
  | Tomatoes -> 16 | Chocolate -> 32 | Pollen -> 64 | Cats -> 128

let all = [Eggs;Peanuts;Shellfish;Strawberries;Tomatoes;Chocolate;Pollen;Cats]

let is_allergic_to allergen score =
  score land allergen_score allergen <> 0

let allergies score =
  List.filter (fun a -> is_allergic_to a score) all

let name = function
  | Eggs -> "eggs" | Peanuts -> "peanuts" | Shellfish -> "shellfish"
  | Strawberries -> "strawberries" | Tomatoes -> "tomatoes"
  | Chocolate -> "chocolate" | Pollen -> "pollen" | Cats -> "cats"

let () =
  let score = 34 in
  Printf.printf "Score %d: " score;
  allergies score |> List.iter (fun a -> Printf.printf "%s " (name a))
```
**Status:** [x]

### 068: Bob — String Pattern Matching
**Source:** https://exercism.org/tracks/ocaml/exercises/bob
**Topic:** String analysis with multiple conditions
**Difficulty:** Beginner
**Category:** String processing
**OCaml:**
```ocaml
let is_question s = String.length (String.trim s) > 0 &&
  String.get (String.trim s) (String.length (String.trim s) - 1) = '?'

let is_yelling s =
  let has_letter = String.to_seq s |> Seq.exists (fun c ->
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) in
  has_letter && String.uppercase_ascii s = s

let is_silence s = String.trim s = ""

let response_for s =
  match is_silence s, is_yelling s, is_question s with
  | true, _, _ -> "Fine. Be that way!"
  | _, true, true -> "Calm down, I know what I'm doing!"
  | _, true, false -> "Whoa, chill out!"
  | _, false, true -> "Sure."
  | _ -> "Whatever."

let () =
  List.iter (fun s ->
    Printf.printf "%s -> %s\n" s (response_for s)
  ) ["WATCH OUT!"; "Does this work?"; "WHAT?!"; "   "; "Hi"]
```
**Status:** [x]

### 069: Perfect Numbers — Classification
**Source:** https://exercism.org/tracks/ocaml/exercises/perfect-numbers
**Topic:** Number classification using sum of divisors
**Difficulty:** Beginner
**Category:** Math/recursion
**OCaml:**
```ocaml
type classification = Perfect | Abundant | Deficient | Invalid

let sum_of_divisors n =
  List.init (n - 1) (fun i -> i + 1)
  |> List.filter (fun d -> n mod d = 0)
  |> List.fold_left (+) 0

let classify n =
  if n <= 0 then Invalid
  else
    let s = sum_of_divisors n in
    if s = n then Perfect
    else if s > n then Abundant
    else Deficient

let name = function
  | Perfect -> "Perfect" | Abundant -> "Abundant"
  | Deficient -> "Deficient" | Invalid -> "Invalid"

let () =
  [6; 28; 12; 7; -1] |> List.iter (fun n ->
    Printf.printf "%d: %s\n" n (name (classify n)))
```
**Status:** [x]

### 070: Raindrops — FizzBuzz Variant
**Source:** https://exercism.org/tracks/ocaml/exercises/raindrops
**Topic:** Conditional string building with fold
**Difficulty:** Beginner
**Category:** Higher-order functions
**OCaml:**
```ocaml
let convert n =
  let rules = [(3, "Pling"); (5, "Plang"); (7, "Plong")] in
  let result = List.fold_left (fun acc (divisor, sound) ->
    if n mod divisor = 0 then acc ^ sound else acc
  ) "" rules in
  if result = "" then string_of_int n else result

let () =
  List.iter (fun n ->
    Printf.printf "%d: %s\n" n (convert n)
  ) [1; 3; 5; 7; 15; 21; 35; 105; 11]
```
**Status:** [x]

### 071: Protein Translation — State Machine
**Source:** https://exercism.org/tracks/ocaml/exercises/protein-translation
**Topic:** Codon-to-protein translation with stop codons
**Difficulty:** Intermediate
**Category:** State machines
**OCaml:**
```ocaml
let codon_to_protein = function
  | "AUG" -> Some "Methionine"
  | "UUU" | "UUC" -> Some "Phenylalanine"
  | "UUA" | "UUG" -> Some "Leucine"
  | "UCU" | "UCC" | "UCA" | "UCG" -> Some "Serine"
  | "UAU" | "UAC" -> Some "Tyrosine"
  | "UGU" | "UGC" -> Some "Cysteine"
  | "UGG" -> Some "Tryptophan"
  | "UAA" | "UAG" | "UGA" -> None  (* STOP *)
  | _ -> failwith "invalid codon"

let translate rna =
  let rec go i acc =
    if i + 3 > String.length rna then List.rev acc
    else
      let codon = String.sub rna i 3 in
      match codon_to_protein codon with
      | None -> List.rev acc
      | Some protein -> go (i + 3) (protein :: acc)
  in go 0 []

let () =
  translate "AUGUUUUCUUAAAUG"
  |> List.iter (Printf.printf "%s ")
```
**Status:** [x]

### 072: ETL — Extract-Transform-Load
**Source:** https://exercism.org/tracks/ocaml/exercises/etl
**Topic:** Transforming data representations using fold
**Difficulty:** Beginner
**Category:** Higher-order functions
**OCaml:**
```ocaml
module SMap = Map.Make(String)

(* Old format: score -> letters. New format: letter -> score *)
let transform old_data =
  List.fold_left (fun acc (score, letters) ->
    List.fold_left (fun acc letter ->
      SMap.add (String.lowercase_ascii letter) score acc
    ) acc letters
  ) SMap.empty old_data

let () =
  let old = [(1, ["A";"E";"I"]); (2, ["D";"G"]); (3, ["B";"C"])] in
  let new_data = transform old in
  SMap.iter (Printf.printf "%s: %d\n") new_data
```
**Status:** [x]

### 073: Luhn Algorithm — Credit Card Validation
**Source:** https://exercism.org/tracks/ocaml/exercises/luhn
**Topic:** Digit transformation pipeline for checksum validation
**Difficulty:** Intermediate
**Category:** Math/recursion
**OCaml:**
```ocaml
let luhn s =
  let digits = String.to_seq s
    |> Seq.filter (fun c -> c <> ' ')
    |> List.of_seq in
  if List.length digits <= 1 then false
  else if List.exists (fun c -> c < '0' || c > '9') digits then false
  else
    let nums = List.rev_map (fun c -> Char.code c - Char.code '0') digits in
    let sum = List.mapi (fun i d ->
      if i mod 2 = 1 then
        let d2 = d * 2 in
        if d2 > 9 then d2 - 9 else d2
      else d
    ) nums |> List.fold_left (+) 0 in
    sum mod 10 = 0

let () =
  List.iter (fun s ->
    Printf.printf "%s: %b\n" s (luhn s)
  ) ["4539 3195 0343 6467"; "8273 1232 7352 0569"; "0000"; "1234"]
```
**Status:** [x]

### 074: Roman Numerals — Greedy Algorithm
**Source:** https://exercism.org/tracks/ocaml/exercises/roman-numerals
**Topic:** Greedy decomposition using a lookup table
**Difficulty:** Beginner
**Category:** Math/recursion
**OCaml:**
```ocaml
let to_roman n =
  let table = [
    (1000,"M");(900,"CM");(500,"D");(400,"CD");
    (100,"C");(90,"XC");(50,"L");(40,"XL");
    (10,"X");(9,"IX");(5,"V");(4,"IV");(1,"I")
  ] in
  let buf = Buffer.create 16 in
  let n = ref n in
  List.iter (fun (value, symbol) ->
    while !n >= value do
      Buffer.add_string buf symbol;
      n := !n - value
    done
  ) table;
  Buffer.contents buf

let () =
  [1; 4; 9; 14; 42; 99; 1994; 2024]
  |> List.iter (fun n -> Printf.printf "%d = %s\n" n (to_roman n))
```
**Status:** [x]

### 075: Flatten Array — Recursive Nested Structure
**Source:** https://exercism.org/tracks/ocaml/exercises/flatten-array
**Topic:** Flattening arbitrarily nested structures with variants
**Difficulty:** Intermediate
**Category:** Records and variants
**OCaml:**
```ocaml
type 'a nested = Val of 'a | Null | List of 'a nested list

let rec flatten = function
  | Val x -> [x]
  | Null -> []
  | List items -> List.concat_map flatten items

let () =
  let nested = List [
    Val 1;
    List [Val 2; Null; List [Val 3; Val 4]; Null];
    Val 5;
    List [List [Val 6]]
  ] in
  flatten nested |> List.iter (Printf.printf "%d ")
```
**Status:** [ ]

### 076: Sublist — List Relationship Classification
**Source:** https://exercism.org/tracks/ocaml/exercises/sublist
**Topic:** Classifying list relationships (equal, sublist, superlist)
**Difficulty:** Intermediate
**Category:** Data structures
**OCaml:**
```ocaml
type relation = Equal | Sublist | Superlist | Unequal

let rec starts_with lst prefix = match lst, prefix with
  | _, [] -> true
  | [], _ -> false
  | h1 :: t1, h2 :: t2 -> h1 = h2 && starts_with t1 t2

let rec is_sublist sub lst = match lst with
  | [] -> sub = []
  | _ :: t -> starts_with lst sub || is_sublist sub t

let classify a b =
  if a = b then Equal
  else if is_sublist a b then Sublist
  else if is_sublist b a then Superlist
  else Unequal

let name = function
  | Equal -> "equal" | Sublist -> "sublist"
  | Superlist -> "superlist" | Unequal -> "unequal"

let () =
  Printf.printf "%s\n" (name (classify [1;2;3] [0;1;2;3;4]));
  Printf.printf "%s\n" (name (classify [0;1;2;3;4] [1;2;3]))
```
**Status:** [ ]

### 077: Zipper on Trees — Navigating Binary Trees
**Source:** https://cs3110.github.io/textbook/chapters/ds/zippers.html
**Topic:** Tree zipper for O(1) local tree navigation and editing
**Difficulty:** Advanced
**Category:** Data structures
**OCaml:**
```ocaml
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

type 'a crumb = Left of 'a * 'a tree | Right of 'a tree * 'a
type 'a zipper = { focus: 'a tree; trail: 'a crumb list }

let of_tree t = { focus = t; trail = [] }

let go_left z = match z.focus with
  | Leaf -> None
  | Node (l, v, r) -> Some { focus = l; trail = Left (v, r) :: z.trail }

let go_right z = match z.focus with
  | Leaf -> None
  | Node (l, v, r) -> Some { focus = r; trail = Right (l, v) :: z.trail }

let go_up z = match z.trail with
  | [] -> None
  | Left (v, r) :: rest -> Some { focus = Node (z.focus, v, r); trail = rest }
  | Right (l, v) :: rest -> Some { focus = Node (l, v, z.focus); trail = rest }

let set_value x z = match z.focus with
  | Leaf -> z
  | Node (l, _, r) -> { z with focus = Node (l, x, r) }

let rec to_tree z = match go_up z with None -> z.focus | Some z' -> to_tree z'
```
**Status:** [ ]

### 078: Fibonacci Variants — Multiple Approaches
**Source:** https://cs3110.github.io/textbook/chapters/basics/functions.html
**Topic:** Comparing direct, tail-recursive, and fold-based Fibonacci
**Difficulty:** Beginner
**Category:** Math/recursion
**OCaml:**
```ocaml
(* Direct recursion *)
let rec fib_naive = function
  | 0 -> 0 | 1 -> 1
  | n -> fib_naive (n-1) + fib_naive (n-2)

(* Tail-recursive with accumulator *)
let fib_tail n =
  let rec go a b = function
    | 0 -> a
    | n -> go b (a + b) (n - 1)
  in go 0 1 n

(* Using fold *)
let fib_fold n =
  let a, _ = List.init n Fun.id
    |> List.fold_left (fun (a, b) _ -> (b, a + b)) (0, 1)
  in a

let () =
  for i = 0 to 10 do
    Printf.printf "fib(%d) = %d %d %d\n"
      i (fib_naive i) (fib_tail i) (fib_fold i)
  done
```
**Status:** [ ]

### 079: Binary to Decimal — Fold on Digits
**Source:** https://exercism.org/tracks/ocaml/exercises/binary
**Topic:** Converting binary string to integer using fold
**Difficulty:** Beginner
**Category:** Math/recursion
**OCaml:**
```ocaml
let binary_to_decimal s =
  String.fold_left (fun acc c ->
    match c with
    | '0' -> acc * 2
    | '1' -> acc * 2 + 1
    | _ -> failwith "invalid binary digit"
  ) 0 s

let decimal_to_binary n =
  if n = 0 then "0"
  else
    let rec go n acc =
      if n = 0 then acc
      else go (n / 2) (string_of_int (n mod 2) ^ acc)
    in go n ""

let () =
  List.iter (fun s ->
    let d = binary_to_decimal s in
    Printf.printf "%s -> %d -> %s\n" s d (decimal_to_binary d)
  ) ["1010"; "11111"; "10000000"; "101010"]
```
**Status:** [ ]

### 080: Yacht — Dice Scoring with Pattern Matching
**Source:** https://exercism.org/tracks/ocaml/exercises/yacht
**Topic:** Complex scoring rules encoded as pattern matching
**Difficulty:** Intermediate
**Category:** Records and variants
**OCaml:**
```ocaml
type category = Ones | Twos | Threes | Fours | Fives | Sixes
  | FullHouse | FourOfAKind | LittleStraight | BigStraight | Yacht | Choice

let count dice n = List.length (List.filter ((=) n) dice)

let score dice = function
  | Ones -> count dice 1 | Twos -> 2 * count dice 2
  | Threes -> 3 * count dice 3 | Fours -> 4 * count dice 4
  | Fives -> 5 * count dice 5 | Sixes -> 6 * count dice 6
  | Choice -> List.fold_left (+) 0 dice
  | Yacht -> if List.for_all ((=) (List.hd dice)) dice then 50 else 0
  | FullHouse ->
    let sorted = List.sort compare dice in
    (match sorted with
     | [a;b;c;d;e] when a=b && b=c && d=e && c<>d -> List.fold_left (+) 0 dice
     | [a;b;c;d;e] when a=b && c=d && d=e && b<>c -> List.fold_left (+) 0 dice
     | _ -> 0)
  | FourOfAKind ->
    let v = List.find (fun n -> count dice n >= 4) (List.sort_uniq compare dice) in
    4 * v
  | LittleStraight ->
    if List.sort compare dice = [1;2;3;4;5] then 30 else 0
  | BigStraight ->
    if List.sort compare dice = [2;3;4;5;6] then 30 else 0

let () =
  Printf.printf "Yacht: %d\n" (score [5;5;5;5;5] Yacht);
  Printf.printf "Full: %d\n" (score [2;2;3;3;3] FullHouse);
  Printf.printf "Choice: %d\n" (score [1;2;3;4;5] Choice)
```
**Status:** [ ]

### 081: Parallel Letter Frequency — Map-Reduce Pattern
**Source:** https://exercism.org/tracks/ocaml/exercises/parallel-letter-frequency
**Topic:** Map-reduce pattern for combining frequency maps
**Difficulty:** Intermediate
**Category:** Higher-order functions
**OCaml:**
```ocaml
module CMap = Map.Make(Char)

let letter_freq s =
  String.fold_left (fun m c ->
    let c = Char.lowercase_ascii c in
    if c >= 'a' && c <= 'z' then
      CMap.update c (function None -> Some 1 | Some n -> Some (n+1)) m
    else m
  ) CMap.empty s

let merge_maps =
  CMap.union (fun _ a b -> Some (a + b))

let parallel_frequency texts =
  texts
  |> List.map letter_freq
  |> List.fold_left merge_maps CMap.empty

let () =
  let texts = ["Hello World"; "Functional Programming"; "OCaml is Great"] in
  let freq = parallel_frequency texts in
  CMap.iter (Printf.printf "%c:%d ") freq
```
**Status:** [ ]

### 082: Circular Buffer — Functional Queue
**Source:** https://dev.realworldocaml.org/
**Topic:** Amortized O(1) functional queue using two lists
**Difficulty:** Intermediate
**Category:** Data structures
**OCaml:**
```ocaml
type 'a queue = { front: 'a list; back: 'a list }

let empty = { front = []; back = [] }
let is_empty q = q.front = [] && q.back = []

let enqueue x q = { q with back = x :: q.back }

let dequeue q = match q.front with
  | h :: t -> Some (h, { q with front = t })
  | [] -> match List.rev q.back with
    | [] -> None
    | h :: t -> Some (h, { front = t; back = [] })

let to_list q = q.front @ List.rev q.back

let () =
  let q = empty |> enqueue 1 |> enqueue 2 |> enqueue 3 in
  let rec drain q = match dequeue q with
    | None -> ()
    | Some (x, q') -> Printf.printf "%d " x; drain q'
  in drain q
```
**Status:** [ ]

### 083: Sierpinski Triangle — Recursive ASCII Art
**Source:** https://rosettacode.org/wiki/Sierpinski_triangle#OCaml
**Topic:** Recursive fractal generation with string manipulation
**Difficulty:** Intermediate
**Category:** Math/recursion
**OCaml:**
```ocaml
let sierpinski n =
  let rec go n =
    if n = 0 then ["*"]
    else
      let prev = go (n - 1) in
      let width = 1 lsl n - 1 in (* 2^n - 1 *)
      let pad s = String.make ((width - String.length s) / 2) ' ' ^ s in
      let top = List.map pad prev in
      let bottom = List.map (fun s -> s ^ " " ^ s) prev in
      top @ bottom
  in
  List.iter print_endline (go n)

let () = sierpinski 4
```
**Status:** [ ]

### 084: Pascal's Triangle — Row Generation
**Source:** https://exercism.org/tracks/ocaml/exercises/pascals-triangle
**Topic:** Generating Pascal's triangle rows using zip-with-add
**Difficulty:** Beginner
**Category:** Math/recursion
**OCaml:**
```ocaml
let next_row row =
  List.map2 (+) (0 :: row) (row @ [0])

let pascal n =
  let rec go row i =
    if i > n then []
    else row :: go (next_row row) (i + 1)
  in go [1] 1

let () =
  pascal 8 |> List.iter (fun row ->
    List.iter (Printf.printf "%d ") row;
    print_newline ()
  )
```
**Status:** [ ]

### 085: Hamming Distance on Lists — Generic Zip
**Source:** https://exercism.org/tracks/ocaml/exercises/hamming
**Topic:** Zip-based comparison of two sequences
**Difficulty:** Beginner
**Category:** Higher-order functions
**OCaml:**
```ocaml
let hamming s1 s2 =
  if String.length s1 <> String.length s2 then
    Error "strands must be of equal length"
  else
    let dist = ref 0 in
    String.iteri (fun i c ->
      if c <> s2.[i] then incr dist
    ) s1;
    Ok !dist

(* Pure functional version *)
let hamming_fp s1 s2 =
  if String.length s1 <> String.length s2 then Error "unequal"
  else
    Ok (Seq.zip (String.to_seq s1) (String.to_seq s2)
    |> Seq.fold_left (fun acc (a, b) -> if a <> b then acc + 1 else acc) 0)

let () =
  match hamming "GAGCCTACTAACGGGAT" "CATCGTAATGACGGCCT" with
  | Ok d -> Printf.printf "Hamming distance: %d\n" d
  | Error e -> Printf.printf "Error: %s\n" e
```
**Status:** [ ]

### 086: Sum of Multiples — Set Union
**Source:** https://exercism.org/tracks/ocaml/exercises/sum-of-multiples
**Topic:** Collecting multiples into a set to avoid double-counting
**Difficulty:** Beginner
**Category:** Math/recursion
**OCaml:**
```ocaml
module IS = Set.Make(Int)

let sum_of_multiples factors limit =
  List.fold_left (fun s factor ->
    if factor = 0 then s
    else
      let multiples = List.init ((limit - 1) / factor) (fun i -> factor * (i + 1)) in
      List.fold_left (fun s m -> IS.add m s) s multiples
  ) IS.empty factors
  |> IS.fold (+) |> fun f -> f 0

let () =
  Printf.printf "%d\n" (sum_of_multiples [3; 5] 1000);
  Printf.printf "%d\n" (sum_of_multiples [2; 3; 5; 7; 11] 10000)
```
**Status:** [ ]

### 087: Series — Sliding Window on Strings
**Source:** https://exercism.org/tracks/ocaml/exercises/series
**Topic:** Generating sliding windows over a string
**Difficulty:** Beginner
**Category:** String processing
**OCaml:**
```ocaml
let series n s =
  if n > String.length s then []
  else
    List.init (String.length s - n + 1) (fun i ->
      String.sub s i n
    )

let largest_product n s =
  if n = 0 then Ok 1
  else if n > String.length s then Error "span too large"
  else
    series n s
    |> List.map (fun sub ->
      String.fold_left (fun acc c ->
        acc * (Char.code c - Char.code '0')
      ) 1 sub
    )
    |> List.fold_left max 0
    |> fun m -> Ok m

let () =
  List.iter (Printf.printf "%s ") (series 3 "49142");
  print_newline ();
  match largest_product 2 "0123456789" with
  | Ok n -> Printf.printf "Largest: %d\n" n
  | Error e -> Printf.printf "Error: %s\n" e
```
**Status:** [ ]

### 088: Atbash Cipher — Char Mapping
**Source:** https://exercism.org/tracks/ocaml/exercises/atbash-cipher
**Topic:** Character transposition cipher with grouping
**Difficulty:** Beginner
**Category:** String processing
**OCaml:**
```ocaml
let atbash_char c =
  if c >= 'a' && c <= 'z' then
    Char.chr (Char.code 'z' - (Char.code c - Char.code 'a'))
  else if c >= '0' && c <= '9' then c
  else '\x00'

let encode s =
  let chars = String.to_seq (String.lowercase_ascii s)
    |> Seq.filter_map (fun c ->
      let c' = atbash_char c in
      if c' <> '\x00' then Some c' else None)
    |> List.of_seq in
  let rec group i = function
    | [] -> []
    | cs ->
      let chunk = List.filteri (fun j _ -> j < 5) cs in
      let rest = List.filteri (fun j _ -> j >= 5) cs in
      (String.init (List.length chunk) (List.nth chunk))
      :: group (i + 1) rest
  in
  String.concat " " (group 0 chars)

let () = Printf.printf "%s\n" (encode "Testing, 1 2 3, testing.")
```
**Status:** [ ]

### 089: Poker Hand Evaluator — Complex Pattern Matching
**Source:** https://rosettacode.org/wiki/Poker_hand_analyser#OCaml
**Topic:** Multi-level pattern matching for hand classification
**Difficulty:** Advanced
**Category:** Records and variants
**OCaml:**
```ocaml
type rank = int
type hand_type = HighCard | Pair | TwoPair | ThreeKind | Straight
  | Flush | FullHouse | FourKind | StraightFlush

let classify (ranks : rank list) (is_flush : bool) =
  let sorted = List.sort (fun a b -> compare b a) ranks in
  let counts = List.sort (fun a b -> compare b a)
    (List.map (fun r -> List.length (List.filter ((=) r) sorted))
      (List.sort_uniq compare sorted)) in
  let is_straight = match sorted with
    | [a;b;c;d;e] -> a - e = 4 && List.length (List.sort_uniq compare sorted) = 5
    | _ -> false in
  match is_flush, is_straight, counts with
  | true, true, _ -> StraightFlush
  | _, _, 4 :: _ -> FourKind
  | _, _, [3; 2] -> FullHouse
  | true, _, _ -> Flush
  | _, true, _ -> Straight
  | _, _, 3 :: _ -> ThreeKind
  | _, _, [2; 2; 1] -> TwoPair
  | _, _, 2 :: _ -> Pair
  | _ -> HighCard

let name = function
  | StraightFlush -> "Straight Flush" | FourKind -> "Four of a Kind"
  | FullHouse -> "Full House" | Flush -> "Flush" | Straight -> "Straight"
  | ThreeKind -> "Three of a Kind" | TwoPair -> "Two Pair"
  | Pair -> "Pair" | HighCard -> "High Card"

let () =
  Printf.printf "%s\n" (name (classify [10;11;12;13;14] true));
  Printf.printf "%s\n" (name (classify [3;3;3;7;7] false))
```
**Status:** [ ]

### 090: Frequency Analysis — Letter Distribution
**Source:** https://rosettacode.org/wiki/Letter_frequency#OCaml
**Topic:** Character frequency analysis with sorting
**Difficulty:** Beginner
**Category:** String processing
**OCaml:**
```ocaml
module CMap = Map.Make(Char)

let frequency s =
  String.fold_left (fun m c ->
    let c = Char.lowercase_ascii c in
    if c >= 'a' && c <= 'z' then
      CMap.update c (function None -> Some 1 | Some n -> Some (n+1)) m
    else m
  ) CMap.empty s

let sorted_freq s =
  frequency s |> CMap.bindings
  |> List.sort (fun (_, a) (_, b) -> compare b a)

let () =
  let text = "The quick brown fox jumps over the lazy dog" in
  sorted_freq text |> List.iter (fun (c, n) ->
    Printf.printf "%c: %s (%d)\n" c (String.make n '#') n)
```
**Status:** [ ]

### 091: Y Combinator — Anonymous Recursion
**Source:** https://rosettacode.org/wiki/Y_combinator#OCaml
**Topic:** Fixed-point combinator for anonymous recursive functions
**Difficulty:** Advanced
**Category:** Higher-order functions
**OCaml:**
```ocaml
(* OCaml needs a recursive type wrapper *)
type 'a fix = Fix of ('a fix -> 'a)

let y f =
  let g (Fix x as w) = f (fun a -> x w a) in
  g (Fix g)

let factorial = y (fun self n ->
  if n = 0 then 1 else n * self (n - 1))

let fibonacci = y (fun self n ->
  if n <= 1 then n else self (n - 1) + self (n - 2))

let () =
  Printf.printf "10! = %d\n" (factorial 10);
  Printf.printf "fib(10) = %d\n" (fibonacci 10)
```
**Status:** [ ]

### 092: Phantom Type State Machine — File Handle
**Source:** https://dev.realworldocaml.org/
**Topic:** Using phantom types to enforce state transitions at compile time
**Difficulty:** Advanced
**Category:** Functors and modules
**OCaml:**
```ocaml
type opened
type closed

type 'state handle = { name: string; content: string list }

let open_file name : opened handle =
  { name; content = ["line1"; "line2"; "line3"] }

let read_line (h : opened handle) n : string =
  List.nth h.content n

let close_file (_ : opened handle) : closed handle =
  { name = "closed"; content = [] }

(* read_line on a closed handle would be a type error! *)
(* let _ = read_line (close_file (open_file "test")) 0 *)

let () =
  let f = open_file "data.txt" in
  Printf.printf "%s\n" (read_line f 0);
  Printf.printf "%s\n" (read_line f 1);
  let _closed = close_file f in
  Printf.printf "File safely closed\n"
```
**Status:** [ ]

### 093: Visitor Pattern via Fold — Expression Evaluator
**Source:** https://cs3110.github.io/textbook/chapters/interp/substitution.html
**Topic:** Using fold as a visitor pattern replacement
**Difficulty:** Intermediate
**Category:** Monadic patterns
**OCaml:**
```ocaml
type expr =
  | Lit of float
  | Add of expr * expr
  | Mul of expr * expr
  | Neg of expr

let rec fold ~lit ~add ~mul ~neg = function
  | Lit x -> lit x
  | Add (a, b) -> add (fold ~lit ~add ~mul ~neg a) (fold ~lit ~add ~mul ~neg b)
  | Mul (a, b) -> mul (fold ~lit ~add ~mul ~neg a) (fold ~lit ~add ~mul ~neg b)
  | Neg a -> neg (fold ~lit ~add ~mul ~neg a)

let eval = fold ~lit:Fun.id ~add:(+.) ~mul:( *.) ~neg:(fun x -> -.x)

let to_string = fold
  ~lit:string_of_float
  ~add:(fun a b -> "(" ^ a ^ " + " ^ b ^ ")")
  ~mul:(fun a b -> "(" ^ a ^ " * " ^ b ^ ")")
  ~neg:(fun a -> "(-" ^ a ^ ")")

let () =
  let e = Add (Mul (Lit 2.0, Lit 3.0), Neg (Lit 1.0)) in
  Printf.printf "%s = %g\n" (to_string e) (eval e)
```
**Status:** [ ]

### 094: Writer Monad — Logging Computation
**Source:** https://cs3110.github.io/textbook/chapters/ds/monads.html
**Topic:** Writer monad for accumulating logs alongside computation
**Difficulty:** Advanced
**Category:** Monadic patterns
**OCaml:**
```ocaml
type 'a writer = { value: 'a; log: string list }

let return x = { value = x; log = [] }
let bind w f =
  let w' = f w.value in
  { value = w'.value; log = w.log @ w'.log }

let tell msg = { value = (); log = [msg] }
let ( >>= ) = bind

let half x =
  { value = x / 2; log = [Printf.sprintf "halved %d to %d" x (x / 2)] }

let compute x =
  return x >>= fun n ->
  half n >>= fun n ->
  tell (Printf.sprintf "result is %d" n) >>= fun () ->
  return n

let () =
  let result = compute 100 in
  Printf.printf "Value: %d\n" result.value;
  List.iter (Printf.printf "  Log: %s\n") result.log
```
**Status:** [ ]

### 095: Topological Sort via Kahn's Algorithm
**Source:** https://rosettacode.org/wiki/Topological_sort#OCaml
**Topic:** Iterative topological sort using in-degree counting
**Difficulty:** Advanced
**Category:** Graphs
**OCaml:**
```ocaml
module SMap = Map.Make(String)
module SSet = Set.Make(String)

let kahn_sort nodes edges =
  let in_deg = List.fold_left (fun m (_, b) ->
    SMap.update b (function None -> Some 1 | Some n -> Some (n+1)) m
  ) (List.fold_left (fun m n -> SMap.add n 0 m) SMap.empty nodes) edges in
  let queue = SMap.fold (fun k v acc -> if v = 0 then k :: acc else acc) in_deg [] in
  let rec go queue in_deg result =
    match queue with
    | [] -> List.rev result
    | node :: rest ->
      let out_edges = List.filter (fun (a, _) -> a = node) edges in
      let in_deg, new_queue = List.fold_left (fun (deg, q) (_, b) ->
        let d = SMap.find b deg - 1 in
        let deg = SMap.add b d deg in
        if d = 0 then (deg, b :: q) else (deg, q)
      ) (in_deg, rest) out_edges in
      go new_queue in_deg (node :: result)
  in go queue in_deg []

let () =
  let nodes = ["a";"b";"c";"d";"e"] in
  let edges = [("a","b");("a","c");("b","d");("c","d");("d","e")] in
  List.iter (Printf.printf "%s ") (kahn_sort nodes edges)
```
**Status:** [ ]

### 096: Lenses — Functional Getters and Setters
**Source:** https://dev.realworldocaml.org/
**Topic:** Simple lens implementation for nested record updates
**Difficulty:** Advanced
**Category:** Higher-order functions
**OCaml:**
```ocaml
type ('s, 'a) lens = {
  get: 's -> 'a;
  set: 'a -> 's -> 's;
}

let compose outer inner = {
  get = (fun s -> inner.get (outer.get s));
  set = (fun a s -> outer.set (inner.set a (outer.get s)) s);
}

let over lens f s = lens.set (f (lens.get s)) s

type address = { street: string; city: string }
type person = { name: string; addr: address }

let addr_lens = { get = (fun p -> p.addr); set = (fun a p -> { p with addr = a }) }
let city_lens = { get = (fun a -> a.city); set = (fun c a -> { a with city = c }) }
let person_city = compose addr_lens city_lens

let () =
  let p = { name = "Alice"; addr = { street = "Main St"; city = "NYC" } } in
  Printf.printf "City: %s\n" (person_city.get p);
  let p = over person_city String.uppercase_ascii p in
  Printf.printf "City: %s\n" (person_city.get p)
```
**Status:** [ ]

### 097: Tail-Recursive Map with CPS
**Source:** https://cs3110.github.io/textbook/chapters/hop/cps.html
**Topic:** Making List.map stack-safe using continuations
**Difficulty:** Intermediate
**Category:** Higher-order functions
**OCaml:**
```ocaml
(* Naive map — not tail-recursive, stack overflow on large lists *)
let rec map_naive f = function
  | [] -> []
  | h :: t -> f h :: map_naive f t

(* Tail-recursive with reverse *)
let map_tr f lst =
  let rec go acc = function
    | [] -> List.rev acc
    | h :: t -> go (f h :: acc) t
  in go [] lst

(* CPS — tail-recursive, preserves order *)
let map_cps f lst =
  let rec go k = function
    | [] -> k []
    | h :: t -> go (fun rest -> k (f h :: rest)) t
  in go Fun.id lst

let () =
  let big = List.init 1_000_000 Fun.id in
  let result = map_tr (fun x -> x * 2) big in
  Printf.printf "Length: %d, first: %d, last: %d\n"
    (List.length result) (List.hd result) (List.nth result 999_999)
```
**Status:** [ ]

### 098: Red-Black Tree — Balanced Insert
**Source:** https://cs3110.github.io/textbook/chapters/ds/rb.html
**Topic:** Red-black tree with Okasaki's functional balancing
**Difficulty:** Advanced
**Category:** Trees
**OCaml:**
```ocaml
type color = Red | Black
type 'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree

let balance = function
  | Black, T (Red, T (Red, a, x, b), y, c), z, d
  | Black, T (Red, a, x, T (Red, b, y, c)), z, d
  | Black, a, x, T (Red, T (Red, b, y, c), z, d)
  | Black, a, x, T (Red, b, y, T (Red, c, z, d)) ->
    T (Red, T (Black, a, x, b), y, T (Black, c, z, d))
  | color, a, x, b -> T (color, a, x, b)

let insert x t =
  let rec ins = function
    | E -> T (Red, E, x, E)
    | T (color, a, y, b) ->
      if x < y then balance (color, ins a, y, b)
      else if x > y then balance (color, a, y, ins b)
      else T (color, a, y, b)
  in
  match ins t with T (_, a, y, b) -> T (Black, a, y, b) | E -> E

let rec mem x = function
  | E -> false
  | T (_, a, y, b) -> x = y || (if x < y then mem x a else mem x b)

let rec to_list = function
  | E -> [] | T (_, a, v, b) -> to_list a @ [v] @ to_list b

let () =
  let t = List.fold_left (fun t x -> insert x t) E [5;3;7;1;4;6;8;2;9] in
  List.iter (Printf.printf "%d ") (to_list t)
```
**Status:** [ ]

### 099: Monoid Pattern — Generic Combining
**Source:** https://dev.realworldocaml.org/
**Topic:** Monoid typeclass pattern using first-class modules
**Difficulty:** Advanced
**Category:** Functors and modules
**OCaml:**
```ocaml
module type MONOID = sig
  type t
  val empty : t
  val combine : t -> t -> t
end

let concat_all (type a) (module M : MONOID with type t = a) (lst : a list) =
  List.fold_left M.combine M.empty lst

module Sum = struct type t = int let empty = 0 let combine = (+) end
module Product = struct type t = int let empty = 1 let combine = ( * ) end
module Concat = struct type t = string let empty = "" let combine = (^) end
module All = struct type t = bool let empty = true let combine = (&&) end

let () =
  Printf.printf "sum: %d\n" (concat_all (module Sum) [1;2;3;4;5]);
  Printf.printf "product: %d\n" (concat_all (module Product) [1;2;3;4;5]);
  Printf.printf "concat: %s\n" (concat_all (module Concat) ["hello";" ";"world"]);
  Printf.printf "all: %b\n" (concat_all (module All) [true; true; false])
```
**Status:** [ ]

### 100: Dijkstra's Shortest Path — Priority Queue
**Source:** https://rosettacode.org/wiki/Dijkstra%27s_algorithm#OCaml
**Topic:** Shortest path algorithm with functional priority queue
**Difficulty:** Advanced
**Category:** Graphs
**OCaml:**
```ocaml
module PQ = Set.Make(struct
  type t = int * string
  let compare (d1,n1) (d2,n2) = compare (d1,n1) (d2,n2)
end)
module SMap = Map.Make(String)

let dijkstra graph start =
  let dist = SMap.singleton start 0 in
  let pq = PQ.singleton (0, start) in
  let rec go pq dist =
    if PQ.is_empty pq then dist
    else
      let (d, u) = PQ.min_elt pq in
      let pq = PQ.remove (d, u) pq in
      let neighbors = try SMap.find u graph with Not_found -> [] in
      let dist, pq = List.fold_left (fun (dist, pq) (v, w) ->
        let alt = d + w in
        let current = try SMap.find v dist with Not_found -> max_int in
        if alt < current then
          (SMap.add v alt dist, PQ.add (alt, v) pq)
        else (dist, pq)
      ) (dist, pq) neighbors in
      go pq dist
  in go pq dist

let () =
  let g = SMap.of_list [
    ("a",[("b",1);("c",4)]); ("b",[("c",2);("d",6)]);
    ("c",[("d",3)]); ("d",[])
  ] in
  let dist = dijkstra g "a" in
  SMap.iter (Printf.printf "%s: %d\n") dist
```
**Status:** [ ]

### 101: Huffman Encoding — Greedy Tree Building
**Source:** https://rosettacode.org/wiki/Huffman_coding#OCaml
**Topic:** Building a Huffman tree from character frequencies
**Difficulty:** Advanced
**Category:** Trees
**OCaml:**
```ocaml
type htree = Leaf of char * int | Node of htree * htree * int

let freq t = match t with Leaf (_,f) -> f | Node (_,_,f) -> f

let build_tree freqs =
  let trees = List.map (fun (c,f) -> Leaf (c,f)) freqs
    |> List.sort (fun a b -> compare (freq a) (freq b)) in
  let rec go = function
    | [t] -> t
    | a :: b :: rest ->
      let merged = Node (a, b, freq a + freq b) in
      let trees = List.sort (fun a b -> compare (freq a) (freq b)) (merged :: rest) in
      go trees
    | [] -> failwith "empty"
  in go trees

let rec codes prefix = function
  | Leaf (c, _) -> [(c, prefix)]
  | Node (l, r, _) -> codes (prefix ^ "0") l @ codes (prefix ^ "1") r

let () =
  let freqs = [('a',5);('b',9);('c',12);('d',13);('e',16);('f',45)] in
  let tree = build_tree freqs in
  codes "" tree |> List.iter (fun (c, code) ->
    Printf.printf "%c: %s\n" c code)
```
**Status:** [ ]

### 102: Persistent Vector — Functional Array
**Source:** https://cs3110.github.io/textbook/chapters/ds/sequences.html
**Topic:** Trie-based persistent vector for O(log n) random access
**Difficulty:** Advanced
**Category:** Data structures
**OCaml:**
```ocaml
(* Simplified persistent vector using balanced binary tree *)
type 'a pvec = Nil | One of 'a | Two of 'a pvec * 'a pvec

let rec size = function
  | Nil -> 0 | One _ -> 1
  | Two (l, r) -> size l + size r

let rec get i = function
  | One x -> if i = 0 then x else failwith "index"
  | Two (l, r) ->
    let ls = size l in
    if i < ls then get i l else get (i - ls) r
  | Nil -> failwith "empty"

let rec set i v = function
  | One _ -> if i = 0 then One v else failwith "index"
  | Two (l, r) ->
    let ls = size l in
    if i < ls then Two (set i v l, r)
    else Two (l, set (i - ls) v r)
  | Nil -> failwith "empty"

let of_list lst =
  let rec build = function
    | [] -> Nil | [x] -> One x
    | lst ->
      let n = List.length lst in
      let left = List.filteri (fun i _ -> i < n/2) lst in
      let right = List.filteri (fun i _ -> i >= n/2) lst in
      Two (build left, build right)
  in build lst

let () =
  let v = of_list [10;20;30;40;50] in
  Printf.printf "v[2] = %d\n" (get 2 v);
  let v = set 2 99 v in
  Printf.printf "v[2] = %d\n" (get 2 v)
```
**Status:** [ ]

### 103: Knapsack Problem — Dynamic Programming Functional
**Source:** https://rosettacode.org/wiki/Knapsack_problem/0-1#OCaml
**Topic:** 0/1 knapsack solved with memoized recursion
**Difficulty:** Advanced
**Category:** Memoization
**OCaml:**
```ocaml
let knapsack items capacity =
  let n = List.length items in
  let items = Array.of_list items in
  let cache = Hashtbl.create 256 in
  let rec solve i cap =
    if i >= n || cap <= 0 then 0
    else match Hashtbl.find_opt cache (i, cap) with
    | Some v -> v
    | None ->
      let (w, v) = items.(i) in
      let without = solve (i+1) cap in
      let with_item = if w <= cap then v + solve (i+1) (cap - w) else 0 in
      let best = max without with_item in
      Hashtbl.add cache (i, cap) best;
      best
  in solve 0 capacity

let () =
  let items = [(2,3);(3,4);(4,5);(5,6)] in
  Printf.printf "Max value: %d\n" (knapsack items 8)
```
**Status:** [ ]

### 104: Power Set — All Subsets
**Source:** https://v2.ocaml.org/learn/tutorials/99problems.html
**Topic:** Generating all subsets of a list recursively
**Difficulty:** Intermediate
**Category:** Math/recursion
**OCaml:**
```ocaml
let rec powerset = function
  | [] -> [[]]
  | x :: rest ->
    let ps = powerset rest in
    ps @ List.map (fun s -> x :: s) ps

let () =
  let sets = powerset [1; 2; 3] in
  Printf.printf "%d subsets:\n" (List.length sets);
  List.iter (fun s ->
    Printf.printf "{%s}\n" (String.concat "," (List.map string_of_int s))
  ) sets
```
**Status:** [ ]


### 025: List.map — Transform Every Element
**Source:** OCaml Standard Library
**Topic:** Apply a function to each element of a list
**Difficulty:** Beginner
**Category:** stdlib-list
**OCaml:**
```ocaml
let numbers = [1; 2; 3; 4; 5]
let doubled = List.map (fun x -> x * 2) numbers
let () = List.iter (fun x -> Printf.printf "%d " x) doubled
(* Output: 2 4 6 8 10 *)
```
**Status:** [ ]

### 026: List.filter — Select Elements by Predicate
**Source:** OCaml Standard Library
**Topic:** Keep only elements satisfying a condition
**Difficulty:** Beginner
**Category:** stdlib-list
**OCaml:**
```ocaml
let numbers = [1; 2; 3; 4; 5; 6; 7; 8]
let evens = List.filter (fun x -> x mod 2 = 0) numbers
let odds = List.filter (fun x -> x mod 2 <> 0) numbers
let () = Printf.printf "Evens: %s\n"
  (String.concat ", " (List.map string_of_int evens))
let () = Printf.printf "Odds: %s\n"
  (String.concat ", " (List.map string_of_int odds))
```
**Status:** [ ]

### 027: List.fold_left — Accumulate a Result
**Source:** OCaml Standard Library
**Topic:** Reduce a list to a single value from left to right
**Difficulty:** Beginner
**Category:** stdlib-list
**OCaml:**
```ocaml
let numbers = [1; 2; 3; 4; 5]
let sum = List.fold_left ( + ) 0 numbers
let product = List.fold_left ( * ) 1 numbers
let max_val = List.fold_left max min_int numbers
let () = Printf.printf "Sum: %d, Product: %d, Max: %d\n" sum product max_val
```
**Status:** [ ]

### 028: List.sort — Sort with Custom Comparator
**Source:** OCaml Standard Library
**Topic:** Sort a list using a comparison function
**Difficulty:** Beginner
**Category:** stdlib-list
**OCaml:**
```ocaml
let words = ["banana"; "apple"; "cherry"; "date"]
let sorted = List.sort String.compare words
let by_length = List.sort (fun a b -> compare (String.length a) (String.length b)) words
let descending = List.sort (fun a b -> String.compare b a) words
let () = List.iter (fun s -> Printf.printf "%s " s) sorted
```
**Status:** [ ]

### 029: List.partition — Split by Predicate
**Source:** OCaml Standard Library
**Topic:** Divide a list into two based on a predicate
**Difficulty:** Beginner
**Category:** stdlib-list
**OCaml:**
```ocaml
let numbers = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]
let (small, big) = List.partition (fun x -> x <= 5) numbers
let () = Printf.printf "Small: %s\n"
  (String.concat " " (List.map string_of_int small))
let () = Printf.printf "Big: %s\n"
  (String.concat " " (List.map string_of_int big))
```
**Status:** [ ]

### 030: List.flatten — Flatten Nested Lists
**Source:** OCaml Standard Library
**Topic:** Concatenate a list of lists into a single list
**Difficulty:** Beginner
**Category:** stdlib-list
**OCaml:**
```ocaml
let nested = [[1; 2]; [3; 4; 5]; [6]; [7; 8; 9; 10]]
let flat = List.flatten nested
let () = Printf.printf "Flat: %s\n"
  (String.concat " " (List.map string_of_int flat))
(* Also useful: List.concat_map *)
let pairs = List.concat_map (fun x -> [x; x * 10]) [1; 2; 3]
```
**Status:** [ ]

### 031: List.assoc — Association Lists as Simple Maps
**Source:** OCaml Standard Library
**Topic:** Use association lists for key-value lookups
**Difficulty:** Beginner
**Category:** stdlib-list
**OCaml:**
```ocaml
let phonebook = [("Alice", "555-1234"); ("Bob", "555-5678"); ("Carol", "555-9012")]
let bobs_number = List.assoc "Bob" phonebook
let has_dave = List.mem_assoc "Dave" phonebook
let without_bob = List.remove_assoc "Bob" phonebook
let () = Printf.printf "Bob: %s, Dave exists: %b\n" bobs_number has_dave
```
**Status:** [ ]

### 032: List.fold_right — Right-to-Left Accumulation
**Source:** OCaml Standard Library
**Topic:** Fold from the right, preserving structure
**Difficulty:** Intermediate
**Category:** stdlib-list
**OCaml:**
```ocaml
(* fold_right preserves order when building lists *)
let duplicate lst =
  List.fold_right (fun x acc -> x :: x :: acc) lst []

let result = duplicate [1; 2; 3]
let () = List.iter (fun x -> Printf.printf "%d " x) result
(* Output: 1 1 2 2 3 3 *)

(* Compare: fold_left would reverse *)
let rev_dup lst =
  List.fold_left (fun acc x -> x :: x :: acc) [] lst
```
**Status:** [ ]

### 033: Array.init and Array.map — Array Creation and Transform
**Source:** OCaml Standard Library
**Topic:** Create and transform arrays with functions
**Difficulty:** Beginner
**Category:** stdlib-array
**OCaml:**
```ocaml
let squares = Array.init 10 (fun i -> i * i)
let doubled = Array.map (fun x -> x * 2) squares
let () = Array.iter (fun x -> Printf.printf "%d " x) squares
let () = print_newline ()
let () = Array.iter (fun x -> Printf.printf "%d " x) doubled
```
**Status:** [ ]

### 034: Array.fold_left — Reduce an Array
**Source:** OCaml Standard Library
**Topic:** Accumulate over array elements
**Difficulty:** Beginner
**Category:** stdlib-array
**OCaml:**
```ocaml
let values = [| 3.14; 2.71; 1.41; 1.73 |]
let sum = Array.fold_left ( +. ) 0.0 values
let avg = sum /. float_of_int (Array.length values)
let min_v = Array.fold_left min infinity values
let () = Printf.printf "Sum: %.2f, Avg: %.2f, Min: %.2f\n" sum avg min_v
```
**Status:** [ ]

### 035: Array.sort — In-Place Sorting
**Source:** OCaml Standard Library
**Topic:** Sort an array in place with a comparator
**Difficulty:** Beginner
**Category:** stdlib-array
**OCaml:**
```ocaml
let arr = [| 5; 3; 8; 1; 9; 2; 7 |]
let () = Array.sort compare arr
let () = Array.iter (fun x -> Printf.printf "%d " x) arr
(* Output: 1 2 3 5 7 8 9 *)

(* Descending *)
let desc = Array.copy arr
let () = Array.sort (fun a b -> compare b a) desc
```
**Status:** [ ]

### 036: Array.blit — Copy Subarray
**Source:** OCaml Standard Library
**Topic:** Copy a portion of one array into another
**Difficulty:** Intermediate
**Category:** stdlib-array
**OCaml:**
```ocaml
let src = [| 10; 20; 30; 40; 50 |]
let dst = Array.make 8 0
let () = Array.blit src 1 dst 2 3
(* dst is now [| 0; 0; 20; 30; 40; 0; 0; 0 |] *)
let () = Array.iter (fun x -> Printf.printf "%d " x) dst
```
**Status:** [ ]

### 037: Array.make and Array.make_matrix — Multi-dimensional Arrays
**Source:** OCaml Standard Library
**Topic:** Create 1D and 2D arrays
**Difficulty:** Beginner
**Category:** stdlib-array
**OCaml:**
```ocaml
let zeros = Array.make 5 0
let matrix = Array.make_matrix 3 4 0.0
let () = matrix.(1).(2) <- 42.0
let () =
  Array.iter (fun row ->
    Array.iter (fun x -> Printf.printf "%.0f " x) row;
    print_newline ()
  ) matrix
```
**Status:** [ ]

### 038: String.split_on_char — Tokenize a String
**Source:** OCaml Standard Library
**Topic:** Split a string into parts by a delimiter character
**Difficulty:** Beginner
**Category:** stdlib-string
**OCaml:**
```ocaml
let csv_line = "Alice,30,Engineer,Amsterdam"
let fields = String.split_on_char ',' csv_line
let () = List.iteri (fun i f -> Printf.printf "Field %d: %s\n" i f) fields

let words = String.split_on_char ' ' "  hello   world  "
let nonempty = List.filter (fun s -> s <> "") words
```
**Status:** [ ]

### 039: String.sub and String.concat — Substring and Join
**Source:** OCaml Standard Library
**Topic:** Extract substrings and join strings
**Difficulty:** Beginner
**Category:** stdlib-string
**OCaml:**
```ocaml
let s = "Hello, World!"
let hello = String.sub s 0 5
let world = String.sub s 7 5
let () = Printf.printf "'%s' and '%s'\n" hello world

let parts = ["one"; "two"; "three"]
let joined = String.concat " | " parts
let () = Printf.printf "Joined: %s\n" joined
```
**Status:** [ ]

### 040: String — Trim, Uppercase, Contains
**Source:** OCaml Standard Library
**Topic:** Common string operations
**Difficulty:** Beginner
**Category:** stdlib-string
**OCaml:**
```ocaml
let s = "  Hello, World!  "
let trimmed = String.trim s
let upper = String.uppercase_ascii trimmed
let lower = String.lowercase_ascii trimmed
let has_world = String.length s > 0 &&
  let rec find i =
    if i > String.length s - 5 then false
    else if String.sub s i 5 = "World" then true
    else find (i + 1)
  in find 0
let () = Printf.printf "Trimmed: '%s'\nUpper: '%s'\n" trimmed upper
```
**Status:** [ ]

### 041: String.map and String.init — Character-level Operations
**Source:** OCaml Standard Library
**Topic:** Transform strings character by character
**Difficulty:** Beginner
**Category:** stdlib-string
**OCaml:**
```ocaml
let rot13 c =
  if c >= 'a' && c <= 'z' then Char.chr ((Char.code c - Char.code 'a' + 13) mod 26 + Char.code 'a')
  else if c >= 'A' && c <= 'Z' then Char.chr ((Char.code c - Char.code 'A' + 13) mod 26 + Char.code 'A')
  else c

let encoded = String.map rot13 "Hello World"
let decoded = String.map rot13 encoded
let () = Printf.printf "%s -> %s\n" encoded decoded

let alphabet = String.init 26 (fun i -> Char.chr (i + Char.code 'a'))
```
**Status:** [ ]

### 042: Option.map and Option.bind — Safe Value Transformation
**Source:** OCaml Standard Library
**Topic:** Chain operations on optional values
**Difficulty:** Intermediate
**Category:** stdlib-option
**OCaml:**
```ocaml
let parse_int s = match int_of_string_opt s with Some n -> Some n | None -> None
let safe_div x y = if y = 0 then None else Some (x / y)

let result =
  parse_int "42"
  |> Option.map (fun x -> x * 2)
  |> Option.bind (fun x -> safe_div x 7)

let () = match result with
  | Some v -> Printf.printf "Result: %d\n" v
  | None -> Printf.printf "No result\n"
```
**Status:** [ ]

### 043: Option.value and Option.is_some — Default Values
**Source:** OCaml Standard Library
**Topic:** Extract values from options with defaults
**Difficulty:** Beginner
**Category:** stdlib-option
**OCaml:**
```ocaml
let config_port = None
let config_host = Some "localhost"

let port = Option.value ~default:8080 config_port
let host = Option.value ~default:"0.0.0.0" config_host

let () = Printf.printf "Server: %s:%d\n" host port
let () = Printf.printf "Port set: %b, Host set: %b\n"
  (Option.is_some config_port) (Option.is_some config_host)
```
**Status:** [ ]

### 044: Option.iter and Option.fold — Side Effects and Folding
**Source:** OCaml Standard Library
**Topic:** Perform actions on optional values
**Difficulty:** Intermediate
**Category:** stdlib-option
**OCaml:**
```ocaml
let maybe_name = Some "Alice"
let no_name : string option = None

let () = Option.iter (fun name -> Printf.printf "Hello, %s!\n" name) maybe_name
let () = Option.iter (fun name -> Printf.printf "Hello, %s!\n" name) no_name

let greeting = Option.fold ~none:"Hello, stranger!" ~some:(fun n -> "Hello, " ^ n ^ "!") maybe_name
let () = print_endline greeting
```
**Status:** [ ]

### 045: Result.bind and Result.map — Error Handling Pipeline
**Source:** OCaml Standard Library
**Topic:** Chain computations that may fail
**Difficulty:** Intermediate
**Category:** stdlib-result
**OCaml:**
```ocaml
let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error ("Not a number: " ^ s)

let check_positive n =
  if n > 0 then Ok n else Error "Must be positive"

let check_range n =
  if n <= 100 then Ok n else Error "Must be <= 100"

let validate s =
  parse_int s
  |> Result.bind check_positive
  |> Result.bind check_range
  |> Result.map (fun n -> n * 2)

let () = match validate "42" with
  | Ok v -> Printf.printf "Valid: %d\n" v
  | Error e -> Printf.printf "Error: %s\n" e
```
**Status:** [ ]

### 046: Result.map_error — Transform Error Types
**Source:** OCaml Standard Library
**Topic:** Map over the error side of a Result
**Difficulty:** Intermediate
**Category:** stdlib-result
**OCaml:**
```ocaml
type app_error = ParseError of string | RangeError of string

let parse s = match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (ParseError ("Invalid: " ^ s))

let check n =
  if n >= 0 && n <= 100 then Ok n
  else Error (RangeError (Printf.sprintf "%d out of range" n))

let string_of_error = function
  | ParseError s -> "Parse: " ^ s
  | RangeError s -> "Range: " ^ s

let () =
  let result = parse "50" |> Result.bind check in
  match result with
  | Ok v -> Printf.printf "Ok: %d\n" v
  | Error e -> Printf.printf "Error: %s\n" (string_of_error e)
```
**Status:** [ ]

### 047: Hashtbl — Create, Add, Find, Iterate
**Source:** OCaml Standard Library
**Topic:** Mutable hash table operations
**Difficulty:** Intermediate
**Category:** stdlib-hashtbl
**OCaml:**
```ocaml
let tbl = Hashtbl.create 16
let () =
  Hashtbl.add tbl "apple" 3;
  Hashtbl.add tbl "banana" 5;
  Hashtbl.add tbl "cherry" 2

let count = Hashtbl.find tbl "banana"
let () = Printf.printf "Bananas: %d\n" count

let () = Hashtbl.iter (fun k v -> Printf.printf "%s: %d\n" k v) tbl

let total = Hashtbl.fold (fun _k v acc -> acc + v) tbl 0
let () = Printf.printf "Total: %d\n" total
```
**Status:** [ ]

### 048: Hashtbl — Word Frequency Counter
**Source:** OCaml Standard Library
**Topic:** Use Hashtbl to count word occurrences
**Difficulty:** Intermediate
**Category:** stdlib-hashtbl
**OCaml:**
```ocaml
let count_words text =
  let tbl = Hashtbl.create 32 in
  let words = String.split_on_char ' ' text in
  List.iter (fun w ->
    let w = String.lowercase_ascii w in
    let n = try Hashtbl.find tbl w with Not_found -> 0 in
    Hashtbl.replace tbl w (n + 1)
  ) words;
  tbl

let freq = count_words "the cat sat on the mat the cat"
let () = Hashtbl.iter (fun w n -> Printf.printf "%s: %d\n" w n) freq
```
**Status:** [ ]

### 049: Hashtbl.to_seq — Convert to Sequence
**Source:** OCaml Standard Library
**Topic:** Bridge between mutable hash tables and functional sequences
**Difficulty:** Intermediate
**Category:** stdlib-hashtbl
**OCaml:**
```ocaml
let tbl = Hashtbl.create 8
let () = List.iter (fun (k,v) -> Hashtbl.add tbl k v)
  [("x", 1); ("y", 2); ("z", 3)]

let sorted_pairs =
  Hashtbl.to_seq tbl
  |> List.of_seq
  |> List.sort (fun (a,_) (b,_) -> String.compare a b)

let () = List.iter (fun (k,v) -> Printf.printf "%s=%d " k v) sorted_pairs
```
**Status:** [ ]

### 050: Map.Make — Immutable Sorted Map
**Source:** OCaml Standard Library
**Topic:** Create and use a functional map module
**Difficulty:** Intermediate
**Category:** stdlib-map
**OCaml:**
```ocaml
module StringMap = Map.Make(String)

let m = StringMap.empty
  |> StringMap.add "alice" 90
  |> StringMap.add "bob" 85
  |> StringMap.add "carol" 92

let bob_score = StringMap.find "bob" m
let has_dave = StringMap.mem "dave" m
let () = Printf.printf "Bob: %d, Dave exists: %b\n" bob_score has_dave

let () = StringMap.iter (fun k v -> Printf.printf "%s: %d\n" k v) m
```
**Status:** [ ]

### 051: Map.Make — Filter, Map, Fold
**Source:** OCaml Standard Library
**Topic:** Functional transformations on maps
**Difficulty:** Intermediate
**Category:** stdlib-map
**OCaml:**
```ocaml
module IntMap = Map.Make(Int)

let scores = List.fold_left (fun m (k,v) -> IntMap.add k v m) IntMap.empty
  [(1, 85); (2, 92); (3, 78); (4, 95); (5, 60)]

let high_scores = IntMap.filter (fun _k v -> v >= 80) scores
let curved = IntMap.map (fun v -> min 100 (v + 5)) scores
let total = IntMap.fold (fun _k v acc -> acc + v) scores 0
let avg = total / IntMap.cardinal scores

let () = Printf.printf "High scorers: %d, Average: %d\n"
  (IntMap.cardinal high_scores) avg
```
**Status:** [ ]

### 052: Map.Make — Bindings and Merge
**Source:** OCaml Standard Library
**Topic:** Extract bindings and merge two maps
**Difficulty:** Intermediate
**Category:** stdlib-map
**OCaml:**
```ocaml
module SMap = Map.Make(String)

let m1 = SMap.of_list [("a", 1); ("b", 2); ("c", 3)]
let m2 = SMap.of_list [("b", 20); ("c", 30); ("d", 40)]

let merged = SMap.union (fun _k v1 v2 -> Some (v1 + v2)) m1 m2

let pairs = SMap.bindings merged
let () = List.iter (fun (k,v) -> Printf.printf "%s: %d\n" k v) pairs
```
**Status:** [ ]

### 053: Set.Make — Immutable Sorted Set
**Source:** OCaml Standard Library
**Topic:** Create and use functional sets
**Difficulty:** Intermediate
**Category:** stdlib-set
**OCaml:**
```ocaml
module IntSet = Set.Make(Int)

let s1 = IntSet.of_list [1; 3; 5; 7; 9]
let s2 = IntSet.of_list [2; 3; 5; 7; 11]

let union = IntSet.union s1 s2
let inter = IntSet.inter s1 s2
let diff = IntSet.diff s1 s2

let print_set s =
  IntSet.elements s |> List.map string_of_int |> String.concat ", "
  |> Printf.printf "{%s}\n"

let () = print_set union; print_set inter; print_set diff
```
**Status:** [ ]

### 054: Set.Make — Set Operations for Data Processing
**Source:** OCaml Standard Library
**Topic:** Use sets for deduplication and membership testing
**Difficulty:** Intermediate
**Category:** stdlib-set
**OCaml:**
```ocaml
module StringSet = Set.Make(String)

let words = ["the"; "cat"; "sat"; "on"; "the"; "mat"; "the"; "cat"]
let unique = StringSet.of_list words
let () = Printf.printf "Unique words: %d\n" (StringSet.cardinal unique)

let stopwords = StringSet.of_list ["the"; "on"; "a"; "an"]
let content_words = StringSet.diff unique stopwords
let () = StringSet.iter (fun w -> Printf.printf "%s " w) content_words
```
**Status:** [ ]

### 055: Seq.unfold — Generate Sequences Lazily
**Source:** OCaml Standard Library
**Topic:** Create lazy sequences with unfold
**Difficulty:** Intermediate
**Category:** stdlib-seq
**OCaml:**
```ocaml
let naturals = Seq.unfold (fun n -> Some (n, n + 1)) 0

let fibs = Seq.unfold (fun (a, b) -> Some (a, (b, a + b))) (0, 1)

let first_10_fibs = fibs |> Seq.take 10 |> List.of_seq
let () = List.iter (fun x -> Printf.printf "%d " x) first_10_fibs

let powers_of_2 = Seq.unfold (fun n -> if n > 1024 then None else Some (n, n * 2)) 1
let () = print_newline ();
  Seq.iter (fun x -> Printf.printf "%d " x) powers_of_2
```
**Status:** [ ]

### 056: Seq.map, Seq.filter — Lazy Transformations
**Source:** OCaml Standard Library
**Topic:** Transform and filter sequences lazily
**Difficulty:** Intermediate
**Category:** stdlib-seq
**OCaml:**
```ocaml
let naturals = Seq.unfold (fun n -> Some (n, n + 1)) 1

let even_squares =
  naturals
  |> Seq.map (fun n -> n * n)
  |> Seq.filter (fun n -> n mod 2 = 0)
  |> Seq.take 8
  |> List.of_seq

let () = List.iter (fun x -> Printf.printf "%d " x) even_squares
(* Output: 4 16 36 64 100 144 196 256 *)
```
**Status:** [ ]

### 057: Seq — Zip and Iterate
**Source:** OCaml Standard Library
**Topic:** Combine sequences and create repeated applications
**Difficulty:** Intermediate
**Category:** stdlib-seq
**OCaml:**
```ocaml
let letters = List.to_seq ['a'; 'b'; 'c'; 'd']
let numbers = List.to_seq [1; 2; 3; 4]
let pairs = Seq.zip letters numbers |> List.of_seq
let () = List.iter (fun (c, n) -> Printf.printf "(%c, %d) " c n) pairs

(* Seq.iterate: repeated function application *)
let collatz n = if n mod 2 = 0 then n / 2 else 3 * n + 1
let seq = Seq.iterate collatz 27 |> Seq.take 20 |> List.of_seq
let () = print_newline ();
  List.iter (fun x -> Printf.printf "%d " x) seq
```
**Status:** [ ]

### 058: Printf and Format — Formatted Output
**Source:** OCaml Standard Library
**Topic:** Type-safe formatted printing
**Difficulty:** Beginner
**Category:** stdlib-printf
**OCaml:**
```ocaml
let () =
  Printf.printf "Integer: %d\n" 42;
  Printf.printf "Float: %.2f\n" 3.14159;
  Printf.printf "String: %s\n" "hello";
  Printf.printf "Char: %c\n" 'A';
  Printf.printf "Bool: %b\n" true;
  Printf.printf "Hex: 0x%x, Oct: 0o%o\n" 255 255;
  Printf.printf "Padded: [%10d] [%-10d]\n" 42 42;
  Printf.printf "Zero-padded: [%06d]\n" 42

let msg = Printf.sprintf "(%d, %d)" 10 20
let () = print_endline msg
```
**Status:** [ ]

### 059: Printf.sprintf — Build Strings with Formatting
**Source:** OCaml Standard Library
**Topic:** Create formatted strings without printing
**Difficulty:** Beginner
**Category:** stdlib-printf
**OCaml:**
```ocaml
let format_record name age score =
  Printf.sprintf "%-15s | %3d | %6.2f" name age score

let header = Printf.sprintf "%-15s | %3s | %6s" "Name" "Age" "Score"
let sep = String.make (String.length header) '-'

let () =
  print_endline header;
  print_endline sep;
  print_endline (format_record "Alice" 30 95.5);
  print_endline (format_record "Bob" 25 87.3);
  print_endline (format_record "Carol" 28 92.1)
```
**Status:** [ ]

### 060: Buffer — Efficient String Building
**Source:** OCaml Standard Library
**Topic:** Build strings incrementally with Buffer
**Difficulty:** Intermediate
**Category:** stdlib-buffer
**OCaml:**
```ocaml
let build_csv rows =
  let buf = Buffer.create 256 in
  List.iter (fun row ->
    Buffer.add_string buf (String.concat "," row);
    Buffer.add_char buf '\n'
  ) rows;
  Buffer.contents buf

let data = [
  ["name"; "age"; "city"];
  ["Alice"; "30"; "Amsterdam"];
  ["Bob"; "25"; "Berlin"]
]
let csv = build_csv data
let () = print_string csv
```
**Status:** [ ]

### 025: Leap Year
**Source:** https://exercism.org/tracks/ocaml/exercises/leap
**Topic:** Boolean logic with divisibility rules
**Difficulty:** Beginner
**Category:** Pattern Matching & Logic
**OCaml:**
```ocaml
let leap_year year =
  (year mod 400 = 0) ||
    (year mod 4 = 0 && year mod 100 <> 0)
```
**Status:** [ ]

### 026: Reverse String
**Source:** https://exercism.org/tracks/ocaml/exercises/reverse-string
**Topic:** String manipulation using index arithmetic
**Difficulty:** Beginner
**Category:** Strings
**OCaml:**
```ocaml
let reverse_string str =
  let len = String.length str in
  String.init len (fun i -> str.[len - 1 - i])
```
**Status:** [ ]

### 027: Two-fer
**Source:** https://exercism.org/tracks/ocaml/exercises/two-fer
**Topic:** Option type for default values
**Difficulty:** Beginner
**Category:** Option Type
**OCaml:**
```ocaml
let two_fer name =
  match name with
  | Some x -> "One for " ^ x ^ ", one for me."
  | None -> "One for you, one for me."
```
**Status:** [ ]

### 028: Raindrops
**Source:** https://exercism.org/tracks/ocaml/exercises/raindrops
**Topic:** Pattern matching on divisibility — FizzBuzz variant
**Difficulty:** Beginner
**Category:** Pattern Matching
**OCaml:**
```ocaml
let raindrop = function
  | n when n mod 105 = 0 -> "PlingPlangPlong"
  | n when n mod 35 = 0 -> "PlangPlong"
  | n when n mod 21 = 0 -> "PlingPlong"
  | n when n mod 15 = 0 -> "PlingPlang"
  | n when n mod 7 = 0 -> "Plong"
  | n when n mod 5 = 0 -> "Plang"
  | n when n mod 3 = 0 -> "Pling"
  | n -> string_of_int n
```
**Status:** [ ]

### 029: Darts
**Source:** https://exercism.org/tracks/ocaml/exercises/darts
**Topic:** Floating-point distance calculation with conditional scoring
**Difficulty:** Beginner
**Category:** Math & Conditionals
**OCaml:**
```ocaml
let score (x: float) (y: float): int =
  let distance = sqrt (x *. x +. y *. y) in
  if distance <= 1.0 then 10
  else if distance <= 5.0 then 5
  else if distance <= 10.0 then 1
  else 0
```
**Status:** [ ]

### 030: RNA Transcription
**Source:** https://exercism.org/tracks/ocaml/exercises/rna-transcription
**Topic:** Polymorphic variant mapping with List.map
**Difficulty:** Beginner
**Category:** Polymorphic Variants
**OCaml:**
```ocaml
type dna = [ `A | `C | `G | `T ]
type rna = [ `A | `C | `G | `U ]

let to_rna = List.map (function
    | `A -> `U
    | `C -> `G
    | `G -> `C
    | `T -> `A)
```
**Status:** [ ]

### 031: Eliud's Eggs (Bit Counting)
**Source:** https://exercism.org/tracks/ocaml/exercises/eliuds-eggs
**Topic:** Bit manipulation — counting set bits
**Difficulty:** Beginner
**Category:** Bit Operations
**OCaml:**
```ocaml
let egg_count number =
  let rec do_count number acc =
    if number = 0 then acc
    else do_count (number lsr 1) (acc + (number land 1))
  in
  do_count number 0
```
**Status:** [ ]

### 032: Sum of Multiples
**Source:** https://exercism.org/tracks/ocaml/exercises/sum-of-multiples
**Topic:** List.init, filter, and fold for numeric aggregation
**Difficulty:** Beginner
**Category:** Higher-Order Functions
**OCaml:**
```ocaml
let sum factors limit =
  List.init (limit - 1) ((+) 1)
  |> List.filter (fun i -> List.exists (fun f -> f <> 0 && i mod f = 0) factors)
  |> List.fold_left ( + ) 0
```
**Status:** [ ]

### 033: Triangle Classification
**Source:** https://exercism.org/tracks/ocaml/exercises/triangle
**Topic:** Boolean predicates with structural classification
**Difficulty:** Beginner
**Category:** Pattern Matching & Logic
**OCaml:**
```ocaml
let is_triangle a b c =
  a > 0 && b > 0 && c > 0 &&
    (a <= b + c && b <= a + c && c <= a + b)

let is_equilateral a b c =
  is_triangle a b c && a = b && b = c

let is_isosceles a b c =
  is_triangle a b c && (a = b || b = c || a = c)

let is_scalene a b c =
  is_triangle a b c && (a <> b && b <> c && a <> c)
```
**Status:** [ ]

### 034: Difference of Squares
**Source:** https://exercism.org/tracks/ocaml/exercises/difference-of-squares
**Topic:** List generation with fold — math formulas
**Difficulty:** Beginner
**Category:** Higher-Order Functions
**OCaml:**
```ocaml
let rec range a b =
  if a > b then [] else a :: (range (a + 1) b)

let square_of_sum n =
  let sum = List.fold_left (+) 0 (range 1 n) in
  sum * sum

let sum_of_squares n =
  let squares = List.map (fun m -> m * m) (range 1 n) in
  List.fold_left (+) 0 squares

let difference_of_squares n =
  square_of_sum n - sum_of_squares n
```
**Status:** [ ]

### 035: Space Age
**Source:** https://exercism.org/tracks/ocaml/exercises/space-age
**Topic:** Variant types for planets with float arithmetic
**Difficulty:** Beginner
**Category:** Variants & Float Arithmetic
**OCaml:**
```ocaml
type planet = Mercury | Venus | Earth | Mars
            | Jupiter | Saturn | Neptune | Uranus

let earth_years seconds = seconds /. 31557600.0

let rel_years = function
  | Mercury -> 0.2408467
  | Venus   -> 0.61519726
  | Earth   -> 1.0
  | Mars    -> 1.8808158
  | Jupiter -> 11.862615
  | Saturn  -> 29.447498
  | Uranus  -> 84.016846
  | Neptune -> 164.79132

let age_on planet seconds =
  let seconds' = Float.of_int seconds in
  earth_years seconds' /. rel_years planet
```
**Status:** [ ]

### 036: Isogram
**Source:** https://exercism.org/tracks/ocaml/exercises/isogram
**Topic:** Character uniqueness check via string traversal
**Difficulty:** Beginner
**Category:** Strings & Recursion
**OCaml:**
```ocaml
let tail word = String.sub word 1 (String.length word - 1)

let rec is_unique ch word =
  match (ch, word) with
  | (_, "") -> true
  | (c, w) when c < 'a' || c > 'z' -> is_unique (String.get w 0) (tail w)
  | (c, w) when String.contains w c -> false
  | (_, w) -> is_unique (String.get w 0) (tail w)

let is_isogram word =
  let lower = String.lowercase_ascii word in
  match lower with
  | "" -> true
  | w -> is_unique (String.get w 0) (tail w)
```
**Status:** [ ]

### 037: Square Root (Newton's Method)
**Source:** https://exercism.org/tracks/ocaml/exercises/square-root
**Topic:** Iterative approximation with recursion — Newton's method
**Difficulty:** Beginner
**Category:** Recursion & Math
**OCaml:**
```ocaml
let square_root n =
  let radicand = float_of_int n in
  let rec aux guess =
    let next = 0.5 *. (guess +. radicand /. guess) in
    if abs_float (next -. guess) < 0.0001 then int_of_float next
    else aux next
  in
  aux (radicand /. 2.0)
```
**Status:** [ ]

### 038: Acronym
**Source:** https://exercism.org/tracks/ocaml/exercises/acronym
**Topic:** String splitting, filtering, and mapping with Base library
**Difficulty:** Beginner
**Category:** Strings & Higher-Order Functions
**OCaml:**
```ocaml
let delimiters = [' '; '-'; '_']
let is_relevant c =
  (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
  List.mem c delimiters

let acronym input =
  let filtered = String.init (String.length input)
    (fun i -> let c = input.[i] in if is_relevant c then c else ' ') in
  let words = String.split_on_char ' ' filtered
    |> List.filter (fun w -> String.length w > 0) in
  let initials = List.map (fun w -> Char.uppercase_ascii w.[0]) words in
  String.init (List.length initials) (List.nth initials)
```
**Status:** [ ]

### 039: ETL (Extract-Transform-Load)
**Source:** https://exercism.org/tracks/ocaml/exercises/etl
**Topic:** Data transformation — inverting key-value pairs
**Difficulty:** Beginner
**Category:** Higher-Order Functions & Data Transformation
**OCaml:**
```ocaml
let transform data =
  let assign points letter = (Char.lowercase_ascii letter, points) in
  let gather (points, letters) = List.map (assign points) letters in
  let compare (a, _) (b, _) = Char.compare a b in
  List.sort compare (List.concat_map gather data)
```
**Status:** [ ]

### 040: Hexadecimal Parser
**Source:** https://exercism.org/tracks/ocaml/exercises/hexadecimal
**Topic:** Character-to-integer conversion with pattern matching
**Difficulty:** Beginner
**Category:** Pattern Matching & Parsing
**OCaml:**
```ocaml
let digit_to_int = function
  | '0'..'9' as c -> Some (Char.code c - 48)
  | 'a'..'f' as c -> Some (Char.code c - 87)
  | 'A'..'F' as c -> Some (Char.code c - 55)
  | _ -> None

let to_int hex_str =
  let chars = List.init (String.length hex_str) (String.get hex_str) in
  let rec go acc = function
    | [] -> acc
    | c :: cs -> match digit_to_int c with
      | Some n -> go (acc * 16 + n) cs
      | None -> 0
  in
  go 0 chars
```
**Status:** [ ]

### 041: Perfect Numbers
**Source:** https://exercism.org/tracks/ocaml/exercises/perfect-numbers
**Topic:** Number classification by aliquot sum — Result type
**Difficulty:** Beginner
**Category:** Math & Result Type
**OCaml:**
```ocaml
let classify n =
  let aliquot = function
    | 1 -> 0
    | n when n > 1 ->
      let rec sum_factors acc factor =
        if factor > n / 2 then acc
        else if n mod factor = 0 then sum_factors (acc + factor) (factor + 1)
        else sum_factors acc (factor + 1)
      in sum_factors 0 1
    | _ -> 0
  in
  if n < 1 then Error "Classification is only possible for positive integers."
  else
    let s = aliquot n in
    if s = n then Ok "perfect"
    else if s > n then Ok "abundant"
    else Ok "deficient"
```
**Status:** [ ]

### 042: Bob — Response Patterns
**Source:** https://exercism.org/tracks/ocaml/exercises/bob
**Topic:** String classification with boolean predicates and pattern matching
**Difficulty:** Beginner
**Category:** Strings & Pattern Matching
**OCaml:**
```ocaml
let is_empty s = String.trim s = ""

let is_shouting s =
  let has_alpha = ref false in
  String.iter (fun c ->
    if c >= 'a' && c <= 'z' then has_alpha := true) s;
  not !has_alpha &&
  String.iter (fun c ->
    if c >= 'A' && c <= 'Z' then has_alpha := true) s;
  !has_alpha

let is_question s =
  String.length s > 0 && s.[String.length s - 1] = '?'

let response_for s =
  let s = String.trim s in
  if is_empty s then "Fine. Be that way!"
  else if is_shouting s && is_question s then "Calm down, I know what I'm doing!"
  else if is_shouting s then "Whoa, chill out!"
  else if is_question s then "Sure."
  else "Whatever."
```
**Status:** [ ]

### 043: Allergies — Bit Flags
**Source:** https://exercism.org/tracks/ocaml/exercises/allergies
**Topic:** Bit masking with variant types — flag-based logic
**Difficulty:** Intermediate
**Category:** Bit Operations & Variants
**OCaml:**
```ocaml
type allergen =
  | Eggs | Peanuts | Shellfish | Strawberries
  | Tomatoes | Chocolate | Pollen | Cats

let allergy_score = function
  | Eggs -> 1 | Peanuts -> 2 | Shellfish -> 4 | Strawberries -> 8
  | Tomatoes -> 16 | Chocolate -> 32 | Pollen -> 64 | Cats -> 128

let allergic_to score allergen =
  score land allergy_score allergen > 0

let allergies score =
  List.filter (allergic_to score)
    [Eggs; Peanuts; Shellfish; Strawberries; Tomatoes; Chocolate; Pollen; Cats]
```
**Status:** [ ]

### 044: Anagram Detection
**Source:** https://exercism.org/tracks/ocaml/exercises/anagram
**Topic:** Sorting characters to detect anagrams
**Difficulty:** Intermediate
**Category:** Strings & Sorting
**OCaml:**
```ocaml
let to_sorted_list s =
  let chars = List.init (String.length s) (String.get s) in
  List.sort Char.compare chars

let anagrams target candidates =
  let target_lc = String.lowercase_ascii target in
  let target_sorted = to_sorted_list target_lc in
  List.filter (fun c ->
    let lc = String.lowercase_ascii c in
    to_sorted_list lc = target_sorted && lc <> target_lc
  ) candidates
```
**Status:** [ ]

### 045: Pangram Detection
**Source:** https://exercism.org/tracks/ocaml/exercises/pangram
**Topic:** Bit set for alphabet coverage check — early termination
**Difficulty:** Intermediate
**Category:** Bit Operations & Strings
**OCaml:**
```ocaml
let is_pangram s =
  let bits = ref 0 in
  let all_letters = (1 lsl 26) - 1 in
  String.iter (fun c ->
    let c = Char.lowercase_ascii c in
    if c >= 'a' && c <= 'z' then
      bits := !bits lor (1 lsl (Char.code c - Char.code 'a'))
  ) s;
  !bits = all_letters
```
**Status:** [ ]

### 046: Luhn Validation
**Source:** https://exercism.org/tracks/ocaml/exercises/luhn
**Topic:** Digit transformation with doubling rule — checksum algorithm
**Difficulty:** Intermediate
**Category:** Strings & Algorithms
**OCaml:**
```ocaml
let valid s =
  let s = String.concat "" (String.split_on_char ' ' s) in
  if String.length s <= 1 then false
  else if not (String.to_seq s |> Seq.for_all (fun c -> c >= '0' && c <= '9')) then false
  else
    let digits = List.init (String.length s) (fun i ->
      Char.code s.[String.length s - 1 - i] - Char.code '0') in
    let doubled = List.mapi (fun i d ->
      if i mod 2 = 1 then let d2 = d * 2 in if d2 >= 10 then d2 - 9 else d2
      else d) digits in
    List.fold_left (+) 0 doubled mod 10 = 0
```
**Status:** [ ]

### 047: Nth Prime
**Source:** https://exercism.org/tracks/ocaml/exercises/nth-prime
**Topic:** Prime generation with trial division
**Difficulty:** Intermediate
**Category:** Math & Recursion
**OCaml:**
```ocaml
let nth_prime n =
  if n <= 0 then Error "there is no zeroth prime"
  else
    let is_prime x =
      let rec aux d = d * d > x || (x mod d <> 0 && aux (d + 1)) in
      aux 2
    in
    let rec find count candidate =
      if count = n then Ok (candidate - 1)
      else if is_prime candidate then find (count + 1) (candidate + 1)
      else find count (candidate + 1)
    in
    find 0 2
```
**Status:** [ ]

### 048: Binary Search
**Source:** https://exercism.org/tracks/ocaml/exercises/binary-search
**Topic:** Divide-and-conquer search on sorted arrays
**Difficulty:** Intermediate
**Category:** Algorithms & Arrays
**OCaml:**
```ocaml
let find xs value =
  let rec go lo hi =
    if lo > hi then Error "value not in array"
    else
      let mid = lo + (hi - lo) / 2 in
      if xs.(mid) < value then go (mid + 1) hi
      else if xs.(mid) > value then go lo (mid - 1)
      else Ok mid
  in
  if Array.length xs = 0 then Error "value not in array"
  else go 0 (Array.length xs - 1)
```
**Status:** [ ]

### 049: Binary Search Tree
**Source:** https://exercism.org/tracks/ocaml/exercises/binary-search-tree
**Topic:** Recursive BST with insert and in-order traversal
**Difficulty:** Intermediate
**Category:** Trees & Recursion
**OCaml:**
```ocaml
type bst = Leaf | Node of bst * int * bst

let empty = Leaf

let value = function
  | Leaf -> Error "empty tree"
  | Node (_, v, _) -> Ok v

let left = function
  | Leaf -> Error "empty tree"
  | Node (l, _, _) -> Ok l

let right = function
  | Leaf -> Error "empty tree"
  | Node (_, _, r) -> Ok r

let rec insert v = function
  | Leaf -> Node (Leaf, v, Leaf)
  | Node (l, v', r) when v <= v' -> Node (insert v l, v', r)
  | Node (l, v', r) -> Node (l, v', insert v r)

let rec to_list = function
  | Leaf -> []
  | Node (l, v, r) -> to_list l @ [v] @ to_list r
```
**Status:** [ ]

### 050: Roman Numerals
**Source:** https://exercism.org/tracks/ocaml/exercises/roman-numerals
**Topic:** Number-to-string conversion with digit decomposition
**Difficulty:** Intermediate
**Category:** Pattern Matching & Strings
**OCaml:**
```ocaml
let to_roman n =
  let build ones halves tens = function
    | 0 -> "" | 1 -> ones | 2 -> ones ^ ones
    | 3 -> ones ^ ones ^ ones | 4 -> ones ^ halves
    | 5 -> halves | 6 -> halves ^ ones
    | 7 -> halves ^ ones ^ ones
    | 8 -> halves ^ ones ^ ones ^ ones
    | 9 -> ones ^ tens | _ -> assert false
  in
  build "M" "" "" (n / 1000 mod 10) ^
  build "C" "D" "M" (n / 100 mod 10) ^
  build "X" "L" "C" (n / 10 mod 10) ^
  build "I" "V" "X" (n mod 10)
```
**Status:** [ ]

### 051: Run-Length Encoding
**Source:** https://exercism.org/tracks/ocaml/exercises/run-length-encoding
**Topic:** Compression and decompression with character grouping
**Difficulty:** Intermediate
**Category:** Strings & Parsing
**OCaml:**
```ocaml
let encode s =
  let len = String.length s in
  if len = 0 then "" else
  let buf = Buffer.create len in
  let flush ch count =
    if count > 1 then Buffer.add_string buf (string_of_int count);
    Buffer.add_char buf ch in
  let rec go i ch count =
    if i >= len then flush ch count
    else if s.[i] = ch then go (i + 1) ch (count + 1)
    else (flush ch count; go (i + 1) s.[i] 1)
  in
  go 1 s.[0] 1;
  Buffer.contents buf

let decode s =
  let len = String.length s in
  let buf = Buffer.create len in
  let rec go i num =
    if i >= len then ()
    else if s.[i] >= '0' && s.[i] <= '9' then
      go (i + 1) (num * 10 + Char.code s.[i] - 48)
    else begin
      let count = if num = 0 then 1 else num in
      for _ = 1 to count do Buffer.add_char buf s.[i] done;
      go (i + 1) 0
    end
  in
  go 0 0;
  Buffer.contents buf
```
**Status:** [ ]

### 052: Matching Brackets
**Source:** https://exercism.org/tracks/ocaml/exercises/matching-brackets
**Topic:** Stack-based bracket matching with fold
**Difficulty:** Intermediate
**Category:** Stacks & Strings
**OCaml:**
```ocaml
let are_balanced s =
  let rec check stack i =
    if i >= String.length s then stack = []
    else match s.[i], stack with
    | ('(' | '[' | '{') as c, _ -> check (c :: stack) (i + 1)
    | ')', '(' :: rest -> check rest (i + 1)
    | ']', '[' :: rest -> check rest (i + 1)
    | '}', '{' :: rest -> check rest (i + 1)
    | (')'|']'|'}'), _ -> false
    | _, _ -> check stack (i + 1)
  in
  check [] 0
```
**Status:** [ ]

### 053: Nucleotide Count
**Source:** https://exercism.org/tracks/ocaml/exercises/nucleotide-count
**Topic:** Character frequency counting with maps and error handling
**Difficulty:** Intermediate
**Category:** Maps & Error Handling
**OCaml:**
```ocaml
let is_nucleotide = function
  | 'A' | 'C' | 'G' | 'T' -> true
  | _ -> false

let count_nucleotide s c =
  if not (is_nucleotide c) then Error c
  else
    let count = ref 0 in
    let error = ref None in
    String.iter (fun c' ->
      if not (is_nucleotide c') then error := Some c'
      else if c = c' then incr count) s;
    match !error with
    | Some e -> Error e
    | None -> Ok !count
```
**Status:** [ ]

### 054: ISBN Verifier
**Source:** https://exercism.org/tracks/ocaml/exercises/isbn-verifier
**Topic:** Checksum validation with weighted digit processing
**Difficulty:** Intermediate
**Category:** Strings & Validation
**OCaml:**
```ocaml
let is_valid s =
  let chars = List.init (String.length s) (String.get s) in
  let rec aux chars acc count =
    match chars with
    | [] -> count = 10 && acc mod 11 = 0
    | '-' :: rest | ' ' :: rest -> aux rest acc count
    | 'X' :: rest when count = 9 -> aux rest (acc + 10 * (count + 1)) (count + 1)
    | c :: rest when c >= '0' && c <= '9' ->
      let value = Char.code c - Char.code '0' in
      aux rest (acc + value * (count + 1)) (count + 1)
    | _ -> false
  in
  aux chars 0 0
```
**Status:** [ ]

### 055: Atbash Cipher
**Source:** https://exercism.org/tracks/ocaml/exercises/atbash-cipher
**Topic:** Character substitution cipher with grouping
**Difficulty:** Intermediate
**Category:** Strings & Encryption
**OCaml:**
```ocaml
let substitute c =
  if c >= 'a' && c <= 'z' then
    Char.chr (Char.code 'z' - (Char.code c - Char.code 'a'))
  else c

let is_encodable = function
  | 'a'..'z' | '0'..'9' -> true | _ -> false

let encode text =
  let lc = String.lowercase_ascii text in
  let filtered = String.to_seq lc |> Seq.filter is_encodable |> String.of_seq in
  let mapped = String.map substitute filtered in
  let len = String.length mapped in
  let buf = Buffer.create (len + len / 5) in
  String.iteri (fun i c ->
    if i > 0 && i mod 5 = 0 then Buffer.add_char buf ' ';
    Buffer.add_char buf c) mapped;
  Buffer.contents buf

let decode text =
  let lc = String.lowercase_ascii text in
  let filtered = String.to_seq lc |> Seq.filter is_encodable |> String.of_seq in
  String.map substitute filtered
```
**Status:** [ ]

### 056: All Your Base
**Source:** https://exercism.org/tracks/ocaml/exercises/all-your-base
**Topic:** Base conversion between arbitrary number systems
**Difficulty:** Intermediate
**Category:** Math & Error Handling
**OCaml:**
```ocaml
let convert_bases ~from ~digits ~target =
  if from <= 1 || target <= 1 then None
  else
    (* Convert to decimal *)
    let to_decimal =
      List.fold_left (fun acc d ->
        match acc with
        | None -> None
        | Some a -> if d < 0 || d >= from then None else Some (a * from + d)
      ) (Some 0) digits
    in
    match to_decimal with
    | None -> None
    | Some 0 -> Some [0]
    | Some n ->
      let rec to_digits acc = function
        | 0 -> acc
        | n -> to_digits (n mod target :: acc) (n / target)
      in
      Some (to_digits [] n)
```
**Status:** [ ]

### 057: Grade School
**Source:** https://exercism.org/tracks/ocaml/exercises/grade-school
**Topic:** Multi-map with sorted retrieval — school roster
**Difficulty:** Intermediate
**Category:** Maps & Sorting
**OCaml:**
```ocaml
module IntMap = Map.Make(Int)

type school = string list IntMap.t

let empty_school = IntMap.empty

let add name grade school =
  let students = try IntMap.find grade school with Not_found -> [] in
  IntMap.add grade (name :: students) school

let grade g school =
  try IntMap.find g school with Not_found -> []

let roster school =
  IntMap.bindings school
  |> List.sort (fun (g1, _) (g2, _) -> compare g1 g2)
  |> List.concat_map (fun (_, names) -> List.sort String.compare names)
```
**Status:** [ ]

### 058: Phone Number
**Source:** https://exercism.org/tracks/ocaml/exercises/phone-number
**Topic:** String validation and extraction with multiple error cases
**Difficulty:** Intermediate
**Category:** Strings & Validation
**OCaml:**
```ocaml
let number s =
  let has_alpha = String.to_seq s |> Seq.exists (fun c ->
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) in
  if has_alpha then Error "letters not permitted"
  else
    let has_punct = String.to_seq s |> Seq.exists (fun c -> c = '@' || c = '!') in
    if has_punct then Error "punctuations not permitted"
    else
      let digits = String.to_seq s |> Seq.filter (fun c -> c >= '0' && c <= '9')
        |> String.of_seq in
      let len = String.length digits in
      let check s =
        if s.[0] = '0' then Error "area code cannot start with zero"
        else if s.[0] = '1' then Error "area code cannot start with one"
        else if s.[3] = '0' then Error "exchange code cannot start with zero"
        else if s.[3] = '1' then Error "exchange code cannot start with one"
        else Ok s
      in
      if len < 10 then Error "must not be fewer than 10 digits"
      else if len = 10 then check digits
      else if len = 11 && digits.[0] = '1' then check (String.sub digits 1 10)
      else if len = 11 then Error "11 digits must start with 1"
      else Error "must not be greater than 11 digits"
```
**Status:** [ ]

### 059: Prime Factors
**Source:** https://exercism.org/tracks/ocaml/exercises/prime-factors
**Topic:** Integer factorization by trial division
**Difficulty:** Intermediate
**Category:** Math & Recursion
**OCaml:**
```ocaml
let factors_of n =
  let rec go n factor acc =
    if n <= 1L then List.rev acc
    else if Int64.rem n factor = 0L then
      go (Int64.div n factor) factor (factor :: acc)
    else
      go n (Int64.add factor 1L) acc
  in
  go n 2L []
```
**Status:** [ ]

### 060: Knapsack (Dynamic Programming)
**Source:** https://exercism.org/tracks/ocaml/exercises/knapsack
**Topic:** 0/1 knapsack with bottom-up DP using arrays
**Difficulty:** Intermediate
**Category:** Dynamic Programming
**OCaml:**
```ocaml
type item = { weight : int; value : int }

let maximum_value items capacity =
  let dp = Array.make (capacity + 1) 0 in
  List.iter (fun item ->
    for c = capacity downto item.weight do
      let with_item = dp.(c - item.weight) + item.value in
      if with_item > dp.(c) then dp.(c) <- with_item
    done
  ) items;
  dp.(capacity)
```
**Status:** [ ]

### 061: Change (Coin Change)
**Source:** https://exercism.org/tracks/ocaml/exercises/change
**Topic:** Minimum coins with dynamic programming
**Difficulty:** Intermediate
**Category:** Dynamic Programming
**OCaml:**
```ocaml
let make_change ~target ~coins =
  if target = 0 then Ok []
  else if target < 0 then Error "target can't be negative"
  else
    let cache = Array.make (target + 1) None in
    cache.(0) <- Some [];
    for i = 1 to target do
      List.iter (fun coin ->
        if coin <= i then
          match cache.(i - coin) with
          | None -> ()
          | Some prev ->
            let candidate = coin :: prev in
            match cache.(i) with
            | None -> cache.(i) <- Some candidate
            | Some curr when List.length candidate < List.length curr ->
              cache.(i) <- Some candidate
            | _ -> ()
      ) coins
    done;
    match cache.(target) with
    | Some coins -> Ok (List.sort compare coins)
    | None -> Error "can't make target with given coins"
```
**Status:** [ ]

### 062: List Operations
**Source:** https://exercism.org/tracks/ocaml/exercises/list-ops
**Topic:** Implement map, filter, fold, reverse, append, concat from scratch
**Difficulty:** Intermediate
**Category:** Higher-Order Functions & Recursion
**OCaml:**
```ocaml
let length l =
  let rec go acc = function [] -> acc | _ :: t -> go (acc + 1) t in
  go 0 l

let reverse l =
  let rec go acc = function [] -> acc | h :: t -> go (h :: acc) t in
  go [] l

let map ~f l =
  let rec go acc = function
    | [] -> acc | h :: t -> go (f h :: acc) t in
  go [] l |> reverse

let filter ~f l =
  let rec go acc = function
    | [] -> acc
    | h :: t when f h -> go (h :: acc) t
    | _ :: t -> go acc t in
  go [] l |> reverse

let rec fold ~init:acc ~f = function
  | [] -> acc | h :: t -> fold ~init:(f acc h) ~f t

let append a b = List.fold_right (fun x acc -> x :: acc) a b

let concat ll = List.fold_right append ll []
```
**Status:** [ ]

### 063: Robot Name
**Source:** https://exercism.org/tracks/ocaml/exercises/robot-name
**Topic:** Mutable state with unique name generation
**Difficulty:** Intermediate
**Category:** Mutation & State
**OCaml:**
```ocaml
type robot = { mutable name_str : string }

let used_names = Hashtbl.create 1000

let random_name () =
  let letter () = Char.chr (Char.code 'A' + Random.int 26) in
  let digit () = Char.chr (Char.code '0' + Random.int 10) in
  Printf.sprintf "%c%c%c%c%c" (letter ()) (letter ()) (digit ()) (digit ()) (digit ())

let fresh_name () =
  let rec try_name () =
    let n = random_name () in
    if Hashtbl.mem used_names n then try_name ()
    else (Hashtbl.add used_names n true; n)
  in try_name ()

let new_robot () = Random.self_init (); { name_str = fresh_name () }
let name r = r.name_str
let reset r = r.name_str <- fresh_name ()
```
**Status:** [ ]

### 064: Beer Song
**Source:** https://exercism.org/tracks/ocaml/exercises/beer-song
**Topic:** String generation with conditional formatting
**Difficulty:** Intermediate
**Category:** Strings & Recursion
**OCaml:**
```ocaml
let bottles = function
  | 0 -> "no more bottles"
  | 1 -> "1 bottle"
  | n -> string_of_int n ^ " bottles"

let verse = function
  | 0 -> String.capitalize_ascii (bottles 0) ^
    " of beer on the wall, " ^ bottles 0 ^ " of beer.\n" ^
    "Go to the store and buy some more, 99 bottles of beer on the wall."
  | n ->
    bottles n ^ " of beer on the wall, " ^ bottles n ^ " of beer.\n" ^
    "Take " ^ (if n > 1 then "one" else "it") ^ " down and pass it around, " ^
    bottles (n - 1) ^ " of beer on the wall."

let recite start count =
  List.init count (fun i -> verse (start - i))
  |> String.concat "\n\n"
```
**Status:** [ ]

### 065: Say (Numbers to English)
**Source:** https://exercism.org/tracks/ocaml/exercises/say
**Topic:** Recursive number-to-words conversion with large number support
**Difficulty:** Advanced
**Category:** Recursion & Pattern Matching
**OCaml:**
```ocaml
let rec in_english_impl = function
  | 0L -> "zero" | 1L -> "one" | 2L -> "two" | 3L -> "three"
  | 4L -> "four" | 5L -> "five" | 6L -> "six" | 7L -> "seven"
  | 8L -> "eight" | 9L -> "nine" | 10L -> "ten" | 11L -> "eleven"
  | 12L -> "twelve" | 13L -> "thirteen" | 14L -> "fourteen"
  | 15L -> "fifteen" | 16L -> "sixteen" | 17L -> "seventeen"
  | 18L -> "eighteen" | 19L -> "nineteen" | 20L -> "twenty"
  | 30L -> "thirty" | 40L -> "forty" | 50L -> "fifty"
  | 60L -> "sixty" | 70L -> "seventy" | 80L -> "eighty" | 90L -> "ninety"
  | n when n <= 99L ->
    in_english_impl (Int64.mul 10L (Int64.div n 10L)) ^ "-" ^
    in_english_impl (Int64.rem n 10L)
  | n when n <= 999L ->
    in_english_impl (Int64.div n 100L) ^ " hundred" ^
    (let r = Int64.rem n 100L in if r = 0L then "" else " " ^ in_english_impl r)
  | n when n <= 999_999L ->
    in_english_impl (Int64.div n 1_000L) ^ " thousand" ^
    (let r = Int64.rem n 1_000L in if r = 0L then "" else " " ^ in_english_impl r)
  | n when n <= 999_999_999L ->
    in_english_impl (Int64.div n 1_000_000L) ^ " million" ^
    (let r = Int64.rem n 1_000_000L in if r = 0L then "" else " " ^ in_english_impl r)
  | n ->
    in_english_impl (Int64.div n 1_000_000_000L) ^ " billion" ^
    (let r = Int64.rem n 1_000_000_000L in if r = 0L then "" else " " ^ in_english_impl r)

let in_english n =
  if n < 0L || n >= 1_000_000_000_000L then Error "input out of range"
  else Ok (in_english_impl n)
```
**Status:** [ ]

### 066: Minesweeper
**Source:** https://exercism.org/tracks/ocaml/exercises/minesweeper
**Topic:** 2D grid annotation with neighbor counting
**Difficulty:** Intermediate
**Category:** Arrays & 2D Grids
**OCaml:**
```ocaml
let annotate strings =
  let rows = Array.of_list strings in
  let h = Array.length rows in
  if h = 0 then [] else
  let w = String.length rows.(0) in
  let grid = Array.init h (fun r -> Array.init w (fun c -> rows.(r).[c])) in
  let count r c =
    let n = ref 0 in
    for dr = -1 to 1 do for dc = -1 to 1 do
      if not (dr = 0 && dc = 0) then
        let r' = r + dr and c' = c + dc in
        if r' >= 0 && r' < h && c' >= 0 && c' < w && grid.(r').(c') = '*' then
          incr n
    done done; !n
  in
  List.init h (fun r ->
    String.init w (fun c ->
      if grid.(r).(c) = '*' then '*'
      else match count r c with 0 -> ' ' | n -> Char.chr (n + 48)))
```
**Status:** [ ]

### 067: Bowling
**Source:** https://exercism.org/tracks/ocaml/exercises/bowling
**Topic:** Complex state machine for bowling score calculation
**Difficulty:** Advanced
**Category:** State Machines & Validation
**OCaml:**
```ocaml
type game = { rolls: int list; frame: int }

let new_game = { rolls = []; frame = 0 }

let roll pins game =
  if pins < 0 then Error "Negative roll is invalid"
  else if pins > 10 then Error "Pin count exceeds pins on the lane"
  else Ok { game with rolls = game.rolls @ [pins] }

let score game =
  let rolls = Array.of_list game.rolls in
  let len = Array.length rolls in
  let get i = if i < len then rolls.(i) else 0 in
  let rec go frame i =
    if frame >= 10 then Ok 0
    else if i >= len then Error "Score cannot be taken until the end of the game"
    else
      let first = get i in
      if first = 10 then (* strike *)
        match go (frame + 1) (i + 1) with
        | Error e -> Error e
        | Ok rest -> Ok (10 + get (i+1) + get (i+2) + rest)
      else
        let second = get (i + 1) in
        if first + second > 10 then Error "Pin count exceeds pins on the lane"
        else if first + second = 10 then (* spare *)
          match go (frame + 1) (i + 2) with
          | Error e -> Error e
          | Ok rest -> Ok (10 + get (i+2) + rest)
        else
          match go (frame + 1) (i + 2) with
          | Error e -> Error e
          | Ok rest -> Ok (first + second + rest)
  in
  go 0 0
```
**Status:** [ ]

### 068: DnD Character
**Source:** https://exercism.org/tracks/ocaml/exercises/dnd-character
**Topic:** Random generation with record types
**Difficulty:** Intermediate
**Category:** Records & Randomness
**OCaml:**
```ocaml
type character = {
  charisma : int; constitution : int; dexterity : int;
  hitpoints : int; intelligence : int; strength : int; wisdom : int;
}

let ability () =
  let rolls = List.init 4 (fun _ -> 1 + Random.int 6) in
  let sorted = List.sort compare rolls |> List.tl in
  List.fold_left (+) 0 sorted

let modifier ~score =
  int_of_float (floor ((float_of_int score -. 10.0) /. 2.0))

let generate_character () =
  let () = Random.self_init () in
  let con = ability () in
  { charisma = ability (); constitution = con; dexterity = ability ();
    hitpoints = 10 + modifier ~score:con;
    intelligence = ability (); strength = ability (); wisdom = ability () }
```
**Status:** [ ]

### 069: Custom Set (Functor)
**Source:** https://exercism.org/tracks/ocaml/exercises/custom-set
**Topic:** Functor-based set implementation with sorted lists
**Difficulty:** Advanced
**Category:** Functors & Modules
**OCaml:**
```ocaml
module type ELEMENT = sig
  type t
  val compare : t -> t -> int
end

module Make (El : ELEMENT) = struct
  type t = El.t list

  let is_empty = function [] -> true | _ -> false
  let is_member l n = List.exists (fun x -> El.compare x n = 0) l
  let of_list = List.sort_uniq El.compare
  let add l x = of_list (x :: l)
  let equal a b = of_list a = of_list b

  let is_subset x y = List.for_all (fun e -> is_member y e) x
  let is_disjoint x y = not (List.exists (fun e -> is_member y e) x)

  let union a b = of_list (a @ b)
  let intersect a b = List.filter (fun e -> is_member b e) a |> of_list
  let difference a b = List.filter (fun e -> not (is_member b e)) a |> of_list
end
```
**Status:** [ ]

### 070: Dominoes Chain
**Source:** https://exercism.org/tracks/ocaml/exercises/dominoes
**Topic:** Backtracking search for Eulerian path in domino chain
**Difficulty:** Advanced
**Category:** Backtracking & Graph Theory
**OCaml:**
```ocaml
type dominoe = int * int

let chain = function
  | [] -> Some []
  | first :: rest ->
    let rec go stones path =
      match stones with
      | [] ->
        let (a, _) = List.hd path and (_, b) = List.hd (List.rev path) in
        if a = b then Some (List.rev path) else None
      | _ ->
        let right_end = snd (List.hd path) in
        let rec try_each before = function
          | [] -> None
          | (a, b) :: after ->
            let remaining = List.rev_append before after in
            let result =
              if a = right_end then go remaining ((a, b) :: path)
              else if b = right_end then go remaining ((b, a) :: path)
              else None
            in
            match result with
            | Some _ -> result
            | None -> try_each ((a, b) :: before) after
        in
        try_each [] stones
    in
    go rest [first]
```
**Status:** [ ]

### 071: Forth Interpreter
**Source:** https://exercism.org/tracks/ocaml/exercises/forth
**Topic:** Stack-based language interpreter with user-defined words
**Difficulty:** Advanced
**Category:** Interpreters & Stacks
**OCaml:**
```ocaml
type forth_state = {
  stack : int list;
  defs : (string * string list) list;
}

let empty = { stack = []; defs = [] }

let lookup word state =
  List.assoc_opt (String.uppercase_ascii word) state.defs

let rec eval_word word state =
  match lookup word state with
  | Some expansion -> eval_words expansion state
  | None ->
    match int_of_string_opt word with
    | Some n -> Ok { state with stack = n :: state.stack }
    | None ->
      match String.uppercase_ascii word, state.stack with
      | "+", a :: b :: rest -> Ok { state with stack = (b + a) :: rest }
      | "-", a :: b :: rest -> Ok { state with stack = (b - a) :: rest }
      | "*", a :: b :: rest -> Ok { state with stack = (b * a) :: rest }
      | "/", 0 :: _ :: _ -> Error "divide by zero"
      | "/", a :: b :: rest -> Ok { state with stack = (b / a) :: rest }
      | "DUP", a :: rest -> Ok { state with stack = a :: a :: rest }
      | "DROP", _ :: rest -> Ok { state with stack = rest }
      | "SWAP", a :: b :: rest -> Ok { state with stack = b :: a :: rest }
      | "OVER", a :: b :: rest -> Ok { state with stack = b :: a :: b :: rest }
      | _ -> Error ("unknown word: " ^ word)

and eval_words words state =
  List.fold_left (fun acc w ->
    match acc with Error _ -> acc | Ok s -> eval_word w s
  ) (Ok state) words

let eval_line line state =
  let words = String.split_on_char ' ' line in
  match words with
  | ":" :: name :: rest when List.length rest > 0 ->
    let body = List.rev (List.tl (List.rev rest)) in
    if int_of_string_opt name <> None then Error "cannot redefine numbers"
    else
      (* Expand existing definitions in body *)
      let expanded = List.concat_map (fun w ->
        match lookup w state with Some exp -> exp | None -> [w]
      ) body in
      Ok { state with defs = (String.uppercase_ascii name, expanded) :: state.defs }
  | _ -> eval_words words state

let evaluate lines =
  let result = List.fold_left (fun acc line ->
    match acc with Error _ -> acc | Ok s -> eval_line line s
  ) (Ok empty) lines in
  match result with
  | Error e -> Error e
  | Ok state -> Ok (List.rev state.stack)
```
**Status:** [ ]

### 072: Zipper (Functional Tree Navigation)
**Source:** https://exercism.org/tracks/ocaml/exercises/zipper
**Topic:** Zipper data structure for efficient tree traversal and modification
**Difficulty:** Advanced
**Category:** Functional Data Structures
**OCaml:**
```ocaml
type 'a tree = { value : 'a; left : 'a tree option; right : 'a tree option }

type 'a trail =
  | Top
  | Left of 'a * 'a tree option * 'a trail
  | Right of 'a * 'a tree option * 'a trail

type 'a zipper = { focus : 'a tree; trail : 'a trail }

let of_tree t = { focus = t; trail = Top }

let rec to_tree z =
  let t = z.focus in
  match z.trail with
  | Top -> t
  | Left (v, r, up) ->
    to_tree { focus = { value = v; left = Some t; right = r }; trail = up }
  | Right (v, l, up) ->
    to_tree { focus = { value = v; left = l; right = Some t }; trail = up }

let value z = z.focus.value

let left z = match z.focus.left with
  | None -> None
  | Some t -> Some { focus = t; trail = Left (z.focus.value, z.focus.right, z.trail) }

let right z = match z.focus.right with
  | None -> None
  | Some t -> Some { focus = t; trail = Right (z.focus.value, z.focus.left, z.trail) }

let up z = match z.trail with
  | Top -> None
  | Left (v, r, up) ->
    Some { focus = { value = v; left = Some z.focus; right = r }; trail = up }
  | Right (v, l, up) ->
    Some { focus = { value = v; left = l; right = Some z.focus }; trail = up }

let set_value v z = { z with focus = { z.focus with value = v } }
let set_left t z = { z with focus = { z.focus with left = t } }
let set_right t z = { z with focus = { z.focus with right = t } }
```
**Status:** [ ]

### 073: Rectangles
**Source:** https://exercism.org/tracks/ocaml/exercises/rectangles
**Topic:** Counting rectangles in ASCII art by tracing edges
**Difficulty:** Advanced
**Category:** 2D Grids & Graph Traversal
**OCaml:**
```ocaml
let count_rectangles pic =
  let h = Array.length pic in
  if h = 0 then 0 else
  let w = String.length pic.(0) in
  let at r c = if r >= 0 && r < h && c >= 0 && c < w then pic.(r).[c] else '.' in
  let count = ref 0 in
  for r1 = 0 to h - 1 do
    for c1 = 0 to w - 1 do
      if at r1 c1 = '+' then
        for r2 = r1 + 1 to h - 1 do
          for c2 = c1 + 1 to w - 1 do
            if at r1 c2 = '+' && at r2 c1 = '+' && at r2 c2 = '+' then
              let top_ok = ref true and bot_ok = ref true
              and lft_ok = ref true and rgt_ok = ref true in
              for c = c1 + 1 to c2 - 1 do
                if at r1 c <> '-' && at r1 c <> '+' then top_ok := false;
                if at r2 c <> '-' && at r2 c <> '+' then bot_ok := false
              done;
              for r = r1 + 1 to r2 - 1 do
                if at r c1 <> '|' && at r c1 <> '+' then lft_ok := false;
                if at r c2 <> '|' && at r c2 <> '+' then rgt_ok := false
              done;
              if !top_ok && !bot_ok && !lft_ok && !rgt_ok then incr count
          done
        done
    done
  done;
  !count
```
**Status:** [ ]

### 074: Palindrome Products
**Source:** https://exercism.org/tracks/ocaml/exercises/palindrome-products
**Topic:** Finding smallest/largest palindrome products with factor tracking
**Difficulty:** Advanced
**Category:** Math & Search
**OCaml:**
```ocaml
let is_palindrome n =
  let s = string_of_int n in
  let len = String.length s in
  let rec check i = i >= len / 2 || (s.[i] = s.[len - 1 - i] && check (i + 1)) in
  check 0

let smallest ~min ~max =
  if min > max then Error "min must be <= max" else
  let best = ref None in
  for x = min to max do for y = x to max do
    let p = x * y in
    if is_palindrome p then
      match !best with
      | None -> best := Some (p, [(x, y)])
      | Some (b, fs) when p < b -> best := Some (p, [(x, y)])
      | Some (b, fs) when p = b -> best := Some (b, (x, y) :: fs)
      | _ -> ()
  done done;
  Ok !best

let largest ~min ~max =
  if min > max then Error "min must be <= max" else
  let best = ref None in
  for x = min to max do for y = x to max do
    let p = x * y in
    if is_palindrome p then
      match !best with
      | None -> best := Some (p, [(x, y)])
      | Some (b, fs) when p > b -> best := Some (p, [(x, y)])
      | Some (b, fs) when p = b -> best := Some (b, (x, y) :: fs)
      | _ -> ()
  done done;
  Ok !best
```
**Status:** [ ]

### 075: Connect (Hex Game)
**Source:** https://exercism.org/tracks/ocaml/exercises/connect
**Topic:** Flood-fill / BFS on hex grid to detect connected paths
**Difficulty:** Advanced
**Category:** Graph Algorithms & BFS
**OCaml:**
```ocaml
type player = O | X

let connect board =
  let rows = Array.of_list board in
  let h = Array.length rows in
  if h = 0 then None else
  let parse r = String.to_seq rows.(r) |> Seq.filter (fun c -> c <> ' ')
    |> Array.of_seq in
  let grid = Array.init h parse in
  let w = Array.length grid.(0) in
  let deltas = [(-1,0);(-1,1);(0,-1);(0,1);(1,-1);(1,0)] in
  let bfs start_cells goal ch =
    let visited = Array.init h (fun _ -> Array.make w false) in
    let queue = Queue.create () in
    List.iter (fun (r, c) ->
      if grid.(r).(c) = ch then (visited.(r).(c) <- true; Queue.add (r, c) queue)
    ) start_cells;
    let found = ref false in
    while not (Queue.is_empty queue) && not !found do
      let (r, c) = Queue.pop queue in
      if goal r c then found := true
      else List.iter (fun (dr, dc) ->
        let r' = r + dr and c' = c + dc in
        if r' >= 0 && r' < h && c' >= 0 && c' < w &&
           not visited.(r').(c') && grid.(r').(c') = ch then
          (visited.(r').(c') <- true; Queue.add (r', c') queue)
      ) deltas
    done; !found
  in
  let x_start = List.init h (fun r -> (r, 0)) in
  if bfs x_start (fun _ c -> c = w - 1) 'X' then Some X
  else
    let o_start = List.init w (fun c -> (0, c)) in
    if bfs o_start (fun r _ -> r = h - 1) 'O' then Some O
    else None
```
**Status:** [ ]

### 076: React (Reactive Cells)
**Source:** https://exercism.org/tracks/ocaml/exercises/react
**Topic:** Reactive programming — input cells, compute cells, and callbacks
**Difficulty:** Advanced
**Category:** Reactive Programming & State
**OCaml:**
```ocaml
type 'a cell =
  | Input of { mutable value : 'a; id : int }
  | Compute1 of { dep : 'a cell; f : 'a -> 'a; mutable value : 'a;
      mutable callbacks : (int * ('a -> unit)) list; id : int }
  | Compute2 of { dep1 : 'a cell; dep2 : 'a cell; f : 'a -> 'a -> 'a;
      mutable value : 'a; mutable callbacks : (int * ('a -> unit)) list; id : int }

let next_id = ref 0
let fresh () = let i = !next_id in incr next_id; i
let cb_id = ref 0
let fresh_cb () = let i = !cb_id in incr cb_id; i

let value_of = function
  | Input c -> c.value | Compute1 c -> c.value | Compute2 c -> c.value

let create_input v = Input { value = v; id = fresh () }

let create_compute1 dep ~f =
  Compute1 { dep; f; value = f (value_of dep);
             callbacks = []; id = fresh () }

let create_compute2 dep1 dep2 ~f =
  Compute2 { dep1; dep2; f; value = f (value_of dep1) (value_of dep2);
             callbacks = []; id = fresh () }

(* Simplified: caller must manually propagate *)
let set_value cell v = match cell with
  | Input c -> c.value <- v | _ -> failwith "cannot set compute"

let recompute = function
  | Compute1 c ->
    let nv = c.f (value_of c.dep) in
    if nv <> c.value then (c.value <- nv;
      List.iter (fun (_, cb) -> cb nv) c.callbacks)
  | Compute2 c ->
    let nv = c.f (value_of c.dep1) (value_of c.dep2) in
    if nv <> c.value then (c.value <- nv;
      List.iter (fun (_, cb) -> cb nv) c.callbacks)
  | _ -> ()

let add_callback cell ~f = match cell with
  | Compute1 c -> let id = fresh_cb () in c.callbacks <- (id, f) :: c.callbacks; id
  | Compute2 c -> let id = fresh_cb () in c.callbacks <- (id, f) :: c.callbacks; id
  | _ -> -1

let remove_callback cell id = match cell with
  | Compute1 c -> c.callbacks <- List.filter (fun (i,_) -> i <> id) c.callbacks
  | Compute2 c -> c.callbacks <- List.filter (fun (i,_) -> i <> id) c.callbacks
  | _ -> ()
```
**Status:** [ ]

### 077: Scrabble Score
**Source:** https://exercism.org/tracks/ocaml/exercises/scrabble-score
**Topic:** Character scoring with lookup table
**Difficulty:** Beginner
**Category:** Strings & Lookup
**OCaml:**
```ocaml
let letter_score = function
  | 'A'|'E'|'I'|'O'|'U'|'L'|'N'|'R'|'S'|'T' -> 1
  | 'D'|'G' -> 2
  | 'B'|'C'|'M'|'P' -> 3
  | 'F'|'H'|'V'|'W'|'Y' -> 4
  | 'K' -> 5
  | 'J'|'X' -> 8
  | 'Q'|'Z' -> 10
  | _ -> 0

let score word =
  let w = String.uppercase_ascii word in
  let total = ref 0 in
  String.iter (fun c -> total := !total + letter_score c) w;
  !total
```
**Status:** [ ]

### 078: Series (Substring Slicing)
**Source:** https://exercism.org/tracks/ocaml/exercises/series
**Topic:** Generating consecutive substrings of a given length
**Difficulty:** Beginner
**Category:** Strings & Slicing
**OCaml:**
```ocaml
let slices s n =
  if n <= 0 || n > String.length s then []
  else List.init (String.length s - n + 1) (fun i -> String.sub s i n)
```
**Status:** [ ]

### 079: Protein Translation
**Source:** https://exercism.org/tracks/ocaml/exercises/protein-translation
**Topic:** Codon-to-protein mapping with stop codon handling
**Difficulty:** Beginner
**Category:** Pattern Matching & Strings
**OCaml:**
```ocaml
let codon_to_protein = function
  | "AUG" -> Some "Methionine"
  | "UUU" | "UUC" -> Some "Phenylalanine"
  | "UUA" | "UUG" -> Some "Leucine"
  | "UCU" | "UCC" | "UCA" | "UCG" -> Some "Serine"
  | "UAU" | "UAC" -> Some "Tyrosine"
  | "UGU" | "UGC" -> Some "Cysteine"
  | "UGG" -> Some "Tryptophan"
  | "UAA" | "UAG" | "UGA" -> None  (* STOP *)
  | _ -> Some "Unknown"

let proteins rna =
  let len = String.length rna in
  let rec go i acc =
    if i + 3 > len then List.rev acc
    else match codon_to_protein (String.sub rna i 3) with
      | None -> List.rev acc
      | Some p -> go (i + 3) (p :: acc)
  in
  go 0 []
```
**Status:** [ ]

### 080: Queen Attack
**Source:** https://exercism.org/tracks/ocaml/exercises/queen-attack
**Topic:** Chess queen placement validation and attack detection
**Difficulty:** Beginner
**Category:** Math & Validation
**OCaml:**
```ocaml
let create ~row ~column =
  if row < 0 || row > 7 || column < 0 || column > 7 then
    Error "invalid position"
  else Ok (row, column)

let can_attack (r1, c1) (r2, c2) =
  r1 = r2 || c1 = c2 || abs (r1 - r2) = abs (c1 - c2)
```
**Status:** [ ]

### 081: Clock (Modular Arithmetic)
**Source:** https://exercism.org/tracks/ocaml/exercises/clock
**Topic:** Time representation with modular arithmetic and equality
**Difficulty:** Intermediate
**Category:** Modular Arithmetic & Types
**OCaml:**
```ocaml
type clock = { hours : int; minutes : int }

let normalize h m =
  let total = h * 60 + m in
  let total = ((total mod 1440) + 1440) mod 1440 in
  { hours = total / 60; minutes = total mod 60 }

let create ~hours ~minutes = normalize hours minutes

let add c m = normalize c.hours (c.minutes + m)
let sub c m = normalize c.hours (c.minutes - m)

let display c = Printf.sprintf "%02d:%02d" c.hours c.minutes

let equal a b = a.hours = b.hours && a.minutes = b.minutes
```
**Status:** [ ]

### 082: Complex Numbers
**Source:** https://exercism.org/tracks/ocaml/exercises/complex-numbers
**Topic:** Complex number arithmetic with real/imaginary parts
**Difficulty:** Intermediate
**Category:** Math & Records
**OCaml:**
```ocaml
type complex = { re : float; im : float }

let create re im = { re; im }
let real c = c.re
let imaginary c = c.im

let add a b = { re = a.re +. b.re; im = a.im +. b.im }
let sub a b = { re = a.re -. b.re; im = a.im -. b.im }

let mul a b = {
  re = a.re *. b.re -. a.im *. b.im;
  im = a.re *. b.im +. a.im *. b.re
}

let conjugate c = { re = c.re; im = -. c.im }
let abs c = sqrt (c.re *. c.re +. c.im *. c.im)

let div a b =
  let denom = b.re *. b.re +. b.im *. b.im in
  { re = (a.re *. b.re +. a.im *. b.im) /. denom;
    im = (a.im *. b.re -. a.re *. b.im) /. denom }

let exp c =
  let ea = Stdlib.exp c.re in
  { re = ea *. cos c.im; im = ea *. sin c.im }
```
**Status:** [ ]

### 025: Sorting algorithms/Quicksort
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Quicksort
**Topic:** Rosetta Code Sorting algorithms/Quicksort implementation in OCaml
**Difficulty:** Intermediate
**Category:** Sorting
**OCaml:**
```ocaml
let rec quicksort gt = function
  | [] -> []
  | x::xs ->
      let ys, zs = List.partition (gt x) xs in
      (quicksort gt ys) @ (x :: (quicksort gt zs))
 
let _ =
  quicksort (>) [4; 65; 2; -31; 0; 99; 83; 782; 1]
```
**Status:** [ ]

### 026: Sorting algorithms/Selection sort
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Selection_sort
**Topic:** Rosetta Code Sorting algorithms/Selection sort implementation in OCaml
**Difficulty:** Intermediate
**Category:** Sorting
**OCaml:**
```ocaml
let rec selection_sort = function
    [] -> []
  | first::lst ->
      let rec select_r small output = function
          [] -> small :: selection_sort output
        | x::xs when x < small -> select_r x (small::output) xs
        | x::xs                -> select_r small (x::output) xs
      in
      select_r first [] lst
```
**Status:** [ ]

### 027: Sorting algorithms/Bubble sort
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Bubble_sort
**Topic:** Rosetta Code Sorting algorithms/Bubble sort implementation in OCaml
**Difficulty:** Intermediate
**Category:** Sorting
**OCaml:**
```ocaml
let rec bsort s =
  let rec _bsort = function
    | x :: x2 :: xs when x > x2 ->
        x2 :: _bsort (x :: xs)
    | x :: x2 :: xs ->
        x :: _bsort (x2 :: xs)
    | s -> s
  in
  let t = _bsort s in
    if t = s then t
    else bsort t
```
**Status:** [ ]

### 028: Sorting algorithms/Heapsort
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Heapsort
**Topic:** Rosetta Code Sorting algorithms/Heapsort implementation in OCaml
**Difficulty:** Intermediate
**Category:** Sorting
**OCaml:**
```ocaml
let heapsort a =

  let swap i j =
    let t = a.(i) in a.(i) <- a.(j); a.(j) <- t in

  let sift k l =
    let rec check x y =
      if 2*x+1 < l then
        let ch =
          if y < l-1 && a.(y) < a.(y+1) then y+1 else y in
        if a.(x) < a.(ch) then (swap x ch; check ch (2*ch+1)) in
    check k (2*k+1) in

  let len = Array.length a in

  for start = (len/2)-1 downto 0 do
    sift start len;
  done;

  for term = len-1 downto 1 do
    swap term 0;
    sift 0 term;
  done;;
```
**Status:** [ ]

### 029: Sorting algorithms/Counting sort
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Counting_sort
**Topic:** Rosetta Code Sorting algorithms/Counting sort implementation in OCaml
**Difficulty:** Intermediate
**Category:** Sorting
**OCaml:**
```ocaml
let counting_sort_array arr lo hi =
  let count = Array.make (hi-lo+1) 0 in
    Array.iter (fun i -> count.(i-lo) <- count.(i-lo) + 1) arr;
    Array.concat (Array.to_list (Array.mapi (fun i x -> Array.make x (lo+i)) count))
```
**Status:** [ ]

### 030: Sorting algorithms/Shell sort
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Shell_sort
**Topic:** Rosetta Code Sorting algorithms/Shell sort implementation in OCaml
**Difficulty:** Intermediate
**Category:** Sorting
**OCaml:**
```ocaml
let shellsort a =
  let len = Array.length a in
  let incSequence = [| 412771; 165103; 66041; 26417; 10567;
                       4231; 1693; 673; 269; 107; 43; 17; 7; 3; 1 |] in
 
  Array.iter (fun increment ->
    if (increment * 2) <= len then
      for i = increment to pred len do
        let temp = a.(i) in
        let rec loop j =
          if j < 0 || a.(j) <= temp then (j)
          else begin
            a.(j + increment) <- a.(j);
            loop (j - increment)
          end
        in
        let j = loop (i - increment) in
        a.(j + increment) <- temp;
      done;
  ) incSequence;
;;
```
**Status:** [ ]

### 031: Sorting algorithms/Bogosort
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Bogosort
**Topic:** Rosetta Code Sorting algorithms/Bogosort implementation in OCaml
**Difficulty:** Intermediate
**Category:** Sorting
**OCaml:**
```ocaml
let rec is_sorted comp = function
 | e1 :: e2 :: r -> comp e1 e2 <= 0 && is_sorted comp (e2 :: r)
 | _             -> true

(* Fisher-Yates shuffle on lists; uses temp array *)
let shuffle l =
  let ar = Array.of_list l in
    for n = Array.length ar - 1 downto 1 do
      let k = Random.int (n+1) in
      let temp = ar.(k) in (* swap ar.(k) and ar.(n) *)
        ar.(k) <- ar.(n);
        ar.(n) <- temp
    done;
    Array.to_list ar

let rec bogosort li =
  if is_sorted compare li then
    li
  else
    bogosort (shuffle li)
```
**Status:** [ ]

### 032: Sorting algorithms/Cocktail sort
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Cocktail_sort
**Topic:** Rosetta Code Sorting algorithms/Cocktail sort implementation in OCaml
**Difficulty:** Intermediate
**Category:** Sorting
**OCaml:**
```ocaml
let swap a i j =
  let tmp = a.(i) in
  a.(i) <- a.(j);
  a.(j) <- tmp;
;;

let cocktail_sort a =
  let begin_ = ref(-1)
  and end_ = ref(Array.length a - 2) in
  let swapped = ref true in
  try while !swapped do
    swapped := false;
    incr begin_;
    for i = !begin_ to !end_ do
      if a.(i) > a.(i+1) then begin
        swap a (i) (i+1);
        swapped := true;
      end;
    done;
    if !swapped = false then raise Exit;
    swapped := false;
    decr end_;
    for i = !end_ downto !begin_ do
      if a.(i) > a.(i+1) then begin
        swap a (i) (i+1);
        swapped := true
      end;
    done;
  done with Exit -> ()
;;

let () =
  let a = [| 3; 7; 4; 9; 6; 1; 8; 5; 2; |] in
  cocktail_sort a;
  Array.iter (Printf.printf " %d") a;
  print_newline();
;;
```
**Status:** [ ]

### 033: Sorting algorithms/Gnome sort
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Gnome_sort
**Topic:** Rosetta Code Sorting algorithms/Gnome sort implementation in OCaml
**Difficulty:** Intermediate
**Category:** Sorting
**OCaml:**
```ocaml
# let gnome_sort a =
    let i = ref 1 
    and j = ref 2 in
    while !i < Array.length a do
      if a.(!i-1) <= a.(!i) then
      begin
        i := !j;
        j := !j + 1;
      end else begin
        swap a (!i-1) (!i);
        i := !i - 1;
        if !i = 0 then begin
          i := !j;
          j := !j + 1;
        end;
      end;
    done;
  ;;
val gnome_sort : 'a array -> unit = <fun>

# let a = [| 7; 9; 4; 2; 1; 3; 6; 5; 0; 8; |] ;;
val a : int array = [|7; 9; 4; 2; 1; 3; 6; 5; 0; 8|]

# gnome_sort a ;;
- : unit = ()

# a ;;
- : int array = [|0; 1; 2; 3; 4; 5; 6; 7; 8; 9|]
```
**Status:** [ ]

### 034: Sorting algorithms/Stooge sort
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Stooge_sort
**Topic:** Rosetta Code Sorting algorithms/Stooge sort implementation in OCaml
**Difficulty:** Intermediate
**Category:** Sorting
**OCaml:**
```ocaml
let swap ar i j =
  let tmp = ar.(i) in
  ar.(i) <- ar.(j);
  ar.(j) <- tmp

let stoogesort ar =
  let rec aux i j =
    if ar.(j) < ar.(i) then
      swap ar i j
    else if j - i > 1 then begin
      let t = (j - i + 1) / 3 in
      aux (i) (j-t);
      aux (i+t) (j);
      aux (i) (j-t);
    end
  in
  aux 0 (Array.length ar - 1)
```
**Status:** [ ]

### 035: Sorting algorithms/Strand sort
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Strand_sort
**Topic:** Rosetta Code Sorting algorithms/Strand sort implementation in OCaml
**Difficulty:** Intermediate
**Category:** Sorting
**OCaml:**
```ocaml
let rec strand_sort (cmp : 'a -> 'a -> int) : 'a list -> 'a list = function
   []    -> []
 | x::xs ->
   let rec extract_strand x = function
      [] -> [x], []
    | x1::xs when cmp x x1 <= 0 ->
      let strand, rest = extract_strand x1 xs in x::strand, rest
    | x1::xs ->
      let strand, rest = extract_strand x xs in strand, x1::rest
   in
   let strand, rest = extract_strand x xs in
   List.merge cmp strand (strand_sort cmp rest)
```
**Status:** [ ]

### 036: Sorting algorithms/Permutation sort
**Source:** https://rosettacode.org/wiki/Sorting_algorithms/Permutation_sort
**Topic:** Rosetta Code Sorting algorithms/Permutation sort implementation in OCaml
**Difficulty:** Intermediate
**Category:** Sorting
**OCaml:**
```ocaml
let rec sorted = function
 | e1 :: e2 :: r -> e1 <= e2 && sorted (e2 :: r)
 | _             -> true

let rec insert e = function
 | []          -> [[e]]
 | h :: t as l -> (e :: l) :: List.map (fun t' -> h :: t') (insert e t)

let permute xs = List.fold_right (fun h z -> List.concat (List.map (insert h) z))
                                 xs [[]]

let permutation_sort l = List.find sorted (permute l)
```
**Status:** [ ]

### 037: Factorial
**Source:** https://rosettacode.org/wiki/Factorial
**Topic:** Rosetta Code Factorial implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let rec factorial n =
  if n <= 0 then 1
  else n * factorial (n-1)
```
**Status:** [ ]

### 038: Greatest common divisor
**Source:** https://rosettacode.org/wiki/Greatest_common_divisor
**Topic:** Rosetta Code Greatest common divisor implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let rec gcd a = function
  | 0 -> a
  | b -> gcd b (a mod b)
```
**Status:** [ ]

### 039: Least common multiple
**Source:** https://rosettacode.org/wiki/Least_common_multiple
**Topic:** Rosetta Code Least common multiple implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let rec gcd u v =
  if v <> 0 then (gcd v (u mod v))
  else (abs u)

let lcm m n =
  match m, n with
  | 0, _ | _, 0 -> 0
  | m, n -> abs (m * n) / (gcd m n)

let () =
  Printf.printf "lcm(35, 21) = %d\n" (lcm 21 35)
```
**Status:** [ ]

### 040: Ackermann function
**Source:** https://rosettacode.org/wiki/Ackermann_function
**Topic:** Rosetta Code Ackermann function implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let rec a m n =
  if m=0 then (n+1) else
  if n=0 then (a (m-1) 1) else
  (a (m-1) (a m (n-1)))
```
**Status:** [ ]

### 041: Combinations
**Source:** https://rosettacode.org/wiki/Combinations
**Topic:** Rosetta Code Combinations implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let combinations m n =
  let rec c = function
    | (0,_) -> [[]]
    | (_,0) -> []
    | (p,q) -> List.append
               (List.map (List.cons (n-q)) (c (p-1, q-1)))
               (c (p , q-1))
  in c (m , n)


let () =
  let rec print_list = function
    | [] -> print_newline ()
    | hd :: tl -> print_int hd ; print_string " "; print_list tl      
  in List.iter print_list (combinations 3 5)
```
**Status:** [ ]

### 042: Combinations with repetitions
**Source:** https://rosettacode.org/wiki/Combinations_with_repetitions
**Topic:** Rosetta Code Combinations with repetitions implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let rec combs_with_rep k xxs =
  match k, xxs with
  | 0,  _ -> [[]]
  | _, [] -> []
  | k, x::xs ->
      List.map (fun ys -> x::ys) (combs_with_rep (k-1) xxs)
      @ combs_with_rep k xs
```
**Status:** [ ]

### 043: Permutations
**Source:** https://rosettacode.org/wiki/Permutations
**Topic:** Rosetta Code Permutations implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
(* Iterative, though loops are implemented as auxiliary recursive functions.
   Translation of Ada version. *)
let next_perm p =
	let n = Array.length p in
	let i = let rec aux i = 
		if (i < 0) || (p.(i) < p.(i+1)) then i
		else aux (i - 1) in aux (n - 2) in
	let rec aux j k = if j < k then
		let t = p.(j) in
			p.(j) <- p.(k);
			p.(k) <- t;
			aux (j + 1) (k - 1)
	else () in aux (i + 1) (n - 1);
	if i < 0 then false else
		let j = let rec aux j =
			if p.(j) > p.(i) then j
			else aux (j + 1) in aux (i + 1) in
		let t = p.(i) in
			p.(i) <- p.(j);
			p.(j) <- t;
			true;;

let print_perm p =
	let n = Array.length p in
	for i = 0 to n - 2 do
		print_int p.(i);
		print_string " "
	done;
	print_int p.(n - 1);
	print_newline ();;

let print_all_perm n =
	let p = Array.init n (function i -> i + 1) in
	print_perm p;
	while next_perm p do
		print_perm p
	done;;

print_all_perm 3;;
(* 1 2 3
   1 3 2
   2 1 3
   2 3 1
   3 1 2
   3 2 1 *)
```
**Status:** [ ]

### 044: Catalan numbers
**Source:** https://rosettacode.org/wiki/Catalan_numbers
**Topic:** Rosetta Code Catalan numbers implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let imp_catalan n =
  let return = ref 1 in
  for i = 1 to n do
    return := !return * 2 * (2 * i - 1) / (i + 1)
  done;
  !return

let rec catalan = function
  | 0 -> 1
  | n -> catalan (n - 1) * 2 * (2 * n - 1) / (n + 1)

let memoize f =
  let cache = Hashtbl.create 20 in
  fun n ->
    match Hashtbl.find_opt cache n with
    | None ->
      let x = f n in
      Hashtbl.replace cache n x;
      x
    | Some x -> x

let catalan_cache = Hashtbl.create 20

let rec memo_catalan n =
  if n = 0 then 1 else
    match Hashtbl.find_opt catalan_cache n with
    | None ->
      let x = memo_catalan (n - 1) * 2 * (2 * n - 1) / (n + 1) in
      Hashtbl.replace catalan_cache n x;
      x
    | Some x -> x

let () =
  if not !Sys.interactive then
    let bench label f n times =
      let start = Unix.gettimeofday () in
      begin
        for i = 1 to times do f n done;
        let stop = Unix.gettimeofday () in
        Printf.printf "%s (%d runs) : %.3f\n"
          label times (stop -. start)
      end in
    let show f g h f' n =
      for i = 0 to n do
        Printf.printf "%2d %7d %7d %7d %7d\n"
          i (f i) (g i) (h i) (f' i)
      done
    in
    List.iter (fun (l, f) -> bench l f 15 10_000_000)
      ["imperative", imp_catalan;
       "recursive", catalan;
       "hand-memoized", memo_catalan;
       "memoized", (memoize catalan)];
    show imp_catalan catalan memo_catalan (memoize catalan) 15
```
**Status:** [ ]

### 045: Chinese remainder theorem
**Source:** https://rosettacode.org/wiki/Chinese_remainder_theorem
**Topic:** Rosetta Code Chinese remainder theorem implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
exception Modular_inverse
let inverse_mod a = function
  | 1 -> 1
  | b -> let rec inner a b x0 x1 =
           if a <= 1 then x1
           else if  b = 0 then raise Modular_inverse
           else inner b (a mod b) (x1 - (a / b) * x0) x0 in
         let x = inner a b 0 1 in
         if x < 0 then x + b else x

let chinese_remainder_exn congruences = 
  let mtot = congruences
             |> List.map (fun (_, x) -> x)
             |> List.fold_left ( *) 1 in
   (List.fold_left (fun acc (r, n) ->
                  acc + r * inverse_mod (mtot / n) n * (mtot / n)
                ) 0 congruences)
             mod mtot

let chinese_remainder congruences =
   try Some (chinese_remainder_exn congruences)
   with modular_inverse -> None
```
**Status:** [ ]

### 046: Ethiopian multiplication
**Source:** https://rosettacode.org/wiki/Ethiopian_multiplication
**Topic:** Rosetta Code Ethiopian multiplication implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
(* We optimize a bit by not keeping the intermediate lists, and summing
   the right column on-the-fly, like in the C version.
   The function takes "halve" and "double" operators and "is_even" predicate as arguments,
   but also "is_zero", "zero" and "add". This allows for more general uses of the
   ethiopian multiplication. *)
let ethiopian is_zero is_even halve zero double add b a =
  let rec g a b r =
    if is_zero a
    then (r)
    else g (halve a) (double b) (if not (is_even a) then (add b r) else (r))
  in
  g a b zero
;;

let imul =
  ethiopian (( = ) 0) (fun x -> x mod 2 = 0) (fun x -> x / 2) 0 (( * ) 2) ( + );;

imul 17 34;;
(* - : int = 578 *)

(* Now, we have implemented the same algorithm as "rapid exponentiation",
   merely changing operator names *)
let ipow =
  ethiopian (( = ) 0) (fun x -> x mod 2 = 0) (fun x -> x / 2) 1 (fun x -> x*x) ( * )
;;

ipow 2 16;;
(* - : int = 65536 *)

(* still renaming operators, if "halving" is just subtracting one,
   and "doubling", adding one, then we get an addition *)
let iadd a b =
  ethiopian (( = ) 0) (fun x -> false) (pred) b (function x -> x) (fun x y -> succ y) 0 a
;;

iadd 421 1000;;
(* - : int = 1421 *)

(* One can do much more with "ethiopian multiplication",
   since the two "multiplicands" and the result may be of three different types,
   as shown by the typing system of ocaml *)

ethiopian;;
- : ('a -> bool) ->          (* is_zero *)
    ('a -> bool) ->          (* is_even *)
    ('a -> 'a) ->            (* halve *)
    'b ->                    (* zero *)
    ('c -> 'c) ->            (* double *)
    ('c -> 'b -> 'b) ->      (* add *)
    'c ->                    (* b *)
    'a ->                    (* a *)
    'b                       (* result *)
= <fun>

(* Here zero is the starting value for the accumulator of the sums
   of values in the right column in the original algorithm. But the "add"
   me do something else, see for example the RosettaCode page on 
   "Exponentiation operator". *)
```
**Status:** [ ]

### 047: Towers of Hanoi
**Source:** https://rosettacode.org/wiki/Towers_of_Hanoi
**Topic:** Rosetta Code Towers of Hanoi implementation in OCaml
**Difficulty:** Advanced
**Category:** Algorithms
**OCaml:**
```ocaml
let rec hanoi n a b c =
  if n <> 0 then begin
    hanoi (pred n) a c b;
    Printf.printf "Move disk from pole %d to pole %d\n" a b;
    hanoi (pred n) c b a
  end

let () =
  hanoi 4 1 2 3
```
**Status:** [ ]

### 048: Modular inverse
**Source:** https://rosettacode.org/wiki/Modular_inverse
**Topic:** Rosetta Code Modular inverse implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let mul_inv a = function 1 -> 1 | b ->
  let rec aux a b x0 x1 =
    if a <= 1 then x1 else
    if b = 0 then failwith "mul_inv" else
    aux b (a mod b) (x1 - (a / b) * x0) x0
  in
  let x = aux a b 0 1 in
  if x < 0 then x + b else x
```
**Status:** [ ]

### 049: Primality by trial division
**Source:** https://rosettacode.org/wiki/Primality_by_trial_division
**Topic:** Rosetta Code Primality by trial division implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let is_prime n =
  let rec test x =
    x * x > n || n mod x <> 0 && n mod (x + 2) <> 0 && test (x + 6)
  in
  if n < 5
  then n lor 1 = 3
  else n land 1 <> 0 && n mod 3 <> 0 && test 5
```
**Status:** [ ]

### 050: Amicable pairs
**Source:** https://rosettacode.org/wiki/Amicable_pairs
**Topic:** Rosetta Code Amicable pairs implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let rec isqrt n =
  if n = 1 then 1
  else let _n = isqrt (n - 1) in
    (_n + (n / _n)) / 2

let sum_divs n =
  let sum = ref 1 in
  for d = 2 to isqrt n do
    if (n mod d) = 0 then sum := !sum + (n / d + d);
  done;
  !sum
 
let () =
  for n = 2 to 20000 do
    let m = sum_divs n in
    if (m > n) then
      if (sum_divs m) = n then Printf.printf "%d %d\n" n m;
  done
```
**Status:** [ ]

### 051: Happy numbers
**Source:** https://rosettacode.org/wiki/Happy_numbers
**Topic:** Rosetta Code Happy numbers implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
open Num

let step =
	let rec aux s n =
	if n =/ Int 0 then s else
		let q = quo_num n (Int 10)
		and r = mod_num n (Int 10)
		in aux (s +/ (r */ r)) q
	in aux (Int 0) ;;

let happy n =
	let rec aux x y =
		if x =/ y then x else aux (step x) (step (step y))
	in (aux n (step n)) =/ Int 1 ;;

let first n =
	let rec aux v x n =
		if n = 0 then v else
			if happy x
			then aux (x::v) (x +/ Int 1) (n - 1)
			else aux v (x +/ Int 1) n
	in aux [ ] (Int 1) n ;;

List.iter print_endline (
	List.rev_map string_of_num (first 8)) ;;
```
**Status:** [ ]

### 052: Digital root
**Source:** https://rosettacode.org/wiki/Digital_root
**Topic:** Rosetta Code Digital root implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let rec digit_sum b n =
  if n < b then n else digit_sum b (n / b) + n mod b

let digital_root b n =
  let rec loop a x =
    if x < b then a, x else loop (succ a) (digit_sum b x)
  in
  loop 0 n

let () =
  let pr_fmt n (p, r) =
    Printf.printf "%u: additive persistence = %u, digital root = %u\n" n p r
  in
  List.iter
    (fun n -> pr_fmt n (digital_root 10 n))
    [627615; 39390; 588225; 393900588225]
```
**Status:** [ ]

### 053: Dot product
**Source:** https://rosettacode.org/wiki/Dot_product
**Topic:** Rosetta Code Dot product implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let dot = List.fold_left2 (fun z x y -> z +. x *. y) 0.

(*
# dot [1.0; 3.0; -5.0] [4.0; -2.0; -1.0];;
- : float = 3.
*)
```
**Status:** [ ]

### 054: Matrix multiplication
**Source:** https://rosettacode.org/wiki/Matrix_multiplication
**Topic:** Rosetta Code Matrix multiplication implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let matrix_multiply x y =
  let x0 = Array.length x
  and y0 = Array.length y in
  let y1 = if y0 = 0 then 0 else Array.length y.(0) in
  let z = Array.make_matrix x0 y1 0 in
  for i = 0 to x0-1 do
    for j = 0 to y1-1 do
      for k = 0 to y0-1 do
        z.(i).(j) <- z.(i).(j) + x.(i).(k) * y.(k).(j)
      done
    done
  done;
  z
```
**Status:** [ ]

### 055: Matrix transposition
**Source:** https://rosettacode.org/wiki/Matrix_transposition
**Topic:** Rosetta Code Matrix transposition implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
open Bigarray

let transpose b =
  let dim1 = Array2.dim1 b
  and dim2 = Array2.dim2 b in
  let kind = Array2.kind b
  and layout = Array2.layout b in
  let b' = Array2.create kind layout dim2 dim1 in
  for i=0 to pred dim1 do
    for j=0 to pred dim2 do
      b'.{j,i} <- b.{i,j}
    done;
  done;
  (b')
;;

let array2_display print newline b =
  for i=0 to Array2.dim1 b - 1 do
    for j=0 to Array2.dim2 b - 1 do
      print b.{i,j}
    done;
    newline();
  done;
;;

let a = Array2.of_array int c_layout [|
  [| 1; 2; 3; 4 |];
  [| 5; 6; 7; 8 |];
|]
;;

array2_display (Printf.printf " %d") print_newline (transpose a) ;;
```
**Status:** [ ]

### 056: Cumulative standard deviation
**Source:** https://rosettacode.org/wiki/Cumulative_standard_deviation
**Topic:** Rosetta Code Cumulative standard deviation implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let sqr x = x *. x

let stddev l =
  let n, sx, sx2 =
    List.fold_left
      (fun (n, sx, sx2) x -> succ n, sx +. x, sx2 +. sqr x)
      (0, 0., 0.) l
  in
  sqrt ((sx2 -. sqr sx /. float n) /. float n)

let _ =
  let l = [ 2.;4.;4.;4.;5.;5.;7.;9. ] in
  Printf.printf "List: ";
  List.iter (Printf.printf "%g  ") l;
  Printf.printf "\nStandard deviation: %g\n" (stddev l)
```
**Status:** [ ]

### 057: Reverse a string
**Source:** https://rosettacode.org/wiki/Reverse_a_string
**Topic:** Rosetta Code Reverse a string implementation in OCaml
**Difficulty:** Beginner
**Category:** Strings
**OCaml:**
```ocaml
let string_rev s =
  let len = String.length s in
  String.init len (fun i -> s.[len - 1 - i])

let () =
  print_endline (string_rev "Hello world!")
```
**Status:** [ ]

### 058: Palindrome detection
**Source:** https://rosettacode.org/wiki/Palindrome_detection
**Topic:** Rosetta Code Palindrome detection implementation in OCaml
**Difficulty:** Beginner
**Category:** Strings
**OCaml:**
```ocaml
let is_palindrome s =
    let l = String.length s in
    let rec comp n =
        n = 0 || (s.[l-n] = s.[n-1] && comp (n-1)) in
    comp (l / 2)
```
**Status:** [ ]

### 059: Levenshtein distance
**Source:** https://rosettacode.org/wiki/Levenshtein_distance
**Topic:** Rosetta Code Levenshtein distance implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let minimum a b c =
  min a (min b c)

let levenshtein_distance s t =
  let m = String.length s
  and n = String.length t in
  (* for all i and j, d.(i).(j) will hold the Levenshtein distance between
     the first i characters of s and the first j characters of t *)
  let d = Array.make_matrix (m+1) (n+1) 0 in

  for i = 0 to m do
    d.(i).(0) <- i  (* the distance of any first string to an empty second string *)
  done;
  for j = 0 to n do
    d.(0).(j) <- j  (* the distance of any second string to an empty first string *)
  done;

  for j = 1 to n do
    for i = 1 to m do

      if s.[i-1] = t.[j-1] then
        d.(i).(j) <- d.(i-1).(j-1)  (* no operation required *)
      else
        d.(i).(j) <- minimum
                       (d.(i-1).(j) + 1)   (* a deletion *)
                       (d.(i).(j-1) + 1)   (* an insertion *)
                       (d.(i-1).(j-1) + 1) (* a substitution *)
    done;
  done;

  d.(m).(n)
;;

let test s t =
  Printf.printf " %s -> %s = %d\n" s t (levenshtein_distance s t);
;;

let () =
  test "kitten" "sitting";
  test "rosettacode" "raisethysword";
;;
```
**Status:** [ ]

### 060: Longest common subsequence
**Source:** https://rosettacode.org/wiki/Longest_common_subsequence
**Topic:** Rosetta Code Longest common subsequence implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let longest xs ys = if List.length xs > List.length ys then xs else ys

let rec lcs a b = match a, b with
   [], _
 | _, []        -> []
 | x::xs, y::ys ->
    if x = y then
      x :: lcs xs ys
    else 
      longest (lcs a ys) (lcs xs b)
```
**Status:** [ ]

### 061: Soundex
**Source:** https://rosettacode.org/wiki/Soundex
**Topic:** Rosetta Code Soundex implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let c2d = function
  | 'B' | 'F' | 'P' | 'V' -> "1"
  | 'C' | 'G' | 'J' | 'K' | 'Q' | 'S' | 'X' | 'Z' -> "2"
  | 'D' | 'T' -> "3"
  | 'L' -> "4"
  | 'M' | 'N' -> "5"
  | 'R' -> "6"
  | _ -> ""

let rec dbl acc = function
  | [] -> (List.rev acc)
  | [c] -> List.rev(c::acc)
  | c1::(c2::_ as tl) ->
      if c1 = c2
      then dbl acc tl
      else dbl (c1::acc) tl

let pad s =
  match String.length s with
  | 0 -> s ^ "000"
  | 1 -> s ^ "00"
  | 2 -> s ^ "0"
  | 3 -> s
  | _ -> String.sub s 0 3

let soundex_aux rem =
  pad(String.concat "" (dbl [] (List.map c2d rem)))

let soundex s =
  let s = String.uppercase s in
  let cl = ref [] in
  String.iter (fun c -> cl := c :: !cl) s;
  match dbl [] (List.rev !cl) with
  | c::rem -> (String.make 1 c) ^ (soundex_aux rem)
  | [] -> invalid_arg "soundex"
```
**Status:** [ ]

### 062: Comma quibbling
**Source:** https://rosettacode.org/wiki/Comma_quibbling
**Topic:** Rosetta Code Comma quibbling implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
open Printf

let quibble list =
  let rec aux = function
    | a :: b :: c :: d :: rest -> a ^ ", " ^ aux (b :: c :: d :: rest)
    | [a; b; c] -> sprintf "%s, %s and %s}" a b c
    | [a; b] -> sprintf "%s and %s}" a b
    | [a] -> sprintf "%s}" a
    | [] -> "}" in
  "{" ^ aux list

let test () =
  [[];
   ["ABC"];
   ["ABC"; "DEF"];
   ["ABC"; "DEF"; "G"; "H"]]
  |> List.iter (fun list -> print_endline (quibble list))
```
**Status:** [ ]

### 063: Word wrap
**Source:** https://rosettacode.org/wiki/Word_wrap
**Topic:** Rosetta Code Word wrap implementation in OCaml
**Difficulty:** Beginner
**Category:** Strings
**OCaml:**
```ocaml
#load "str.cma"

let txt = "In olden times when wishing still helped one, there lived
a king whose daughters were all beautiful, but the youngest was so
beautiful that the sun itself, which has seen so much, was astonished
whenever it shone in her face.  Close by the king's castle lay a great
dark forest, and under an old lime-tree in the forest was a well, and
when the day was very warm, the king's child went out into the forest
and sat down by the side of the cool fountain, and when she was bored
she took a golden ball, and threw it up on high and caught it, and
this ball was her favorite plaything."

let () =
  let line_width = int_of_string Sys.argv.(1) in
  let words = Str.split (Str.regexp "[ \n]+") txt in
  let buf = Buffer.create 10 in
  let _ =
    List.fold_left (fun (width, sep) word ->
      let wlen = String.length word in
      let len = width + wlen + 1 in
      if len > line_width then
      begin
        Buffer.add_char buf '\n';
        Buffer.add_string buf word;
        (wlen, " ")
      end else begin
        Buffer.add_string buf sep;
        Buffer.add_string buf word;
        (len, " ")
      end
    ) (0, "") words
  in
  print_endline (Buffer.contents buf)
```
**Status:** [ ]

### 064: Strip whitespace from a string/Top and tail
**Source:** https://rosettacode.org/wiki/Strip_whitespace_from_a_string/Top_and_tail
**Topic:** Rosetta Code Strip whitespace from a string/Top and tail implementation in OCaml
**Difficulty:** Beginner
**Category:** Strings
**OCaml:**
```ocaml
let left_pos s len =
  let rec aux i =
    if i >= len then None
    else match s.[i] with
    | ' ' | '\n' | '\t' | '\r' -> aux (succ i)
    | _ -> Some i
  in
  aux 0

let right_pos s len =
  let rec aux i =
    if i < 0 then None
    else match s.[i] with
    | ' ' | '\n' | '\t' | '\r' -> aux (pred i)
    | _ -> Some i
  in
  aux (pred len)

let trim s =
  let len = String.length s in
  match left_pos s len, right_pos s len with
  | Some i, Some j -> String.sub s i (j - i + 1)
  | None, None -> ""
  | _ -> assert false

let ltrim s =
  let len = String.length s in
  match left_pos s len with
  | Some i -> String.sub s i (len - i)
  | None -> ""

let rtrim s =
  let len = String.length s in
  match right_pos s len with
  | Some i -> String.sub s 0 (i + 1)
  | None -> ""
```
**Status:** [ ]

### 065: Priority queue
**Source:** https://rosettacode.org/wiki/Priority_queue
**Topic:** Rosetta Code Priority queue implementation in OCaml
**Difficulty:** Intermediate
**Category:** Data Structures
**OCaml:**
```ocaml
module PQ = Base.PriorityQueue

let () =
  let tasks = [
    3, "Clear drains";
    4, "Feed cat";
    5, "Make tea";
    1, "Solve RC tasks";
    2, "Tax return";
  ] in
  let pq = PQ.make (fun (prio1, _) (prio2, _) -> prio1 > prio2) in
  List.iter (PQ.add pq) tasks;
  while not (PQ.is_empty pq) do
    let _, task = PQ.first pq in
    PQ.remove_first pq;
    print_endline task
  done
```
**Status:** [ ]

### 066: Doubly-linked list/Element definition
**Source:** https://rosettacode.org/wiki/Doubly-linked_list/Element_definition
**Topic:** Rosetta Code Doubly-linked list/Element definition implementation in OCaml
**Difficulty:** Intermediate
**Category:** Data Structures
**OCaml:**
```ocaml
type 'a dlink = {
  mutable data: 'a;
  mutable next: 'a dlink option;
  mutable prev: 'a dlink option;
}

let dlink_of_list li =
  let f prev_dlink x =
    let dlink = {
      data = x;
      prev = None;
      next = prev_dlink }
    in
    begin match prev_dlink with
    | None -> ()
    | Some prev_dlink ->
        prev_dlink.prev <- Some dlink
    end;
    Some dlink
  in
  List.fold_left f None (List.rev li)
;;

let list_of_dlink =
  let rec aux acc = function
  | None -> List.rev acc
  | Some{ data = d;
          prev = _;
          next = next } -> aux (d::acc) next
  in
  aux []
;;

let iter_forward_dlink f =
  let rec aux = function
  | None -> ()
  | Some{ data = d;
          prev = _;
          next = next } -> f d; aux next
  in
  aux
;;
```
**Status:** [ ]

### 067: Hash from two arrays
**Source:** https://rosettacode.org/wiki/Hash_from_two_arrays
**Topic:** Rosetta Code Hash from two arrays implementation in OCaml
**Difficulty:** Intermediate
**Category:** Data Structures
**OCaml:**
```ocaml
let keys = [ "foo"; "bar"; "baz" ]
and vals = [ 16384; 32768; 65536 ]
and hash = Hashtbl.create 0;;

List.iter2 (Hashtbl.add hash) keys vals;;
```
**Status:** [ ]

### 068: Accumulator factory
**Source:** https://rosettacode.org/wiki/Accumulator_factory
**Topic:** Rosetta Code Accumulator factory implementation in OCaml
**Difficulty:** Advanced
**Category:** Functional
**OCaml:**
```ocaml
let accumulator sum0 =
  let sum = ref sum0 in
  fun n ->
    sum := !sum +. n;
    !sum;;

let _ =
  let x = accumulator 1.0 in
  ignore (x 5.0);
  let _ = accumulator 3.0 in
  Printf.printf "%g\n" (x 2.3)
;;
```
**Status:** [ ]

### 069: Closures/Value capture
**Source:** https://rosettacode.org/wiki/Closures/Value_capture
**Topic:** Rosetta Code Closures/Value capture implementation in OCaml
**Difficulty:** Advanced
**Category:** Functional
**OCaml:**
```ocaml
let () =
  let cls = Array.init 10 (fun i -> (function () -> i * i)) in
  Random.self_init ();
  for i = 1 to 6 do
    let x = Random.int 9 in
    Printf.printf " fun.(%d) = %d\n" x (cls.(x) ());
  done
```
**Status:** [ ]

### 070: Higher-order functions
**Source:** https://rosettacode.org/wiki/Higher-order_functions
**Topic:** Rosetta Code Higher-order functions implementation in OCaml
**Difficulty:** Advanced
**Category:** Functional
**OCaml:**
```ocaml
# let func1 f = f "a string";;
val func1 : (string -> 'a) -> 'a = <fun>
# let func2 s = "func2 called with " ^ s;;
val func2 : string -> string = <fun>

# print_endline (func1 func2);;
func2 called with a string
- : unit = ()
```
**Status:** [ ]

### 071: Anonymous recursion
**Source:** https://rosettacode.org/wiki/Anonymous_recursion
**Topic:** Rosetta Code Anonymous recursion implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let fib n =
  let rec real = function
      0 -> 1
    | 1 -> 1
    | n -> real (n-1) + real (n-2)
  in
  if n < 0 then
    None
  else
    Some (real n)
```
**Status:** [ ]

### 072: Partial function application
**Source:** https://rosettacode.org/wiki/Partial_function_application
**Topic:** Rosetta Code Partial function application implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
# 
let fs f s = List.map f s
let f1 value = value * 2
let f2 value = value * value

let fsf1 = fs f1
let fsf2 = fs f2
;;
val fs : ('a -> 'b) -> 'a list -> 'b list = <fun>
val f1 : int -> int = <fun>
val f2 : int -> int = <fun>
val fsf1 : int list -> int list = <fun>
val fsf2 : int list -> int list = <fun>

# fsf1 [0; 1; 2; 3];;
- : int list = [0; 2; 4; 6]
# fsf2 [0; 1; 2; 3];;
- : int list = [0; 1; 4; 9]
# fsf1 [2; 4; 6; 8];;
- : int list = [4; 8; 12; 16]
# fsf2 [2; 4; 6; 8];;
- : int list = [4; 16; 36; 64]
```
**Status:** [ ]

### 073: Man or boy test
**Source:** https://rosettacode.org/wiki/Man_or_boy_test
**Topic:** Rosetta Code Man or boy test implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let rec a k x1 x2 x3 x4 x5 =
  if k <= 0 then
    x4 () + x5 ()
  else
    let m = ref k in
    let rec b () =
      decr m;
      a !m b x1 x2 x3 x4
    in
    b ()

let () =
  Printf.printf "%d\n" (a 10 (fun () -> 1) (fun () -> -1) (fun () -> -1) (fun () -> 1) (fun () -> 0))
```
**Status:** [ ]

### 074: Remove duplicate elements
**Source:** https://rosettacode.org/wiki/Remove_duplicate_elements
**Topic:** Rosetta Code Remove duplicate elements implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let uniq lst =
  let unique_set = Hashtbl.create (List.length lst) in
  List.iter (fun x -> Hashtbl.replace unique_set x ()) lst;
  Hashtbl.fold (fun x () xs -> x :: xs) unique_set []

let _ =
  uniq [1;2;3;2;3;4]
```
**Status:** [ ]

### 075: Sequence of non-squares
**Source:** https://rosettacode.org/wiki/Sequence_of_non-squares
**Topic:** Rosetta Code Sequence of non-squares implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
# let nonsqr n = n + truncate (0.5 +. sqrt (float n));;
val nonsqr : int -> int = <fun>
# (* first 22 values (as a list) has no squares: *)
  for i = 1 to 22 do
    Printf.printf "%d " (nonsqr i)
  done;
  print_newline ();;
2 3 5 6 7 8 10 11 12 13 14 15 17 18 19 20 21 22 23 24 26 27
- : unit = ()
# (* The following check shows no squares up to one million: *)
  for i = 1 to 1_000_000 do
    let j = sqrt (float (nonsqr i)) in
      assert (j <> floor j)
  done;;
- : unit = ()
```
**Status:** [ ]

### 076: Range extraction
**Source:** https://rosettacode.org/wiki/Range_extraction
**Topic:** Rosetta Code Range extraction implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let range_extract = function
  | [] -> []
  | x::xs ->
    let f (i,j,ret) k =
      if k = succ j then (i,k,ret) else (k,k,(i,j)::ret) in
    let (m,n,ret) = List.fold_left f (x,x,[]) xs in
    List.rev ((m,n)::ret)

let string_of_range rng =
  let str (a,b) =
    if a = b then string_of_int a
    else Printf.sprintf "%d%c%d" a (if b = succ a then ',' else '-') b in
  String.concat "," (List.map str rng)

let () =
  let li =
    [ 0; 1; 2; 4; 6; 7; 8; 11; 12; 14; 15; 16; 17; 18; 19; 20; 21;
      22; 23; 24; 25; 27; 28; 29; 30; 31; 32; 33; 35; 36; 37; 38; 39 ]
  in
  let rng = range_extract li in
  print_endline(string_of_range rng)
```
**Status:** [ ]

### 077: Range expansion
**Source:** https://rosettacode.org/wiki/Range_expansion
**Topic:** Rosetta Code Range expansion implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
#load "str.cma"

let range a b =
  if b < a then invalid_arg "range";
  let rec aux i acc =
    if i = b then List.rev (i::acc)
    else aux (succ i) (i::acc)
  in
  aux a []

let parse_piece s =
  try Scanf.sscanf s "%d-%d" (fun a b -> range a b)
  with _ -> [int_of_string s]

let range_expand rng =
  let ps = Str.split (Str.regexp_string ",") rng in
  List.flatten (List.map parse_piece ps)

let () =
  let rng = "-6,-3--1,3-5,7-11,14,15,17-20" in
  let exp = range_expand rng in
  List.iter (Printf.printf " %d") exp;
  print_newline ()
```
**Status:** [ ]

### 078: Tree traversal
**Source:** https://rosettacode.org/wiki/Tree_traversal
**Topic:** Rosetta Code Tree traversal implementation in OCaml
**Difficulty:** Intermediate
**Category:** Data Structures
**OCaml:**
```ocaml
type 'a tree = Empty
             | Node of 'a * 'a tree * 'a tree

let rec preorder f = function
    Empty        -> ()
  | Node (v,l,r) -> f v;
                    preorder f l;
                    preorder f r

let rec inorder f = function
    Empty        -> ()
  | Node (v,l,r) -> inorder f l;
                    f v;
                    inorder f r

let rec postorder f = function
    Empty        -> ()
  | Node (v,l,r) -> postorder f l;
                    postorder f r;
                    f v

let levelorder f x =
  let queue = Queue.create () in
    Queue.add x queue;
    while not (Queue.is_empty queue) do
      match Queue.take queue with
          Empty        -> ()
        | Node (v,l,r) -> f v;
                          Queue.add l queue;
                          Queue.add r queue
    done

let tree =
  Node (1,
        Node (2,
              Node (4,
                    Node (7, Empty, Empty),
                    Empty),
              Node (5, Empty, Empty)),
        Node (3,
              Node (6,
                    Node (8, Empty, Empty),
                    Node (9, Empty, Empty)),
              Empty))

let () =
  preorder   (Printf.printf "%d ") tree; print_newline ();
  inorder    (Printf.printf "%d ") tree; print_newline ();
  postorder  (Printf.printf "%d ") tree; print_newline ();
  levelorder (Printf.printf "%d ") tree; print_newline ()
```
**Status:** [ ]

### 079: Algebraic data types
**Source:** https://rosettacode.org/wiki/Algebraic_data_types
**Topic:** Rosetta Code Algebraic data types implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
type color = R | B
type 'a tree = E | T of color * 'a tree * 'a * 'a tree

(** val balance : color * 'a tree * 'a * 'a tree -> 'a tree *)
let balance = function
  | B, T (R, T (R,a,x,b), y, c), z, d
  | B, T (R, a, x, T (R,b,y,c)), z, d
  | B, a, x, T (R, T (R,b,y,c), z, d)
  | B, a, x, T (R, b, y, T (R,c,z,d)) -> T (R, T (B,a,x,b), y, T (B,c,z,d))
  | col, a, x, b                      -> T (col, a, x, b) 

(** val insert : 'a -> 'a tree -> 'a tree *)
let insert x s = 
  let rec ins = function
    | E                  -> T (R,E,x,E)
    | T (col,a,y,b) as s ->
	if x < y then
	  balance (col, ins a, y, b)
	else if x > y then
	  balance (col, a, y, ins b)
	else
	  s
  in let T (_,a,y,b) = ins s 
  in T (B,a,y,b)
```
**Status:** [ ]

### 080: Compound data type
**Source:** https://rosettacode.org/wiki/Compound_data_type
**Topic:** Rosetta Code Compound data type implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
type tree = Empty
          | Leaf of int
          | Node of tree * tree

let t1 = Node (Leaf 1, Node (Leaf 2, Leaf 3))
```
**Status:** [ ]

### 081: Abstract type
**Source:** https://rosettacode.org/wiki/Abstract_type
**Topic:** Rosetta Code Abstract type implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
class virtual foo =
  object
    method virtual bar : int
  end
```
**Status:** [ ]

### 082: Define a primitive data type
**Source:** https://rosettacode.org/wiki/Define_a_primitive_data_type
**Topic:** Rosetta Code Define a primitive data type implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
exception Out_of_bounds

type 'a bounds = { min: 'a; max: 'a }

type 'a bounded = { value: 'a; bounds: 'a bounds }

let mk_bounds ~min ~max = { min=min; max=max } ;;
(** val mk_bounds : min:'a -> max:'a -> 'a bounds *)

let check_bounds ~value ~bounds =
  if value < bounds.min || value > bounds.max then
    raise Out_of_bounds ;;
(** val check_bounds : value:'a -> bounds:'a bounds -> unit *)

let mk_bounded ~value ~bounds =
  check_bounds ~value ~bounds;
  { value=value; bounds=bounds } ;;
(** val mk_bounded : value:'a -> bounds:'a bounds -> 'a bounded *)

let op f a b =
  if a.bounds <> b.bounds then
    invalid_arg "different bounds";
  let res = f a.value b.value in
  check_bounds res a.bounds;
  (mk_bounded res a.bounds)
  ;;            
(** val op : ('a -> 'a -> 'a) -> 'a bounded -> 'a bounded -> 'a bounded *)
```
**Status:** [ ]

### 083: N-queens problem
**Source:** https://rosettacode.org/wiki/N-queens_problem
**Topic:** Rosetta Code N-queens problem implementation in OCaml
**Difficulty:** Advanced
**Category:** Algorithms
**OCaml:**
```ocaml
(* Authors: Nicolas Barnier, Pascal Brisset
   Copyright 2004 CENA. All rights reserved.
   This code is distributed under the terms of the GNU LGPL *)

open Facile
open Easy

(* Print a solution *)
let print queens =
  let n = Array.length queens in
  if n <= 10 then (* Pretty printing *)
    for i = 0 to n - 1 do
      let c = Fd.int_value queens.(i) in (* queens.(i) is bound *)
      for j = 0 to n - 1 do
        Printf.printf "%c " (if j = c then '*' else '-')
      done;
      print_newline ()
    done
  else (* Short print *)
    for i = 0 to n-1 do
      Printf.printf "line %d : col %a\n" i Fd.fprint queens.(i)
    done;
  flush stdout;
;;

(* Solve the n-queens problem *)
let queens n =
  (* n decision variables in 0..n-1 *)
  let queens = Fd.array n 0 (n-1) in

  (* 2n auxiliary variables for diagonals *)
  let shift op = Array.mapi (fun i qi -> Arith.e2fd (op (fd2e qi) (i2e i))) queens in
  let diag1 = shift (+~) and diag2 = shift (-~) in

  (* Global constraints *)
  Cstr.post (Alldiff.cstr queens);
  Cstr.post (Alldiff.cstr diag1);
  Cstr.post (Alldiff.cstr diag2);

  (* Heuristic Min Size, Min Value *)
  let h a = (Var.Attr.size a, Var.Attr.min a) in
  let min_min = Goals.Array.choose_index (fun a1 a2 -> h a1 < h a2) in

  (* Search goal *)
  let labeling = Goals.Array.forall ~select:min_min Goals.indomain in

  (* Solve *)
  let bt = ref 0 in
  if Goals.solve ~control:(fun b -> bt := b) (labeling queens) then begin
    Printf.printf "%d backtracks\n" !bt;
    print queens
  end else
    prerr_endline "No solution"

let _ =
  if Array.length Sys.argv <> 2
  then raise (Failure "Usage: queens <nb of queens>");
  Gc.set ({(Gc.get ()) with Gc.space_overhead = 500}); (* May help except with an underRAMed system *)
  queens (int_of_string Sys.argv.(1));;
```
**Status:** [ ]

### 084: Maze generation
**Source:** https://rosettacode.org/wiki/Maze_generation
**Topic:** Rosetta Code Maze generation implementation in OCaml
**Difficulty:** Advanced
**Category:** Algorithms
**OCaml:**
```ocaml
let seen = Hashtbl.create 7
let mark t = Hashtbl.add seen t true
let marked t = Hashtbl.mem seen t

let walls = Hashtbl.create 7
let ord a b = if a <= b then (a,b) else (b,a)
let join a b = Hashtbl.add walls (ord a b) true
let joined a b = Hashtbl.mem walls (ord a b)

let () =
  let nx = int_of_string Sys.argv.(1) in
  let ny = int_of_string Sys.argv.(2) in

  let shuffle lst =
     let nl = List.map (fun c -> (Random.bits (), c)) lst in
     List.map snd (List.sort compare nl) in

  let get_neighbours (x,y) =
    let lim n k = (0 <= k) && (k < n) in
    let bounds (x,y) = lim nx x && lim ny y in
    List.filter bounds [(x-1,y);(x+1,y);(x,y-1);(x,y+1)] in

  let rec visit cell =
    mark cell;
    let check k =
      if not (marked k) then (join cell k; visit k) in
    List.iter check (shuffle (get_neighbours cell)) in

  let print_maze () =
    begin
    for i = 1 to nx do print_string "+---";done; print_endline "+";
    let line n j k l s t u =
      for i = 0 to n do print_string (if joined (i,j) (i+k,j+l) then s else t) done;
      print_endline u in
    for j = 0 to ny-2 do
      print_string "|   ";
      line (nx-2) j 1 0 "    " "|   " "|";
      line (nx-1) j 0 1 "+   " "+---" "+";
    done;
    print_string "|   ";
    line (nx-2) (ny-1) 1 0 "    " "|   " "|";
    for i = 1 to nx do print_string "+---";done; print_endline "+";
   end in

  Random.self_init();
  visit (Random.int nx, Random.int ny);
  print_maze ();
```
**Status:** [ ]

### 085: Conway's Game of Life
**Source:** https://rosettacode.org/wiki/Conway's_Game_of_Life
**Topic:** Rosetta Code Conway's Game of Life implementation in OCaml
**Difficulty:** Advanced
**Category:** Algorithms
**OCaml:**
```ocaml
let get g x y =
  try g.(x).(y)
  with _ -> 0

let neighbourhood g x y =
  (get g (x-1) (y-1)) +
  (get g (x-1) (y  )) +
  (get g (x-1) (y+1)) +
  (get g (x  ) (y-1)) +
  (get g (x  ) (y+1)) +
  (get g (x+1) (y-1)) +
  (get g (x+1) (y  )) +
  (get g (x+1) (y+1)) 

let next_cell g x y =
  let n = neighbourhood g x y in
  match g.(x).(y), n with
  | 1, 0 | 1, 1                      -> 0  (* lonely *)
  | 1, 4 | 1, 5 | 1, 6 | 1, 7 | 1, 8 -> 0  (* overcrowded *)
  | 1, 2 | 1, 3                      -> 1  (* lives *)
  | 0, 3                             -> 1  (* get birth *)
  | _ (* 0, (0|1|2|4|5|6|7|8) *)     -> 0  (* barren *)

let copy g = Array.map Array.copy g

let next g =
  let width = Array.length g
  and height = Array.length g.(0)
  and new_g = copy g in
  for x = 0 to pred width do
    for y = 0 to pred height do
      new_g.(x).(y) <- (next_cell g x y)
    done
  done;
  (new_g)

let print g =
  let width = Array.length g
  and height = Array.length g.(0) in
  for x = 0 to pred width do
    for y = 0 to pred height do
      if g.(x).(y) = 0
      then print_char '.'
      else print_char 'o'
    done;
    print_newline ()
  done
```
**Status:** [ ]

### 086: Langton's ant
**Source:** https://rosettacode.org/wiki/Langton's_ant
**Topic:** Rosetta Code Langton's ant implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
open Graphics

type dir = North | East | South | West

let turn_left = function
  | North -> West
  | East  -> North
  | South -> East
  | West  -> South

let turn_right = function
  | North -> East
  | East  -> South
  | South -> West
  | West  -> North

let move (x, y) = function
  | North -> x, y + 1
  | East  -> x + 1, y
  | South -> x, y - 1
  | West  -> x - 1, y

let () =
  open_graph "";
  let rec loop (x, y as pos) dir =
    let color = point_color x y in
    set_color (if color = white then black else white);
    plot x y;
    let dir = (if color = white then turn_right else turn_left) dir in
    if not(key_pressed()) then loop (move pos dir) dir
  in
  loop (size_x()/2, size_y()/2) North
```
**Status:** [ ]

### 087: 100 doors
**Source:** https://rosettacode.org/wiki/100_doors
**Topic:** Rosetta Code 100 doors implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let max_doors = 100

let show_doors =
  Array.iteri (fun i x -> Printf.printf "Door %d is %s\n" (i+1)
                                        (if x then "open" else "closed"))

let flip_doors doors =
  for i = 1 to max_doors do
    let rec flip idx =
      if idx < max_doors then begin
        doors.(idx) <- not doors.(idx);
        flip (idx + i)
      end
    in flip (i - 1)
  done;
  doors

let () =
  show_doors (flip_doors (Array.make max_doors false))
```
**Status:** [ ]

### 088: FizzBuzz
**Source:** https://rosettacode.org/wiki/FizzBuzz
**Topic:** Rosetta Code FizzBuzz implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let fizzbuzz i =
  match i mod 3, i mod 5 with
    0, 0 -> "FizzBuzz"
  | 0, _ -> "Fizz"
  | _, 0 -> "Buzz"
  | _    -> string_of_int i
 
let _ =
  for i = 1 to 100 do print_endline (fizzbuzz i) done
```
**Status:** [ ]

### 089: Emirp primes
**Source:** https://rosettacode.org/wiki/Emirp_primes
**Topic:** Rosetta Code Emirp primes implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let int_reverse =
  let rec loop m n =
    if n < 10 then m + n else loop ((m + n mod 10) * 10) (n / 10)
  in loop 0

let is_prime n =
  let not_divisible x = n mod x <> 0 in
  seq_primes |> Seq.take_while (fun x -> x * x <= n) |> Seq.for_all not_divisible

let seq_emirps =
  let is_emirp n = let m = int_reverse n in m <> n && is_prime m in
  seq_primes |> Seq.filter is_emirp

let () =
  let seq_show sq = print_newline (Seq.iter (Printf.printf " %u") sq) in
  seq_emirps |> Seq.take 20 |> seq_show;
  seq_emirps |> Seq.drop_while ((>) 7700) |> Seq.take_while ((>) 8000) |> seq_show;
  seq_emirps |> Seq.drop 9999 |> Seq.take 1 |> seq_show
```
**Status:** [ ]

### 090: Additive primes
**Source:** https://rosettacode.org/wiki/Additive_primes
**Topic:** Rosetta Code Additive primes implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let rec digit_sum n =
  if n < 10 then n else n mod 10 + digit_sum (n / 10)

let is_prime n =
  let rec test x =
    let q = n / x in x > q || x * q <> n && n mod (x + 2) <> 0 && test (x + 6)
  in if n < 5 then n lor 1 = 3 else n land 1 <> 0 && n mod 3 <> 0 && test 5

let is_additive_prime n =
  is_prime n && is_prime (digit_sum n)

let () =
  Seq.ints 0 |> Seq.take_while ((>) 500) |> Seq.filter is_additive_prime
  |> Seq.iter (Printf.printf " %u") |> print_newline
```
**Status:** [ ]

### 091: Balanced brackets
**Source:** https://rosettacode.org/wiki/Balanced_brackets
**Topic:** Rosetta Code Balanced brackets implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let generate_brackets n =
  let rec aux i acc =
    if i <= 0 then acc else
      aux (pred i) ('['::']'::acc)
  in
  let brk = aux n [] in
  List.sort (fun _ _ -> (Random.int 3) - 1) brk 

let is_balanced brk =
  let rec aux = function
    | [], 0 -> true
    | '['::brk, level -> aux (brk, succ level)
    | ']'::brk, 0 -> false
    | ']'::brk, level -> aux (brk, pred level)
    | _ -> assert false
  in
  aux (brk, 0)

let () =
  let n = int_of_string Sys.argv.(1) in
  Random.self_init();
  let brk = generate_brackets n in
  List.iter print_char brk;
  Printf.printf " %B\n" (is_balanced brk);
;;
```
**Status:** [ ]

### 092: Balanced ternary
**Source:** https://rosettacode.org/wiki/Balanced_ternary
**Topic:** Rosetta Code Balanced ternary implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
type btdigit = Pos | Zero | Neg
type btern = btdigit list

let to_string n =
   String.concat ""
      (List.rev_map (function Pos -> "+" | Zero -> "0" | Neg -> "-") n)

let from_string s =
   let sl = ref [] in
   let digit = function '+' -> Pos | '-' -> Neg | '0' -> Zero
     | _ -> failwith "invalid digit" in
    String.iter (fun c -> sl := (digit c) :: !sl) s; !sl

let rec to_int = function
   | [Zero] | [] -> 0
   | Pos :: t -> 1 + 3 * to_int t
   | Neg :: t -> -1 + 3 * to_int t
   | Zero :: t -> 3 * to_int t

let rec from_int n =
   if n = 0 then [] else
   match n mod 3 with
      | 0 -> Zero :: from_int (n/3)
      | 1 | -2 -> Pos :: from_int ((n-1)/3)
      | 2 | -1 -> Neg :: from_int ((n+1)/3)

let rec (+~) n1 n2 = match (n1,n2) with
   | ([], a) | (a,[]) -> a
   | (Pos::t1, Neg::t2) | (Neg::t1, Pos::t2) | (Zero::t1, Zero::t2) ->
      let sum = t1 +~ t2 in if sum = [] then [] else Zero :: sum
   | (Pos::t1, Pos::t2) -> Neg :: t1 +~ t2 +~ [Pos]
   | (Neg::t1, Neg::t2) -> Pos :: t1 +~ t2 +~ [Neg]
   | (Zero::t1, h::t2) | (h::t1, Zero::t2) -> h :: t1 +~ t2

let neg = List.map (function Pos -> Neg | Neg -> Pos | Zero -> Zero)
let (-~) a b = a +~ (neg b)

let rec ( *~) n1 = function
   | [] -> []
   | [Pos] -> n1
   | [Neg] -> neg n1
   | Pos::t -> (Zero :: t *~ n1) +~ n1
   | Neg::t -> (Zero :: t *~ n1) -~ n1
   | Zero::t -> Zero :: t *~ n1

let a = from_string "+-0++0+"
let b = from_int (-436)
let c = from_string "+-++-"
let d = a *~ (b -~ c)
let _ =
  Printf.printf "a = %d\nb = %d\nc = %d\na * (b - c) = %s = %d\n"
   (to_int a) (to_int b) (to_int c) (to_string d) (to_int d);
```
**Status:** [ ]

### 093: Amb
**Source:** https://rosettacode.org/wiki/Amb
**Topic:** Rosetta Code Amb implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let set_1 = ["the"; "that"; "a"]
let set_2 = ["frog"; "elephant"; "thing"]
let set_3 = ["walked"; "treaded"; "grows"]
let set_4 = ["slowly"; "quickly"]

let combs ll =
  let rec aux acc = function
  | [] -> (List.map List.rev acc)
  | hd::tl ->
      let acc =
        List.fold_left
          (fun _ac l ->
            List.fold_left (fun _ac v -> (v::l)::_ac) _ac hd
          ) [] acc
      in
      aux acc tl
  in
  aux [[]] ll

let last s = s.[pred(String.length s)]
let joined a b = (last a = b.[0])

let rec test = function
  | a::b::tl -> (joined a b) && (test (b::tl))
  | _ -> true

let print_set set =
  List.iter (Printf.printf " %s") set;
  print_newline();
;;

let () =
  let sets = combs [set_1; set_2; set_3; set_4] in
  let sets = List.filter test sets in
  List.iter print_set sets;
;;
```
**Status:** [ ]

### 094: Hofstadter Q sequence
**Source:** https://rosettacode.org/wiki/Hofstadter_Q_sequence
**Topic:** Rosetta Code Hofstadter Q sequence implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
(* valid results for n in 0..119628 *)
let seq_hofstadter_q n =
  let a = Bigarray.(Array1.create int16_unsigned c_layout n) in
  let () =
    for i = 0 to pred n do
      a.{i} <- if i < 2 then 1 else a.{i - a.{pred i}} + a.{i - a.{i - 2}}
    done
  in
  Seq.init n (Bigarray.Array1.get a)

let () =
  let count_backflip (a, c) b = b, if b < a then succ c else c
  and hq = seq_hofstadter_q 100_000 in
  let () = Seq.(hq |> take 10 |> iter (Printf.printf " %u")) in
  let () = Seq.(hq |> drop 999 |> take 1 |> iter (Printf.printf "\n%u\n")) in
  hq |> Seq.fold_left count_backflip (0, 0) |> snd |> Printf.printf "%u\n"
```
**Status:** [ ]

### 095: Look-and-say sequence
**Source:** https://rosettacode.org/wiki/Look-and-say_sequence
**Topic:** Rosetta Code Look-and-say sequence implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let rec seeAndSay = function
  | [], nys -> List.rev nys
  | x::xs, [] -> seeAndSay(xs, [x; 1])
  | x::xs, y::n::nys when x=y -> seeAndSay(xs, y::1+n::nys)
  | x::xs, nys -> seeAndSay(xs, x::1::nys)
```
**Status:** [ ]

### 096: Continued fraction
**Source:** https://rosettacode.org/wiki/Continued_fraction
**Topic:** Rosetta Code Continued fraction implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let pi = 3, fun n -> ((2*n-1)*(2*n-1), 6)
and nap = 2, fun n -> (max 1 (n-1), n)
and root2 = 1, fun n -> (1, 2) in

let eval (i,f) k =
  let rec frac n =
    let a, b = f n in
    float a /. (float b +.
      if n >= k then 0.0 else frac (n+1)) in
  float i +. frac 1 in

Printf.printf "sqrt(2)\t= %.15f\n" (eval root2 1000);
Printf.printf "e\t= %.15f\n" (eval nap 1000);
Printf.printf "pi\t= %.15f\n" (eval pi 1000);
```
**Status:** [ ]

### 097: Jensen's Device
**Source:** https://rosettacode.org/wiki/Jensen's_Device
**Topic:** Rosetta Code Jensen's Device implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let i = ref 42 (* initial value doesn't matter *)

let sum' i lo hi term =
  let result = ref 0. in
    i := lo;
    while !i <= hi do
      result := !result +. term ();
      incr i
    done;
    !result

let () =
  Printf.printf "%f\n" (sum' i 1 100 (fun () -> 1. /. float !i))
```
**Status:** [ ]

### 098: Entropy
**Source:** https://rosettacode.org/wiki/Entropy
**Topic:** Rosetta Code Entropy implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
module CharMap = Map.Make(Char)

let entropy s =
  let count map c =
    CharMap.update c (function Some n -> Some (n +. 1.) | None -> Some 1.) map
  and calc _ n sum =
    sum +. n *. Float.log2 n
  in
  let sum = CharMap.fold calc (String.fold_left count CharMap.empty s) 0.
  and len = float (String.length s) in
  Float.log2 len -. sum /. len

let () =
  entropy "1223334444" |> string_of_float |> print_endline
```
**Status:** [ ]

### 099: Monte Carlo methods
**Source:** https://rosettacode.org/wiki/Monte_Carlo_methods
**Topic:** Rosetta Code Monte Carlo methods implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let get_pi throws =
  let rec helper i count =
    if i = throws then count
    else
      let rand_x = Random.float 2.0 -. 1.0
      and rand_y = Random.float 2.0 -. 1.0 in
      let dist = sqrt (rand_x *. rand_x +. rand_y *. rand_y) in
      if dist < 1.0 then
        helper (i+1) (count+1)
      else
        helper (i+1) count
  in float (4 * helper 0 0) /. float throws
```
**Status:** [ ]

### 100: Benford's law
**Source:** https://rosettacode.org/wiki/Benford's_law
**Topic:** Rosetta Code Benford's law implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
open Num

let fib =
  let rec fib_aux f0 f1 = function
    | 0 -> f0
    | 1 -> f1
    | n -> fib_aux f1 (f1 +/ f0) (n - 1)
  in
  fib_aux (num_of_int 0) (num_of_int 1) ;;

let create_fibo_string = function n -> string_of_num (fib n) ;;
let rec range i j = if i > j then [] else i :: (range (i + 1) j)

let n_max = 1000 ;;

let numbers = range 1 n_max in
  let get_first_digit = function s -> Char.escaped (String.get s 0) in
    let first_digits = List.map get_first_digit (List.map create_fibo_string numbers) in
  let data = Array.create 9 0 in
    let fill_data vec = function n -> vec.(n - 1) <- vec.(n - 1) + 1 in
    List.iter (fill_data data) (List.map int_of_string first_digits) ;
    Printf.printf "\nFrequency of the first digits in the Fibonacci sequence:\n" ;
    Array.iter (Printf.printf "%f ")
      (Array.map (fun x -> (float x) /. float (n_max)) data) ;

let xvalues = range 1 9 in
  let benfords_law = function x -> log10 (1.0 +. 1.0 /. float (x)) in
    Printf.printf "\nPrediction of Benford's law:\n " ;
    List.iter (Printf.printf "%f ") (List.map benfords_law xvalues) ;
    Printf.printf "\n" ;;
```
**Status:** [ ]

### 101: Hamming numbers
**Source:** https://rosettacode.org/wiki/Hamming_numbers
**Topic:** Rosetta Code Hamming numbers implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
module ISet = Set.Make(struct type t = int let compare=compare end)

let pq = ref (ISet.singleton 1)

let next () =
  let m = ISet.min_elt !pq in
  pq := ISet.(remove m !pq  |> add (2*m) |> add (3*m) |> add (5*m));
  m

let () =

  print_string "The first 20 are: ";

  for i = 1 to 20
  do
    Printf.printf "%d " (next ())
  done;

  for i = 21 to 1690
  do
    ignore (next ())
  done;

  Printf.printf "\nThe 1691st is %d\n" (next ());
```
**Status:** [ ]

### 102: Regular expressions
**Source:** https://rosettacode.org/wiki/Regular_expressions
**Topic:** Rosetta Code Regular expressions implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
#load "str.cma";;
let str = "I am a string";;
try
  ignore(Str.search_forward (Str.regexp ".*string$") str 0);
  print_endline "ends with 'string'"
with Not_found -> ()
;;
```
**Status:** [ ]

### 103: 99 bottles of beer
**Source:** https://rosettacode.org/wiki/99_bottles_of_beer
**Topic:** Rosetta Code 99 bottles of beer implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
for n = 99 downto 1 do
  Printf.printf "%d bottles of beer on the wall\n" n;
  Printf.printf "%d bottles of beer\n" n;
  Printf.printf "Take one down, pass it around\n";
  Printf.printf "%d bottles of beer on the wall\n\n" (pred n);
done
```
**Status:** [ ]

### 104: Guess the number
**Source:** https://rosettacode.org/wiki/Guess_the_number
**Topic:** Rosetta Code Guess the number implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
#!/usr/bin/env ocaml

let () =
  Random.self_init();
  let n =
    if Random.bool () then
      let n = 2 + Random.int 8 in
      print_endline "Please guess a number between 1 and 10 excluded";
      (n)
    else
      let n = 1 + Random.int 10 in
      print_endline "Please guess a number between 1 and 10 included";
      (n)
  in
  while read_int () <> n do
    print_endline "The guess was wrong! Please try again!"
  done;
  print_endline "Well guessed!"
```
**Status:** [ ]

### 105: Bulls and cows
**Source:** https://rosettacode.org/wiki/Bulls_and_cows
**Topic:** Rosetta Code Bulls and cows implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let rec input () =
  let s = read_line () in
  try
    if String.length s <> 4 then raise Exit;
    String.iter (function
      | '1'..'9' -> ()
      | _ -> raise Exit
    ) s;
    let t = [ s.[0]; s.[1]; s.[2]; s.[3] ] in
    let _ = List.fold_left  (* reject entry with duplication *)
              (fun ac b -> if List.mem b ac then raise Exit; (b::ac))
              [] t in
    List.map (fun c -> int_of_string (String.make 1 c)) t
  with Exit ->
    prerr_endline "That is an invalid entry. Please try again.";
    input ()
;;

let print_score g t =
  let bull = ref 0 in
  List.iter2 (fun x y ->
    if x = y then incr bull
  ) g t;
  let cow = ref 0 in
  List.iter (fun x ->
    if List.mem x t then incr cow
  ) g;
  cow := !cow - !bull;
  Printf.printf "%d bulls, %d cows\n%!" !bull !cow
;;

let () =
  Random.self_init ();
  let rec mkgoal acc = function 4 -> acc
  | i ->
      let n = succ(Random.int 9) in
      if List.mem n acc
      then mkgoal acc i
      else mkgoal (n::acc) (succ i)
  in
  let g = mkgoal [] 0 in
  let found = ref false in
  while not !found do
    let t = input () in
    if t = g
    then found := true
    else print_score g t
  done;
  print_endline "Congratulations you guessed correctly";
;;
```
**Status:** [ ]

### 106: Anti-primes
**Source:** https://rosettacode.org/wiki/Anti-primes
**Topic:** Rosetta Code Anti-primes implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let num_divisors (n : int) : int =
  if n = 0 || n = 1 then 1 else if n = 2 then 2 else
  List.init (n / 2) ((+) 1) (* O(n) *)
  |> List.filter (fun i -> n mod i = 0) 
  |> List.length
    
let first_n_antiprimes (n : int) : int list =
  let rec loop = function
    | i, record, antis when List.length antis = n -> antis
    | i, record, antis -> let nd = num_divisors i in
      if nd > record then loop (i + 1, nd, i :: antis) else
      loop (i + 1, record, antis)
  in loop (2, 1, [1]) |> List.rev
  
let () = first_n_antiprimes 19
  |> List.map string_of_int
  |> String.concat ", "
  |> Printf.printf "[%s]\n"
```
**Status:** [ ]

### 107: Arithmetic evaluation
**Source:** https://rosettacode.org/wiki/Arithmetic_evaluation
**Topic:** Rosetta Code Arithmetic evaluation implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
type expression =
  | Const of float
  | Sum  of expression * expression   (* e1 + e2 *)
  | Diff of expression * expression   (* e1 - e2 *)
  | Prod of expression * expression   (* e1 * e2 *)
  | Quot of expression * expression   (* e1 / e2 *)

let rec eval = function
  | Const c -> c
  | Sum (f, g) -> eval f +. eval g
  | Diff(f, g) -> eval f -. eval g
  | Prod(f, g) -> eval f *. eval g
  | Quot(f, g) -> eval f /. eval g

open Genlex

let lexer = make_lexer ["("; ")"; "+"; "-"; "*"; "/"]

let rec parse_expr = parser
     [< e1 = parse_mult; e = parse_more_adds e1 >] -> e
 and parse_more_adds e1 = parser
     [< 'Kwd "+"; e2 = parse_mult; e = parse_more_adds (Sum(e1, e2)) >] -> e
   | [< 'Kwd "-"; e2 = parse_mult; e = parse_more_adds (Diff(e1, e2)) >] -> e
   | [< >] -> e1
 and parse_mult = parser
     [< e1 = parse_simple; e = parse_more_mults e1 >] -> e
 and parse_more_mults e1 = parser
     [< 'Kwd "*"; e2 = parse_simple; e = parse_more_mults (Prod(e1, e2)) >] -> e
   | [< 'Kwd "/"; e2 = parse_simple; e = parse_more_mults (Quot(e1, e2)) >] -> e
   | [< >] -> e1
 and parse_simple = parser
   | [< 'Int i >] -> Const(float i)
   | [< 'Float f >] -> Const f
   | [< 'Kwd "("; e = parse_expr; 'Kwd ")" >] -> e


let parse_expression = parser [< e = parse_expr; _ = Stream.empty >] -> e

let read_expression s = parse_expression(lexer(Stream.of_string s))
```
**Status:** [ ]

### 108: Arithmetic-geometric mean
**Source:** https://rosettacode.org/wiki/Arithmetic-geometric_mean
**Topic:** Rosetta Code Arithmetic-geometric mean implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let rec agm a g tol =
  if tol > abs_float (a -. g) then a else
  agm (0.5*.(a+.g)) (sqrt (a*.g)) tol

let _ = Printf.printf "%.16f\n" (agm 1.0 (sqrt 0.5) 1e-15)
```
**Status:** [ ]

### 109: Arithmetic-geometric mean/Calculate Pi
**Source:** https://rosettacode.org/wiki/Arithmetic-geometric_mean/Calculate_Pi
**Topic:** Rosetta Code Arithmetic-geometric mean/Calculate Pi implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let limit = 10000 and n = 2800
let x = Array.make (n+1) 2000

let rec g j sum =
  if j < 1 then sum else
    let sum = sum * j + limit * x.(j) in
    x.(j) <- sum mod (j * 2 - 1);
    g (j - 1) (sum / (j * 2 - 1))

let rec f i carry =
  if i = 0 then () else
    let sum = g i 0 in
    Printf.printf "%04d" (carry + sum / limit);
    f (i - 14) (sum mod limit)

let () =
  f n 0;
  print_newline()
```
**Status:** [ ]

### 110: Arithmetic/Complex
**Source:** https://rosettacode.org/wiki/Arithmetic/Complex
**Topic:** Rosetta Code Arithmetic/Complex implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
open Complex

let print_complex z =
  Printf.printf "%f + %f i\n" z.re z.im

let () =
  let a = { re = 1.0; im = 1.0 }
  and b = { re = 3.14159; im = 1.25 } in
  print_complex (add a b);
  print_complex (mul a b);
  print_complex (inv a);
  print_complex (neg a);
  print_complex (conj a)
```
**Status:** [ ]

### 111: Arithmetic/Integer
**Source:** https://rosettacode.org/wiki/Arithmetic/Integer
**Topic:** Rosetta Code Arithmetic/Integer implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let _ =
  let a = read_int ()
  and b = read_int () in

  Printf.printf "a + b = %d\n" (a + b);
  Printf.printf "a - b = %d\n" (a - b);
  Printf.printf "a * b = %d\n" (a * b);
  Printf.printf "a / b = %d\n" (a / b);    (* truncates towards 0 *)
  Printf.printf "a mod b = %d\n" (a mod b) (* same sign as first operand *)
```
**Status:** [ ]

### 112: Arithmetic/Rational
**Source:** https://rosettacode.org/wiki/Arithmetic/Rational
**Topic:** Rosetta Code Arithmetic/Rational implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
#load "nums.cma";;
open Num;;

for candidate = 2 to 1 lsl 19 do
  let sum = ref (num_of_int 1 // num_of_int candidate) in
  for factor = 2 to truncate (sqrt (float candidate)) do
    if candidate mod factor = 0 then
      sum := !sum +/ num_of_int 1 // num_of_int factor
                  +/ num_of_int 1 // num_of_int (candidate / factor)
  done;
  if is_integer_num !sum then
    Printf.printf "Sum of recipr. factors of %d = %d exactly %s\n%!"
        candidate (int_of_num !sum) (if int_of_num !sum = 1 then "perfect!" else "")
done;;
```
**Status:** [ ]

### 113: Array concatenation
**Source:** https://rosettacode.org/wiki/Array_concatenation
**Topic:** Rosetta Code Array concatenation implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
# let list1 = [1; 2; 3];;
val list1 : int list = [1; 2; 3]
# let list2 = [4; 5; 6];;
val list2 : int list = [4; 5; 6]
# let list1and2 = list1 @ list2;;
val list1and2 : int list = [1; 2; 3; 4; 5; 6]
```
**Status:** [ ]

### 114: Arrays
**Source:** https://rosettacode.org/wiki/Arrays
**Topic:** Rosetta Code Arrays implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
# Array.make 6 'A' ;;
- : char array = [|'A'; 'A'; 'A'; 'A'; 'A'; 'A'|]

# Array.init 8 (fun i -> i * 10) ;;
- : int array = [|0; 10; 20; 30; 40; 50; 60; 70|]

# let arr = [|0; 1; 2; 3; 4; 5; 6 |] ;;
val arr : int array = [|0; 1; 2; 3; 4; 5; 6|]

# arr.(4) ;;
- : int = 4

# arr.(4) <- 65 ;;
- : unit = ()

# arr ;;
- : int array = [|0; 1; 2; 3; 65; 5; 6|]
```
**Status:** [ ]

### 115: Ascending primes
**Source:** https://rosettacode.org/wiki/Ascending_primes
**Topic:** Rosetta Code Ascending primes implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let is_prime n =
  let rec test x =
    let q = n / x in x > q || x * q <> n && n mod (x + 2) <> 0 && test (x + 6)
  in if n < 5 then n lor 1 = 3 else n land 1 <> 0 && n mod 3 <> 0 && test 5

let ascending_ints =
  let rec range10 m d = if d < 10 then m + d :: range10 m (succ d) else [] in
  let up n = range10 (n * 10) (succ (n mod 10)) in
  let rec next l = if l = [] then [] else l @ next (List.concat_map up l) in
  next [0]

let () =
  List.filter is_prime ascending_ints
  |> List.iter (Printf.printf " %u") |> print_newline
```
**Status:** [ ]

### 116: Associative array/Creation
**Source:** https://rosettacode.org/wiki/Associative_array/Creation
**Topic:** Rosetta Code Associative array/Creation implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let hash = Hashtbl.create 0;;
List.iter (fun (key, value) -> Hashtbl.add hash key value)
  ["foo", 5; "bar", 10; "baz", 15];;
```
**Status:** [ ]

### 117: Associative array/Iteration
**Source:** https://rosettacode.org/wiki/Associative_array/Iteration
**Topic:** Rosetta Code Associative array/Iteration implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
#!/usr/bin/env ocaml

let map = [| ('A', 1); ('B', 2); ('C', 3) |] ;;

(* iterate over pairs *)
Array.iter (fun (k,v) -> Printf.printf "key: %c - value: %d\n" k v) map ;;

(* iterate over keys *)
Array.iter (fun (k,_) -> Printf.printf "key: %c\n" k) map ;;

(* iterate over values *)
Array.iter (fun (_,v) -> Printf.printf "value: %d\n" v) map ;;

(* in functional programming it is often more useful to fold over the elements *)
Array.fold_left (fun acc (k,v) -> acc ^ Printf.sprintf "key: %c - value: %d\n" k v) "Elements:\n" map ;;
```
**Status:** [ ]

### 118: Associative array/Merging
**Source:** https://rosettacode.org/wiki/Associative_array/Merging
**Topic:** Rosetta Code Associative array/Merging implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
type ty = 
    | TFloat of float
    | TInt of int
    | TString of string

type key = string
type assoc = string * ty

let string_of_ty : ty -> string = function
    | TFloat x -> string_of_float x
    | TInt i -> string_of_int i
    | TString s -> s

let print_pair key el =
    Printf.printf "%s: %s\n" key (string_of_ty el)
;;
```
**Status:** [ ]

### 119: Averages/Arithmetic mean
**Source:** https://rosettacode.org/wiki/Averages/Arithmetic_mean
**Topic:** Rosetta Code Averages/Arithmetic mean implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let mean_floats = function
  | [] -> 0.
  | xs -> List.fold_left (+.) 0. xs /. float_of_int (List.length xs)

let mean_ints xs = mean_floats (List.map float_of_int xs)
```
**Status:** [ ]

### 120: Averages/Mean angle
**Source:** https://rosettacode.org/wiki/Averages/Mean_angle
**Topic:** Rosetta Code Averages/Mean angle implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let pi = 3.14159_26535_89793_23846_2643

let deg2rad d =
  d *. pi /. 180.0
 
let rad2deg r =
  r *. 180.0 /. pi

let mean_angle angles =
  let rad_angles = List.map deg2rad angles in
  let sum_sin = List.fold_left (fun sum a -> sum +. sin a) 0.0 rad_angles
  and sum_cos = List.fold_left (fun sum a -> sum +. cos a) 0.0 rad_angles
  in
  rad2deg (atan2 sum_sin sum_cos)

let test angles =
  Printf.printf "The mean angle of [%s] is: %g degrees\n"
    (String.concat "; " (List.map (Printf.sprintf "%g") angles))
    (mean_angle angles)

let () =
  test [350.0; 10.0];
  test [90.0; 180.0; 270.0; 360.0];
  test [10.0; 20.0; 30.0];
;;
```
**Status:** [ ]

### 121: Averages/Mean time of day
**Source:** https://rosettacode.org/wiki/Averages/Mean_time_of_day
**Topic:** Rosetta Code Averages/Mean time of day implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let pi_twice = 2.0 *. 3.14159_26535_89793_23846_2643
let day = float (24 * 60 * 60)

let rad_of_time t =
  t *. pi_twice /. day
 
let time_of_rad r =
  r *. day /. pi_twice

let mean_angle angles =
  let sum_sin = List.fold_left (fun sum a -> sum +. sin a) 0.0 angles
  and sum_cos = List.fold_left (fun sum a -> sum +. cos a) 0.0 angles in
  atan2 sum_sin sum_cos

let mean_time times =
  let angles = List.map rad_of_time times in
  let t = time_of_rad (mean_angle angles) in
  if t < 0.0 then t +. day else t

let parse_time t =
  Scanf.sscanf t "%d:%d:%d" (fun h m s -> float (s + m * 60 + h * 3600))

let round x = int_of_float (floor (x +. 0.5))

let string_of_time t =
  let t = round t in
  let h = t / 3600 in
  let rem = t mod 3600 in
  let m = rem / 60 in
  let s = rem mod 60 in
  Printf.sprintf "%d:%d:%d" h m s

let () =
  let times = ["23:00:17"; "23:40:20"; "00:12:45"; "00:17:19"] in
  Printf.printf "The mean time of [%s] is: %s\n"
    (String.concat "; " times)
    (string_of_time (mean_time (List.map parse_time times)))
```
**Status:** [ ]

### 122: 15 puzzle game
**Source:** https://rosettacode.org/wiki/15_puzzle_game
**Topic:** Rosetta Code 15 puzzle game implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
module Puzzle =
struct
  type t = int array
  let make () =
    [| 15; (* 0: the empty space *)
        0;  1;  2;  3;
        4;  5;  6;  7;
        8;  9; 10; 11;
       12; 13; 14;  |]

  let move p n =
    let hole, i = p.(0), p.(n) in
    p.(0) <- i;
    p.(n) <- hole

  let print p =
    let out = Array.make 16 "   " in
    for i = 1 to 15 do
      out.(p.(i)) <- Printf.sprintf " %2d" i
    done;
    for i = 0 to 15 do
      if (i mod 4) = 0 then print_newline ();
      print_string out.(i);
    done

  let shuffle p n =
    for i = 1 to n do
      move p (1 + Random.int 15)
    done
end

let play () =
  let p = Puzzle.make () in
  Puzzle.shuffle p 20;
  while true do
    Puzzle.print p;
    print_string " > ";
    Puzzle.move p (read_line () |> int_of_string)
  done
```
**Status:** [ ]

### 123: Abbreviations, easy
**Source:** https://rosettacode.org/wiki/Abbreviations,_easy
**Topic:** Rosetta Code Abbreviations, easy implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let cmds = "\
  Add ALTer  BAckup Bottom  CAppend Change SCHANGE  CInsert CLAst COMPress COpy
  COUnt COVerlay CURsor DELete CDelete Down DUPlicate Xedit EXPand EXTract Find
  NFind NFINDUp NFUp CFind FINdup FUp FOrward GET Help HEXType Input POWerinput
  Join SPlit SPLTJOIN  LOAD  Locate CLocate  LOWercase UPPercase  LPrefix MACRO
  MErge MODify MOve MSG Next Overlay PARSE PREServe PURge PUT PUTD  Query  QUIT
  READ  RECover REFRESH RENum REPeat  Replace CReplace  RESet  RESTore  RGTLEFT
  RIght LEft  SAVE  SET SHift SI  SORT  SOS  STAck STATus  TOP TRAnsfer Type Up"

let user =
  "riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin"

let char_is_uppercase c =
  match c with
  | 'A'..'Z' -> true
  | _ -> false

let get_abbr s =
  let seq = String.to_seq s in
  let seq = Seq.filter char_is_uppercase seq in
  (String.of_seq seq)

let () =
  let cmds = Str.split (Str.regexp "[ \r\n]+") cmds in
  let cmds =
    List.map (fun s ->
      get_abbr s,
      String.uppercase_ascii s
    ) cmds
  in
  let user = Str.split (Str.regexp "[ \r\n]+") user in
  let r =
    List.map (fun ucmd ->
      let n = String.length ucmd in
      let find_abbr (abbr, cmd) =
        let na = String.length abbr in
        let nc = String.length cmd in
        if n < na || nc < n then false else
          let sub = String.sub cmd 0 n in
          (sub = String.uppercase_ascii ucmd)
      in
      match List.find_opt find_abbr cmds with
      | Some (_, found) -> found
      | None -> "*error*"
    ) user
  in
  print_endline (String.concat " " r)
```
**Status:** [ ]

### 124: Abbreviations, simple
**Source:** https://rosettacode.org/wiki/Abbreviations,_simple
**Topic:** Rosetta Code Abbreviations, simple implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
open String

let table_as_string =
  "add 1 alter 3  backup 2  bottom 1  Cappend 2  change 1 \
   Schange  Cinsert 2  Clast 3 compress 4 copy 2 count 3 \
   Coverlay 3 cursor 3  delete 3 Cdelete 2  down 1  duplicate 3 \
   xEdit 1 expand 3 extract 3  find 1 Nfind 2 Nfindup 6 NfUP 3 \
   Cfind 2 findUP 3 fUP 2 forward 2  get  help 1 hexType 4 \
   input 1 powerInput 3  join 1 split 2 spltJOIN load locate 1 \
   Clocate 2 lowerCase 3 upperCase 3 Lprefix 2  macro  merge 2 \
   modify 3 move 2 msg  next 1 overlay 1 parse preserve 4 purge 3 \
   put putD query 1 quit  read recover 3 refresh renum 3 repeat 3 \
   replace 1 Creplace 2 reset 3 restore 4 rgtLEFT right 2 left 2 \
   save  set  shift 2  si  sort  sos  stack 3 status 4 top \
   transfer 3  type 1  up 1"
```
**Status:** [ ]

### 125: ABC correlation
**Source:** https://rosettacode.org/wiki/ABC_correlation
**Topic:** Rosetta Code ABC correlation implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
(* Function to count number of occurrences of char `chr` in string `str`. *)
let count (chr : char) (str : string) : int 
  = str |> String.to_seq |> Seq.filter ((=) chr) |> Seq.length

let main () : bool = 
  (* Get input from command line arg... *)
  let input = Array.get Sys.argv 1 in 
  (* ...count number of occurences of a, b, c in input... *)
  List.map ((|>) input) (List.map count ['a';'b';'c'])
  (* ...return whether they are all equal. *)
  |> (fun l -> List.for_all ((=) (List.hd l)) l)
  
(* Get and print result. *)
let () = Printf.printf "%b" @@ main ()
```
**Status:** [ ]

### 126: ABC problem
**Source:** https://rosettacode.org/wiki/ABC_problem
**Topic:** Rosetta Code ABC problem implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let blocks = [
  ('B', 'O');  ('X', 'K');  ('D', 'Q');  ('C', 'P');
  ('N', 'A');  ('G', 'T');  ('R', 'E');  ('T', 'G');
  ('Q', 'D');  ('F', 'S');  ('J', 'W');  ('H', 'U');
  ('V', 'I');  ('A', 'N');  ('O', 'B');  ('E', 'R');
  ('F', 'S');  ('L', 'Y');  ('P', 'C');  ('Z', 'M');
]

let find_letter blocks c =
  let found, remaining =
    List.partition (fun (c1, c2) -> c1 = c || c2 = c) blocks
  in
  match found with
  | _ :: res -> Some (res @ remaining)
  | _ -> None

let can_make_word w =
  let n = String.length w in
  let rec aux i _blocks =
    if i >= n then true else
      match find_letter _blocks w.[i] with
      | None -> false
      | Some rem_blocks ->
          aux (succ i) rem_blocks
  in
  aux 0 blocks

let test label f (word, should) =
  Printf.printf "- %s %S = %B  (should: %B)\n" label word (f word) should

let () =
  List.iter (test "can make word" can_make_word) [
    "A", true;
    "BARK", true;
    "BOOK", false;
    "TREAT", true;
    "COMMON", false;
    "SQUAD", true;
    "CONFUSE", true;
  ]
```
**Status:** [ ]

### 127: ABC words
**Source:** https://rosettacode.org/wiki/ABC_words
**Topic:** Rosetta Code ABC words implementation in OCaml
**Difficulty:** Beginner
**Category:** Strings
**OCaml:**
```ocaml
let is_abc_word (word : string) : bool =
  try 
    String.index word 'a' 
    |> fun i -> String.index_from word i 'b' 
    |> fun i -> String.index_from word i 'c'
    |> ignore; true
  with Not_found -> false

let () = 
  In_channel.with_open_text "unixdict.txt" In_channel.input_all
  |> String.split_on_char '\n'
  |> List.filter is_abc_word
  |> String.concat ", "
  |> print_endline
```
**Status:** [ ]

### 128: Address of a variable
**Source:** https://rosettacode.org/wiki/Address_of_a_variable
**Topic:** Rosetta Code Address of a variable implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let address_of (x:'a) : nativeint =
  if Obj.is_block (Obj.repr x) then
    Nativeint.shift_left (Nativeint.of_int (Obj.magic x)) 1 (* magic *)
  else
    invalid_arg "Can only find address of boxed values.";;

let () =
  let a = 3.14 in
  Printf.printf "%nx\n" (address_of a);;
  let b = ref 42 in
  Printf.printf "%nx\n" (address_of b);;
  let c = 17 in
  Printf.printf "%nx\n" (address_of c);; (* error, because int is unboxed *)
```
**Status:** [ ]

### 129: Align columns
**Source:** https://rosettacode.org/wiki/Align_columns
**Topic:** Rosetta Code Align columns implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
#load "str.cma"
open Str

let input = "\
Given$a$text$file$of$many$lines,$where$fields$within$a$line$
are$delineated$by$a$single$'dollar'$character,$write$a$program
that$aligns$each$column$of$fields$by$ensuring$that$words$in$each$
column$are$separated$by$at$least$one$space.
Further,$allow$for$each$word$in$a$column$to$be$either$left$
justified,$right$justified,$or$center$justified$within$its$column."

let () =
  let lines = split (regexp_string "\n") input in
  let fields_l = List.map (split (regexp_string "$")) lines in
  let fields_l = List.map Array.of_list fields_l in
  let n = (* number of columns *)
    List.fold_left
      (fun n fields -> max n (Array.length fields))
      0 fields_l
  in
  let pads = Array.make n 0 in
  List.iter (
    (* calculate the max padding for each column *)
    Array.iteri
      (fun i word -> pads.(i) <- max pads.(i) (String.length word))
  ) fields_l;

  let print f =
    List.iter (fun fields ->
      Array.iteri (fun i word ->
        f word (pads.(i) - (String.length word))
      ) fields;
      print_newline()
    ) fields_l;
  in

  (* left column-aligned output *)
  print (fun word pad ->
    let spaces = String.make pad ' ' in
    Printf.printf "%s%s " word spaces);

  (* right column-aligned output *)
  print (fun word pad ->
    let spaces = String.make pad ' ' in
    Printf.printf "%s%s " spaces word);

  (* center column-aligned output *)
  print (fun word pad ->
    let pad1 = pad / 2 in
    let pad2 = pad - pad1 in
    let sp1 = String.make pad1 ' ' in
    let sp2 = String.make pad2 ' ' in
    Printf.printf "%s%s%s " sp1 word sp2);
;;
```
**Status:** [ ]

### 130: Alternade words
**Source:** https://rosettacode.org/wiki/Alternade_words
**Topic:** Rosetta Code Alternade words implementation in OCaml
**Difficulty:** Beginner
**Category:** Strings
**OCaml:**
```ocaml
module StrSet = Set.Make(String)

let seq_lines ch =
  let rec repeat () =
    match input_line ch with
    | s -> Seq.Cons (s, repeat)
    | exception End_of_file -> Nil
  in repeat

let min_len l s =
  l <= String.length s

let get_alternade set s =
  let s0 = String.init (succ (String.length s) lsr 1) (fun i -> s.[i + i])
  and s1 = String.init (String.length s lsr 1) (fun i -> s.[i + succ i]) in
  if StrSet.mem s0 set && StrSet.mem s1 set
  then Some (Printf.sprintf "%s | %s %s" s s0 s1) else None

let () =
  let set = seq_lines stdin |> Seq.filter (min_len 3) |> StrSet.of_seq in
  StrSet.to_seq set |> Seq.filter (min_len 6)
  |> Seq.filter_map (get_alternade set) |> Seq.iter print_endline
```
**Status:** [ ]

### 131: Anadromes
**Source:** https://rosettacode.org/wiki/Anadromes
**Topic:** Rosetta Code Anadromes implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
module StrSet = Set.Make(String)

let read_line_seq ch =
  let rec repeat () =
    match input_line ch with
    | s -> Seq.Cons (s, repeat)
    | exception End_of_file -> Nil
  in repeat

let string_rev s =
  let last = pred (String.length s) in
  String.init (succ last) (fun i -> s.[last - i])

let get_anadromes set =
  let aux s =
    let r = string_rev s in
    if s < r && StrSet.mem r set
    then Some (s, r)
    else None
  in
  Seq.filter_map aux (StrSet.to_seq set)

let () = read_line_seq stdin |> Seq.filter (fun s -> String.length s > 6)
  |> Seq.map String.lowercase_ascii |> StrSet.of_seq |> get_anadromes
  |> Seq.iter (fun (s, r) -> Printf.printf "%9s | %s\n" s r)
```
**Status:** [ ]

### 132: Angle difference between two bearings
**Source:** https://rosettacode.org/wiki/Angle_difference_between_two_bearings
**Topic:** Rosetta Code Angle difference between two bearings implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let get_diff b1 b2 =
  let r = mod_float (b2 -. b1) 360.0 in
  if r < -180.0
  then r +. 360.0
  else if r >= 180.0
  then r -. 360.0
  else r
 
let () =
  print_endline "Input in -180 to +180 range";
  Printf.printf " %g\n" (get_diff 20.0 45.0);
  Printf.printf " %g\n" (get_diff (-45.0) 45.0);
  Printf.printf " %g\n" (get_diff (-85.0) 90.0);
  Printf.printf " %g\n" (get_diff (-95.0) 90.0);
  Printf.printf " %g\n" (get_diff (-45.0) 125.0);
  Printf.printf " %g\n" (get_diff (-45.0) 145.0);
  Printf.printf " %g\n" (get_diff (-45.0) 125.0);
  Printf.printf " %g\n" (get_diff (-45.0) 145.0);
  Printf.printf " %g\n" (get_diff 29.4803 (-88.6381));
  Printf.printf " %g\n" (get_diff (-78.3251) (-159.036));
 
  print_endline "Input in wider range";
  Printf.printf " %g\n" (get_diff (-70099.74233810938) 29840.67437876723);
  Printf.printf " %g\n" (get_diff (-165313.6666297357) 33693.9894517456);
  Printf.printf " %g\n" (get_diff 1174.8380510598456 (-154146.66490124757));
  Printf.printf " %g\n" (get_diff 60175.77306795546 42213.07192354373);
;;
```
**Status:** [ ]

### 133: Append numbers at same position in strings
**Source:** https://rosettacode.org/wiki/Append_numbers_at_same_position_in_strings
**Topic:** Rosetta Code Append numbers at same position in strings implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let lists = [
  ["1"; "2"; "3"; "4"; "5"; "6"; "7"; "8"; "9"];
  ["10"; "11"; "12"; "13"; "14"; "15"; "16"; "17"; "18"];
  ["19"; "20"; "21"; "22"; "23"; "24"; "25"; "26"; "27"]]

let reduce f = function
  | h :: t -> List.fold_left f h t
  | _ -> invalid_arg "reduce"

let () =
  reduce (List.map2 (^)) lists |> String.concat ", " |> print_endline
```
**Status:** [ ]

### 134: Approximate equality
**Source:** https://rosettacode.org/wiki/Approximate_equality
**Topic:** Rosetta Code Approximate equality implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let approx_eq v1 v2 epsilon =
  Float.abs (v1 -. v2) < epsilon

let test a b =
  let epsilon = 1e-18 in
  Printf.printf "%g, %g => %b\n" a b (approx_eq a b epsilon)

let () =
  test 100000000000000.01 100000000000000.011;
  test 100.01 100.011;
  test (10000000000000.001 /. 10000.0) 1000000000.0000001000;
  test 0.001 0.0010000001;
  test 0.000000000000000000000101 0.0;
  test ((sqrt 2.0) *. (sqrt 2.0)) 2.0;
  test (-. (sqrt 2.0) *. (sqrt 2.0)) (-2.0);
  test 3.14159265358979323846 3.14159265358979324;
;;
```
**Status:** [ ]

### 135: Apéry's constant
**Source:** https://rosettacode.org/wiki/Apéry's_constant
**Topic:** Rosetta Code Apéry's constant implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
(* Using:
  https://ocaml.org/p/decimal
  https://ocaml.org/p/zarith *)

Decimal.Context.default := Decimal.Context.make ~prec:100 ()

let cubesum (nterms : int) : Decimal.t = 
  Array.init nterms ((+) 1)
  |> Array.map Decimal.of_int
  |> Array.map (fun d -> Decimal.(d * d * d))
  |> Array.map Decimal.((/) one)
  |> Array.fold_left Decimal.(+) Decimal.zero

let markov (nterms : int) : Decimal.t =
  let apery_term (k : int) : Decimal.t =
    let num = Z.((fac k) ** 2) in
    let den = Z.( * ) (Z.fac @@ 2 * k) Z.((of_int k) ** 3) in
    let frac = Decimal.(div (of_bigint num) (of_bigint den)) in
    let coeff = Decimal.of_int (if k mod 2 = 1 then +1 else -1) in
    Decimal.(coeff * frac) in
  let five_halves = Decimal.(div (of_int 5) (of_int 2)) in
  List.init nterms (fun n -> apery_term @@ n + 1)
    |> List.fold_left Decimal.(+) Decimal.zero 
    |> Decimal.( * ) five_halves

let wedeniwski (nterms : int) : Decimal.t =
  let coeffs = [|126392; 412708; 531578; 336367; 104000; 12463|] in
  let zcoeffs = Array.map Z.of_int coeffs in
  let apery_term (k : int ) : Decimal.t = 
    let kz = Z.of_int k in
    let pows = Array.init 6 (fun p -> Z.( ** ) kz (5 - p)) in
    let zfact_cubed = fun z -> Z.(pow (fac z) 3) in
    let num_lhs = [|2 * k + 1; 2 * k; k|] 
      |> Array.map zfact_cubed 
      |> Array.fold_left Z.( * ) Z.one 
      |> Decimal.of_bigint in
    let num_rhs = Array.combine zcoeffs pows
      |> Array.map (fun (c, p) -> Z.( * ) c p)  
      |> Array.fold_left Z.(+) Z.zero 
      |> Decimal.of_bigint in
    let den = Z.( * ) (zfact_cubed @@ 4 * k + 3) (Z.fac @@ 3 * k + 2) 
      |> Decimal.of_bigint in
    let sgn = Decimal.of_int (if k mod 2 = 0 then +1 else -1) in
    Decimal.(sgn * (num_lhs * num_rhs) / den) in
    List.init nterms apery_term
      |> List.fold_left Decimal.(+) Decimal.zero
      |> Decimal.(( * ) (div one (of_int 24)))

let () =
  cubesum 1000
    |> Decimal.to_string
    |> Printf.printf "Naive: %s\n\n";
  markov 158
    |> Decimal.to_string
    |> Printf.printf "Markov: %s\n\n";
  wedeniwski 20
    |> Decimal.to_string
    |> Printf.printf "Wedeniwski: %s\n";
```
**Status:** [ ]

### 136: Arbitrary-precision integers (included)
**Source:** https://rosettacode.org/wiki/Arbitrary-precision_integers_(included)
**Topic:** Rosetta Code Arbitrary-precision integers (included) implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
open Num
open Str
open String

let () =
  let answer = (Int 5) **/ (Int 4) **/ (Int 3) **/ (Int 2) in
  let answer_string = string_of_num answer in
  Printf.printf "has %d digits: %s ... %s\n"
                (length answer_string)
                (first_chars answer_string 20)
                (last_chars answer_string 20)
```
**Status:** [ ]

### 137: Assertions
**Source:** https://rosettacode.org/wiki/Assertions
**Topic:** Rosetta Code Assertions implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let a = get_some_value () in
  assert (a = 42); (* throws Assert_failure when a is not 42 *)
  (* evaluate stuff to return here when a is 42 *)
```
**Status:** [ ]

### 138: Attractive numbers
**Source:** https://rosettacode.org/wiki/Attractive_numbers
**Topic:** Rosetta Code Attractive numbers implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let is_prime (n : int) : bool = 
  if n = 2 then true else if n < 2 || n mod 2 = 0 then false else
	let lim = (n |> float_of_int |> sqrt |> int_of_float) + 1 in
	let rec loop = function 
		| i when i > lim -> true
		| i when n mod i = 0 -> false
		| i -> loop (i + 2)
	in loop 3

let prime_factors (n : int) : int list =
  let rec loop = function
    | factors, i, r when r = 1 -> factors
    | factors, i, r when is_prime i && r mod i = 0
      -> loop (i :: factors, i, r / i) 
    | factors, i, r -> loop (factors, i+1, r)
  in loop ([], 2, n)

let is_attractive (n : int) : bool =
  n |> prime_factors |> List.length |> is_prime

let () = 
  List.init 120 ((+) 1)
  |> List.filter is_attractive
  |> List.map string_of_int
  |> String.concat ","
  |> Printf.printf "[%s]"
```
**Status:** [ ]

### 139: Averages/Median
**Source:** https://rosettacode.org/wiki/Averages/Median
**Topic:** Rosetta Code Averages/Median implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
(* note: this modifies the input array *)
let median array =
  let len = Array.length array in
    Array.sort compare array;
    (array.((len-1)/2) +. array.(len/2)) /. 2.0;;

let a = [|4.1; 5.6; 7.2; 1.7; 9.3; 4.4; 3.2|];;
median a;;
let a = [|4.1; 7.2; 1.7; 9.3; 4.4; 3.2|];;
median a;;
```
**Status:** [ ]

### 140: Averages/Mode
**Source:** https://rosettacode.org/wiki/Averages/Mode
**Topic:** Rosetta Code Averages/Mode implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let mode lst =
  let seen = Hashtbl.create 42 in
    List.iter (fun x ->
                 let old = if Hashtbl.mem seen x then
                   Hashtbl.find seen x
                 else 0 in
                   Hashtbl.replace seen x (old + 1))
      lst;
    let best = Hashtbl.fold (fun _ -> max) seen 0 in
      Hashtbl.fold (fun k v acc ->
                      if v = best then k :: acc
                      else acc)
        seen []
```
**Status:** [ ]

### 141: Averages/Pythagorean means
**Source:** https://rosettacode.org/wiki/Averages/Pythagorean_means
**Topic:** Rosetta Code Averages/Pythagorean means implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let means v =
  let n = Array.length v
  and a = ref 0.0
  and b = ref 1.0
  and c = ref 0.0 in
  for i=0 to n-1 do
    a := !a +. v.(i);
    b := !b *. v.(i);
    c := !c +. 1.0/.v.(i);
  done;
  let nn = float_of_int n in
  (!a /. nn, !b ** (1.0/.nn), nn /. !c)
;;
```
**Status:** [ ]

### 142: Averages/Root mean square
**Source:** https://rosettacode.org/wiki/Averages/Root_mean_square
**Topic:** Rosetta Code Averages/Root mean square implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let rms a =
  sqrt (Array.fold_left (fun s x -> s +. x*.x) 0.0 a /.
        float_of_int (Array.length a))
;;

rms (Array.init 10 (fun i -> float_of_int (i+1))) ;;
(* 6.2048368229954285 *)
```
**Status:** [ ]

### 143: Averages/Simple moving average
**Source:** https://rosettacode.org/wiki/Averages/Simple_moving_average
**Topic:** Rosetta Code Averages/Simple moving average implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let sma (n, s, q) x =
  let l = Queue.length q and s = s +. x in
  Queue.push x q;
  if l < n then 
    (n, s, q), s /. float (l + 1)
  else (
    let s = s -. Queue.pop q in
    (n, s, q), s /. float l
  )

let _ =
  let periodLst = [ 3; 5 ] in
  let series = [ 1.; 2.; 3.; 4.; 5.; 5.; 4.; 3.; 2.; 1. ] in
  
  List.iter (fun d -> 
    Printf.printf "SIMPLE MOVING AVERAGE: PERIOD = %d\n" d;
    ignore (
      List.fold_left (fun o x ->
	let o, m = sma o x in
	Printf.printf "Next number = %-2g, SMA = %g\n" x m;
	o
      ) (d, 0., Queue.create ()) series;
    );
    print_newline ();
  ) periodLst
```
**Status:** [ ]

### 144: Babbage problem
**Source:** https://rosettacode.org/wiki/Babbage_problem
**Topic:** Rosetta Code Babbage problem implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let rec f a=
if (a*a) mod 1000000 != 269696
then f(a+1)
else a
in
let a= f 1 in
Printf.printf "smallest positive integer whose square ends in the digits 269696 is %d\n" a
```
**Status:** [ ]

### 145: Base 16 numbers needing a to f
**Source:** https://rosettacode.org/wiki/Base_16_numbers_needing_a_to_f
**Topic:** Rosetta Code Base 16 numbers needing a to f implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let rec has_xdigit n =
  n land 15 > 9 || n > 15 && has_xdigit (n lsr 4)

let () =
  Seq.(ints 1 |> take 500 |> filter has_xdigit |> map string_of_int)
  |> List.of_seq |> String.concat " " |> print_endline
```
**Status:** [ ]

### 146: Best shuffle
**Source:** https://rosettacode.org/wiki/Best_shuffle
**Topic:** Rosetta Code Best shuffle implementation in OCaml
**Difficulty:** Intermediate
**Category:** General
**OCaml:**
```ocaml
let best_shuffle s =
  let len = String.length s in
  let r = String.copy s in
  for i = 0 to pred len do
    for j = 0 to pred len do
      if i <> j && s.[i] <> r.[j] && s.[j] <> r.[i] then
        begin
          let tmp = r.[i] in
          r.[i] <- r.[j];
          r.[j] <- tmp;
        end
    done;
  done;
  (r)

let count_same s1 s2 =
  let len1 = String.length s1
  and len2 = String.length s2 in
  let n = ref 0 in
  for i = 0 to pred (min len1 len2) do
    if s1.[i] = s2.[i] then incr n
  done;
  !n

let () =
  let test s =
    let s2 = best_shuffle s in
    Printf.printf " '%s', '%s' -> %d\n" s s2 (count_same s s2);
  in
  test "tree";
  test "abracadabra";
  test "seesaw";
  test "elk";
  test "grrrrrr";
  test "up";
  test "a";
;;
```
**Status:** [ ]

### 147: Binary digits
**Source:** https://rosettacode.org/wiki/Binary_digits
**Topic:** Rosetta Code Binary digits implementation in OCaml
**Difficulty:** Intermediate
**Category:** Math
**OCaml:**
```ocaml
let bin_of_int d =
  let last_digit n = [|"0"; "1"|].(n land 1) in
  let rec aux lst = function
    | 0 -> lst
    | n -> aux (last_digit n :: lst) (n lsr 1)
  in
  String.concat "" (aux [last_digit d] (d lsr 1))

(* test *)
let () = [0; 1; 2; 5; 50; 9000; -5]
  |> List.map bin_of_int |> String.concat ", " |> print_endline
```
**Status:** [ ]

### 148: Binary strings
**Source:** https://rosettacode.org/wiki/Binary_strings
**Topic:** Rosetta Code Binary strings implementation in OCaml
**Difficulty:** Beginner
**Category:** Strings
**OCaml:**
```ocaml
# let str = "some text" ;;
val str : string = "some text"

(* modifying a character, OCaml strings are mutable *)
# str.[0] <- 'S' ;;
- : unit = ()
```
**Status:** [ ]
Generated 27 more entries


### 061: Pattern Matching Basics
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/pattern_matching.html
**Topic:** Match expressions for structured data decomposition
**Difficulty:** Beginner
**Category:** pattern-matching
**OCaml:**
```ocaml
let describe_list = function
  | [] -> "empty"
  | [x] -> Printf.sprintf "singleton: %d" x
  | [x; y] -> Printf.sprintf "pair: %d, %d" x y
  | x :: _ -> Printf.sprintf "starts with %d" x

let () =
  List.iter (fun lst ->
    Printf.printf "%s\n" (describe_list lst)
  ) [[]; [1]; [2;3]; [4;5;6]]
```
**Status:** [ ]

### 062: Recursive List Functions
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/lists.html
**Topic:** Write recursive functions over lists
**Difficulty:** Beginner
**Category:** recursion
**OCaml:**
```ocaml
let rec length = function
  | [] -> 0
  | _ :: tl -> 1 + length tl

let rec append l1 l2 = match l1 with
  | [] -> l2
  | hd :: tl -> hd :: append tl l2

let rec rev_acc acc = function
  | [] -> acc
  | hd :: tl -> rev_acc (hd :: acc) tl
let rev = rev_acc []

let () = Printf.printf "length: %d\n" (length [1;2;3;4;5])
let () = List.iter (fun x -> Printf.printf "%d " x) (rev [1;2;3])
```
**Status:** [ ]

### 063: Algebraic Data Types — Binary Tree
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/algebraic_data_types.html
**Topic:** Define and traverse a binary tree with variants
**Difficulty:** Intermediate
**Category:** algebraic-types
**OCaml:**
```ocaml
type 'a tree =
  | Leaf
  | Node of 'a tree * 'a * 'a tree

let rec insert x = function
  | Leaf -> Node (Leaf, x, Leaf)
  | Node (l, v, r) ->
    if x < v then Node (insert x l, v, r)
    else if x > v then Node (l, v, insert x r)
    else Node (l, v, r)

let rec inorder = function
  | Leaf -> []
  | Node (l, v, r) -> inorder l @ [v] @ inorder r

let tree = List.fold_left (fun t x -> insert x t) Leaf [5;3;7;1;4;6;8]
let () = List.iter (fun x -> Printf.printf "%d " x) (inorder tree)
```
**Status:** [ ]

### 064: Higher-Order Functions — Map and Filter from Scratch
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/hop/higher_order.html
**Topic:** Implement map and filter as higher-order functions
**Difficulty:** Beginner
**Category:** higher-order
**OCaml:**
```ocaml
let rec my_map f = function
  | [] -> []
  | x :: xs -> f x :: my_map f xs

let rec my_filter pred = function
  | [] -> []
  | x :: xs ->
    if pred x then x :: my_filter pred xs
    else my_filter pred xs

let squares = my_map (fun x -> x * x) [1;2;3;4;5]
let big = my_filter (fun x -> x > 10) squares
let () = List.iter (fun x -> Printf.printf "%d " x) big
```
**Status:** [ ]

### 065: Tail Recursion
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/lists.html
**Topic:** Convert recursive functions to tail-recursive form
**Difficulty:** Intermediate
**Category:** recursion
**OCaml:**
```ocaml
(* Non-tail-recursive: stack overflow on large lists *)
let rec sum_naive = function
  | [] -> 0
  | x :: xs -> x + sum_naive xs

(* Tail-recursive with accumulator *)
let sum lst =
  let rec aux acc = function
    | [] -> acc
    | x :: xs -> aux (acc + x) xs
  in aux 0 lst

(* Tail-recursive map using rev *)
let map f lst =
  let rec aux acc = function
    | [] -> List.rev acc
    | x :: xs -> aux (f x :: acc) xs
  in aux [] lst

let () = Printf.printf "Sum: %d\n" (sum (List.init 1000000 Fun.id))
```
**Status:** [ ]

### 066: Records and Named Fields
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/records.html
**Topic:** Define and use record types
**Difficulty:** Beginner
**Category:** records
**OCaml:**
```ocaml
type student = {
  name : string;
  id : int;
  gpa : float;
  year : int;
}

let alice = { name = "Alice"; id = 1001; gpa = 3.8; year = 3 }
let bob = { name = "Bob"; id = 1002; gpa = 3.5; year = 2 }

let promote s = { s with year = s.year + 1 }
let alice_next = promote alice

let honor_roll students =
  List.filter (fun s -> s.gpa >= 3.7) students

let () = Printf.printf "%s (year %d, GPA %.1f)\n"
  alice_next.name alice_next.year alice_next.gpa
```
**Status:** [ ]

### 067: Modules and Signatures
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/modules/modules.html
**Topic:** Define modules with signatures for encapsulation
**Difficulty:** Intermediate
**Category:** modules
**OCaml:**
```ocaml
module type STACK = sig
  type 'a t
  val empty : 'a t
  val push : 'a -> 'a t -> 'a t
  val pop : 'a t -> ('a * 'a t) option
  val is_empty : 'a t -> bool
end

module ListStack : STACK = struct
  type 'a t = 'a list
  let empty = []
  let push x s = x :: s
  let pop = function
    | [] -> None
    | x :: xs -> Some (x, xs)
  let is_empty = function [] -> true | _ -> false
end

let s = ListStack.(empty |> push 1 |> push 2 |> push 3)
let () = match ListStack.pop s with
  | Some (x, _) -> Printf.printf "Top: %d\n" x
  | None -> print_endline "Empty"
```
**Status:** [ ]

### 068: Functors — Parameterized Modules
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/modules/functors.html
**Topic:** Create modules parameterized by other modules
**Difficulty:** Advanced
**Category:** functors
**OCaml:**
```ocaml
module type COMPARABLE = sig
  type t
  val compare : t -> t -> int
end

module MakeSortedList (C : COMPARABLE) = struct
  type t = C.t list
  let empty = []
  let rec insert x = function
    | [] -> [x]
    | hd :: tl ->
      if C.compare x hd <= 0 then x :: hd :: tl
      else hd :: insert x tl
  let to_list t = t
end

module IntSorted = MakeSortedList(Int)
let s = List.fold_left (fun acc x -> IntSorted.insert x acc) IntSorted.empty [5;3;7;1;4]
let () = List.iter (fun x -> Printf.printf "%d " x) (IntSorted.to_list s)
```
**Status:** [ ]

### 069: Mutable State — Refs and Loops
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/mut/refs.html
**Topic:** Use references for mutable state
**Difficulty:** Beginner
**Category:** mutability
**OCaml:**
```ocaml
let counter = ref 0

let next () =
  let v = !counter in
  counter := v + 1;
  v

let () =
  for _ = 1 to 5 do
    Printf.printf "%d " (next ())
  done;
  print_newline ()

(* Imperative factorial *)
let factorial n =
  let result = ref 1 in
  for i = 2 to n do
    result := !result * i
  done;
  !result

let () = Printf.printf "10! = %d\n" (factorial 10)
```
**Status:** [ ]

### 070: Mutable Records and Arrays
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/mut/mutable_fields.html
**Topic:** Mutable record fields for stateful objects
**Difficulty:** Intermediate
**Category:** mutability
**OCaml:**
```ocaml
type counter = {
  mutable count : int;
  name : string;
}

let make_counter name = { count = 0; name }

let increment c = c.count <- c.count + 1
let reset c = c.count <- 0
let value c = c.count

let c = make_counter "clicks"
let () =
  for _ = 1 to 10 do increment c done;
  Printf.printf "%s: %d\n" c.name (value c);
  reset c;
  Printf.printf "After reset: %d\n" (value c)
```
**Status:** [ ]

### 071: Exceptions — Define, Raise, Handle
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/exceptions.html
**Topic:** Custom exceptions and exception handling
**Difficulty:** Intermediate
**Category:** exceptions
**OCaml:**
```ocaml
exception Invalid_input of string
exception Out_of_range of { value: int; min: int; max: int }

let safe_sqrt x =
  if x < 0.0 then raise (Invalid_input "negative number")
  else sqrt x

let clamp ~min ~max x =
  if x < min || x > max then
    raise (Out_of_range { value = x; min; max })
  else x

let () =
  (try Printf.printf "sqrt(4) = %.1f\n" (safe_sqrt 4.0) with _ -> ());
  (try ignore (safe_sqrt (-1.0)) with
   | Invalid_input msg -> Printf.printf "Error: %s\n" msg);
  (try ignore (clamp ~min:0 ~max:100 150) with
   | Out_of_range r -> Printf.printf "Out of range: %d not in [%d,%d]\n" r.value r.min r.max)
```
**Status:** [ ]

### 072: Variant Types — Shape Calculator
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/algebraic_data_types.html
**Topic:** Use variants to model different cases
**Difficulty:** Beginner
**Category:** algebraic-types
**OCaml:**
```ocaml
type shape =
  | Circle of float
  | Rectangle of float * float
  | Triangle of float * float * float

let area = function
  | Circle r -> Float.pi *. r *. r
  | Rectangle (w, h) -> w *. h
  | Triangle (a, b, c) ->
    let s = (a +. b +. c) /. 2.0 in
    sqrt (s *. (s -. a) *. (s -. b) *. (s -. c))

let perimeter = function
  | Circle r -> 2.0 *. Float.pi *. r
  | Rectangle (w, h) -> 2.0 *. (w +. h)
  | Triangle (a, b, c) -> a +. b +. c

let shapes = [Circle 5.0; Rectangle (3.0, 4.0); Triangle (3.0, 4.0, 5.0)]
let () = List.iter (fun s ->
  Printf.printf "Area: %.2f, Perimeter: %.2f\n" (area s) (perimeter s)
) shapes
```
**Status:** [ ]

### 073: Polymorphic Variants
**Source:** Real World OCaml — https://dev.realworldocaml.org/variants.html
**Topic:** Open variant types with backtick syntax
**Difficulty:** Advanced
**Category:** variants
**OCaml:**
```ocaml
let describe_color = function
  | `Red -> "red"
  | `Green -> "green"
  | `Blue -> "blue"
  | `Custom (r, g, b) -> Printf.sprintf "rgb(%d,%d,%d)" r g b

let is_primary = function
  | `Red | `Green | `Blue -> true
  | `Custom _ -> false

let colors = [`Red; `Blue; `Custom (128, 0, 255)]
let () = List.iter (fun c ->
  Printf.printf "%s (primary: %b)\n" (describe_color c) (is_primary c)
) colors
```
**Status:** [ ]

### 074: GADTs — Type-Safe Expression Evaluator
**Source:** Real World OCaml — https://dev.realworldocaml.org/gadts.html
**Topic:** Generalized algebraic data types for type safety
**Difficulty:** Advanced
**Category:** gadts
**OCaml:**
```ocaml
type _ expr =
  | Int : int -> int expr
  | Bool : bool -> bool expr
  | Add : int expr * int expr -> int expr
  | If : bool expr * 'a expr * 'a expr -> 'a expr
  | Eq : int expr * int expr -> bool expr

let rec eval : type a. a expr -> a = function
  | Int n -> n
  | Bool b -> b
  | Add (a, b) -> eval a + eval b
  | If (cond, t, f) -> if eval cond then eval t else eval f
  | Eq (a, b) -> eval a = eval b

let result = eval (If (Eq (Add (Int 2, Int 3), Int 5), Int 1, Int 0))
let () = Printf.printf "Result: %d\n" result
```
**Status:** [ ]

### 075: First-Class Modules
**Source:** Real World OCaml — https://dev.realworldocaml.org/first-class-modules.html
**Topic:** Pack and unpack modules as values
**Difficulty:** Advanced
**Category:** modules
**OCaml:**
```ocaml
module type SHOWABLE = sig
  type t
  val to_string : t -> string
end

let show (type a) (module S : SHOWABLE with type t = a) (x : a) =
  S.to_string x

let int_show = (module struct
  type t = int
  let to_string = string_of_int
end : SHOWABLE with type t = int)

let float_show = (module struct
  type t = float
  let to_string = Printf.sprintf "%.2f"
end : SHOWABLE with type t = float)

let () =
  Printf.printf "%s\n" (show int_show 42);
  Printf.printf "%s\n" (show float_show 3.14)
```
**Status:** [ ]

### 076: Objects in OCaml
**Source:** Real World OCaml — https://dev.realworldocaml.org/objects.html
**Topic:** Object-oriented features in OCaml
**Difficulty:** Advanced
**Category:** objects
**OCaml:**
```ocaml
class point x_init y_init = object (self)
  val mutable x = x_init
  val mutable y = y_init
  method get_x = x
  method get_y = y
  method move dx dy = x <- x + dx; y <- y + dy
  method distance_to (other : point) =
    let dx = float_of_int (x - other#get_x) in
    let dy = float_of_int (y - other#get_y) in
    sqrt (dx *. dx +. dy *. dy)
  method to_string = Printf.sprintf "(%d, %d)" x y
end

let p1 = new point 0 0
let p2 = new point 3 4
let () =
  Printf.printf "Distance: %.1f\n" (p1#distance_to p2);
  p1#move 1 1;
  Printf.printf "p1 moved to %s\n" p1#to_string
```
**Status:** [ ]

### 077: Lazy Evaluation
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/ds/streams.html
**Topic:** Delay computation with lazy values
**Difficulty:** Intermediate
**Category:** lazy-evaluation
**OCaml:**
```ocaml
let expensive_computation () =
  Printf.printf "Computing...\n";
  List.init 1000 Fun.id |> List.fold_left ( + ) 0

let lazy_val = lazy (expensive_computation ())

let () =
  Printf.printf "Before force\n";
  let v1 = Lazy.force lazy_val in
  Printf.printf "First: %d\n" v1;
  let v2 = Lazy.force lazy_val in  (* cached, no recomputation *)
  Printf.printf "Second: %d\n" v2
```
**Status:** [ ]

### 078: Mutual Recursion
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/algebraic_data_types.html
**Topic:** Define mutually recursive functions with 'and'
**Difficulty:** Intermediate
**Category:** recursion
**OCaml:**
```ocaml
let rec is_even n =
  if n = 0 then true
  else is_odd (n - 1)
and is_odd n =
  if n = 0 then false
  else is_even (n - 1)

(* Mutually recursive types *)
type 'a tree = Node of 'a * 'a forest
and 'a forest = 'a tree list

let rec tree_size (Node (_, children)) = 1 + forest_size children
and forest_size = function
  | [] -> 0
  | t :: ts -> tree_size t + forest_size ts

let t = Node (1, [Node (2, []); Node (3, [Node (4, [])])])
let () = Printf.printf "Tree size: %d\n" (tree_size t)
```
**Status:** [ ]

### 079: Pipe Operator and Function Composition
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/hop/pipelining.html
**Topic:** Chain operations with |> and compose functions
**Difficulty:** Beginner
**Category:** higher-order
**OCaml:**
```ocaml
let ( >> ) f g x = g (f x)

let process =
  String.split_on_char ' '
  >> List.filter (fun s -> s <> "")
  >> List.map String.uppercase_ascii
  >> List.sort String.compare
  >> String.concat ", "

let result = process "  the quick  brown fox  "
let () = Printf.printf "Result: %s\n" result

(* With pipe operator *)
let result2 =
  [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]
  |> List.filter (fun x -> x mod 2 = 0)
  |> List.map (fun x -> x * x)
  |> List.fold_left ( + ) 0
let () = Printf.printf "Sum of even squares: %d\n" result2
```
**Status:** [ ]

### 080: Recursive Descent Parser
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/interp/parsing.html
**Topic:** Build a simple arithmetic expression parser
**Difficulty:** Advanced
**Category:** parsing
**OCaml:**
```ocaml
type expr = Num of int | Add of expr * expr | Mul of expr * expr

let rec parse_expr tokens =
  let (left, rest) = parse_term tokens in
  match rest with
  | "+" :: rest2 ->
    let (right, rest3) = parse_expr rest2 in
    (Add (left, right), rest3)
  | _ -> (left, rest)
and parse_term tokens =
  let (left, rest) = parse_atom tokens in
  match rest with
  | "*" :: rest2 ->
    let (right, rest3) = parse_term rest2 in
    (Mul (left, right), rest3)
  | _ -> (left, rest)
and parse_atom = function
  | n :: rest -> (Num (int_of_string n), rest)
  | [] -> failwith "unexpected end"

let rec eval = function
  | Num n -> n | Add (a,b) -> eval a + eval b | Mul (a,b) -> eval a * eval b

let tokens = String.split_on_char ' ' "2 + 3 * 4"
let (ast, _) = parse_expr tokens
let () = Printf.printf "2 + 3 * 4 = %d\n" (eval ast)
```
**Status:** [ ]

### 081: Queue Implemented with Two Stacks
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/ds/amortized.html
**Topic:** Amortized O(1) queue using two lists
**Difficulty:** Intermediate
**Category:** data-structures
**OCaml:**
```ocaml
type 'a queue = { front: 'a list; back: 'a list }

let empty = { front = []; back = [] }

let enqueue x q = { q with back = x :: q.back }

let dequeue q = match q.front with
  | x :: front -> Some (x, { q with front })
  | [] -> match List.rev q.back with
    | [] -> None
    | x :: front -> Some (x, { front; back = [] })

let of_list lst = { front = lst; back = [] }

let q = empty |> enqueue 1 |> enqueue 2 |> enqueue 3
let () = match dequeue q with
  | Some (x, _) -> Printf.printf "Front: %d\n" x
  | None -> print_endline "Empty"
```
**Status:** [ ]

### 082: Association List to Map Conversion
**Source:** Real World OCaml — https://dev.realworldocaml.org/maps-and-hashtables.html
**Topic:** Convert between association lists and maps
**Difficulty:** Intermediate
**Category:** data-structures
**OCaml:**
```ocaml
module SMap = Map.Make(String)

let alist_to_map lst =
  List.fold_left (fun m (k, v) -> SMap.add k v m) SMap.empty lst

let map_to_alist m = SMap.bindings m

let data = [("name", "Alice"); ("city", "Amsterdam"); ("lang", "OCaml")]
let m = alist_to_map data
let () = SMap.iter (fun k v -> Printf.printf "%s: %s\n" k v) m

(* Update and convert back *)
let m2 = SMap.add "year" "2024" m |> SMap.remove "city"
let pairs = map_to_alist m2
let () = List.iter (fun (k,v) -> Printf.printf "%s=%s " k v) pairs
```
**Status:** [ ]

### 083: Phantom Types for Unit Safety
**Source:** Real World OCaml — https://dev.realworldocaml.org/gadts.html
**Topic:** Use phantom types to prevent unit confusion
**Difficulty:** Advanced
**Category:** type-safety
**OCaml:**
```ocaml
type meters
type seconds
type _ quantity = Q of float

let meters (x : float) : meters quantity = Q x
let seconds (x : float) : seconds quantity = Q x

let add (Q a : 'a quantity) (Q b : 'a quantity) : 'a quantity = Q (a +. b)
let scale (Q a : 'a quantity) (f : float) : 'a quantity = Q (a *. f)
let value (Q x : _ quantity) = x

let d1 = meters 100.0
let d2 = meters 50.0
let total = add d1 d2  (* OK: same units *)
let () = Printf.printf "Total: %.1f meters\n" (value total)

(* let bad = add d1 (seconds 5.0)  (* Type error! *) *)
```
**Status:** [ ]

### 084: Continuation Passing Style (CPS)
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/hop/higher_order.html
**Topic:** Transform functions to continuation-passing style
**Difficulty:** Advanced
**Category:** higher-order
**OCaml:**
```ocaml
(* Direct style *)
let rec factorial n =
  if n <= 1 then 1 else n * factorial (n - 1)

(* CPS style — always tail-recursive *)
let factorial_cps n =
  let rec aux n k =
    if n <= 1 then k 1
    else aux (n - 1) (fun result -> k (n * result))
  in aux n Fun.id

(* CPS tree traversal *)
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

let rec sum_cps t k = match t with
  | Leaf -> k 0
  | Node (l, v, r) ->
    sum_cps l (fun sl -> sum_cps r (fun sr -> k (sl + v + sr)))

let () = Printf.printf "5! = %d\n" (factorial_cps 5)
```
**Status:** [ ]

### 085: Currying and Partial Application
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/hop/currying.html
**Topic:** Curried functions and partial application patterns
**Difficulty:** Beginner
**Category:** higher-order
**OCaml:**
```ocaml
let add x y = x + y
let add5 = add 5
let () = Printf.printf "add5 3 = %d\n" (add5 3)

let multiply x y = x * y
let double = multiply 2
let triple = multiply 3

let clamp ~min ~max x =
  if x < min then min else if x > max then max else x

let clamp_percent = clamp ~min:0 ~max:100

let results = List.map clamp_percent [-5; 42; 150; 99]
let () = List.iter (fun x -> Printf.printf "%d " x) results
```
**Status:** [ ]

### 086: Let Bindings and Scope
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/basics/expressions.html
**Topic:** Understanding let expressions and lexical scope
**Difficulty:** Beginner
**Category:** basics
**OCaml:**
```ocaml
(* let..in creates a local scope *)
let area_of_ring ~inner ~outer =
  let pi = Float.pi in
  let sq r = r *. r in
  pi *. (sq outer -. sq inner)

(* Shadowing, not mutation *)
let x = 5
let x = x + 1  (* new binding, old x is shadowed *)
let () = Printf.printf "x = %d\n" x  (* 6 *)

(* Nested let..in *)
let hypotenuse a b =
  let a2 = a *. a in
  let b2 = b *. b in
  sqrt (a2 +. b2)

let () = Printf.printf "Ring area: %.2f\n" (area_of_ring ~inner:3.0 ~outer:5.0)
let () = Printf.printf "Hypotenuse: %.2f\n" (hypotenuse 3.0 4.0)
```
**Status:** [ ]

### 087: Labeled and Optional Arguments
**Source:** Real World OCaml — https://dev.realworldocaml.org/variables-and-functions.html
**Topic:** Named parameters and default values
**Difficulty:** Intermediate
**Category:** functions
**OCaml:**
```ocaml
let create_greeting ?(title="Mr.") ?(greeting="Hello") ~name () =
  Printf.sprintf "%s, %s %s!" greeting title name

let () =
  print_endline (create_greeting ~name:"Smith" ());
  print_endline (create_greeting ~title:"Dr." ~name:"Jones" ());
  print_endline (create_greeting ~greeting:"Dear" ~title:"Prof." ~name:"Lee" ())

(* Optional with default *)
let pad ?(char=' ') ?(width=20) s =
  let len = String.length s in
  if len >= width then s
  else s ^ String.make (width - len) char

let () = Printf.printf "[%s]\n" (pad "hello")
let () = Printf.printf "[%s]\n" (pad ~char:'.' ~width:15 "hello")
```
**Status:** [ ]

### 088: Type Annotations and Constraints
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/basics/functions.html
**Topic:** Explicit type annotations in OCaml
**Difficulty:** Beginner
**Category:** basics
**OCaml:**
```ocaml
(* Parameter annotations *)
let add (x : int) (y : int) : int = x + y

(* Return type annotation *)
let divide (x : float) (y : float) : float option =
  if y = 0.0 then None else Some (x /. y)

(* Polymorphic annotation *)
let first (pair : 'a * 'b) : 'a = fst pair
let swap (x : 'a) (y : 'b) : 'b * 'a = (y, x)

(* Type alias *)
type point = float * float
type vector = float * float

let translate ((px, py) : point) ((vx, vy) : vector) : point =
  (px +. vx, py +. vy)

let () =
  let p = translate (1.0, 2.0) (3.0, 4.0) in
  Printf.printf "(%.1f, %.1f)\n" (fst p) (snd p)
```
**Status:** [ ]

### 089: Imperative Programming — While Loops and Refs
**Source:** Real World OCaml — https://dev.realworldocaml.org/imperative-programming.html
**Topic:** Imperative constructs: while, for, refs
**Difficulty:** Intermediate
**Category:** imperative
**OCaml:**
```ocaml
(* GCD using while loop *)
let gcd a b =
  let a = ref (abs a) and b = ref (abs b) in
  while !b <> 0 do
    let t = !b in
    b := !a mod !b;
    a := t
  done;
  !a

(* Collatz sequence length *)
let collatz_length n =
  let n = ref n and steps = ref 0 in
  while !n <> 1 do
    if !n mod 2 = 0 then n := !n / 2
    else n := 3 * !n + 1;
    incr steps
  done;
  !steps

let () =
  Printf.printf "gcd(48, 36) = %d\n" (gcd 48 36);
  Printf.printf "collatz(27) = %d steps\n" (collatz_length 27)
```
**Status:** [ ]

### 090: Recursive Types — Expression Tree
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/interp/substitution.html
**Topic:** Define and evaluate a recursive expression type
**Difficulty:** Intermediate
**Category:** algebraic-types
**OCaml:**
```ocaml
type expr =
  | Lit of float
  | Var of string
  | Add of expr * expr
  | Mul of expr * expr
  | Neg of expr

let rec eval env = function
  | Lit n -> n
  | Var x -> List.assoc x env
  | Add (a, b) -> eval env a +. eval env b
  | Mul (a, b) -> eval env a *. eval env b
  | Neg e -> -.(eval env e)

let rec to_string = function
  | Lit n -> Printf.sprintf "%.0f" n
  | Var x -> x
  | Add (a, b) -> Printf.sprintf "(%s + %s)" (to_string a) (to_string b)
  | Mul (a, b) -> Printf.sprintf "(%s * %s)" (to_string a) (to_string b)
  | Neg e -> Printf.sprintf "(-%s)" (to_string e)

let e = Add (Mul (Var "x", Lit 2.0), Lit 3.0)
let () = Printf.printf "%s = %.0f\n" (to_string e) (eval [("x", 5.0)] e)
```
**Status:** [ ]

### 091: Tuple Patterns and Destructuring
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/pattern_matching.html
**Topic:** Pattern match on tuples and nested structures
**Difficulty:** Beginner
**Category:** pattern-matching
**OCaml:**
```ocaml
let distance (x1, y1) (x2, y2) =
  let dx = x2 -. x1 and dy = y2 -. y1 in
  sqrt (dx *. dx +. dy *. dy)

let classify_point = function
  | (0.0, 0.0) -> "origin"
  | (x, 0.0) -> Printf.sprintf "x-axis at %.1f" x
  | (0.0, y) -> Printf.sprintf "y-axis at %.1f" y
  | (x, y) -> Printf.sprintf "(%.1f, %.1f)" x y

let min_max (a, b) = if a <= b then (a, b) else (b, a)

let () =
  Printf.printf "Distance: %.2f\n" (distance (0.0, 0.0) (3.0, 4.0));
  Printf.printf "%s\n" (classify_point (3.0, 0.0));
  let (lo, hi) = min_max (42, 17) in
  Printf.printf "min=%d max=%d\n" lo hi
```
**Status:** [ ]

### 092: Nested Pattern Matching
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/pattern_matching.html
**Topic:** Complex nested patterns with guards
**Difficulty:** Intermediate
**Category:** pattern-matching
**OCaml:**
```ocaml
type card = { suit: string; rank: int }

let card_name c = match c.rank with
  | 1 -> "Ace of " ^ c.suit
  | 11 -> "Jack of " ^ c.suit
  | 12 -> "Queen of " ^ c.suit
  | 13 -> "King of " ^ c.suit
  | n -> string_of_int n ^ " of " ^ c.suit

let compare_hands h1 h2 = match (h1, h2) with
  | ([], []) -> 0
  | ([], _) -> -1
  | (_, []) -> 1
  | (c1 :: _, c2 :: _) when c1.rank <> c2.rank -> compare c2.rank c1.rank
  | (_ :: rest1, _ :: rest2) -> compare_hands rest1 rest2

let hand = [{ suit="Hearts"; rank=13 }; { suit="Spades"; rank=1 }]
let () = List.iter (fun c -> Printf.printf "%s\n" (card_name c)) hand
```
**Status:** [ ]

### 093: Sequence — Custom Generators
**Source:** OCaml Standard Library
**Topic:** Build custom sequence generators
**Difficulty:** Intermediate
**Category:** stdlib-seq
**OCaml:**
```ocaml
(* Primes via sieve *)
let primes =
  let rec sieve s () = match s () with
    | Seq.Nil -> Seq.Nil
    | Seq.Cons (p, rest) ->
      Seq.Cons (p, sieve (Seq.filter (fun n -> n mod p <> 0) rest))
  in
  sieve (Seq.unfold (fun n -> Some (n, n+1)) 2)

let first_20_primes = primes |> Seq.take 20 |> List.of_seq
let () =
  List.iter (fun p -> Printf.printf "%d " p) first_20_primes;
  print_newline ()
```
**Status:** [ ]

### 094: Custom Iterators with Sequences
**Source:** OCaml Standard Library
**Topic:** Range and step iterators using Seq
**Difficulty:** Intermediate
**Category:** stdlib-seq
**OCaml:**
```ocaml
let range ?(step=1) start stop =
  Seq.unfold (fun i ->
    if (step > 0 && i < stop) || (step < 0 && i > stop)
    then Some (i, i + step)
    else None
  ) start

let () =
  range 0 10 |> Seq.iter (fun x -> Printf.printf "%d " x);
  print_newline ();
  range ~step:2 0 20 |> Seq.iter (fun x -> Printf.printf "%d " x);
  print_newline ();
  range ~step:(-3) 30 0 |> Seq.iter (fun x -> Printf.printf "%d " x);
  print_newline ()
```
**Status:** [ ]

### 095: Map.Make — Group By Key
**Source:** OCaml Standard Library
**Topic:** Group list elements by a key function using Map
**Difficulty:** Intermediate
**Category:** stdlib-map
**OCaml:**
```ocaml
module SMap = Map.Make(String)

let group_by key_fn lst =
  List.fold_left (fun m x ->
    let k = key_fn x in
    let existing = try SMap.find k m with Not_found -> [] in
    SMap.add k (x :: existing) m
  ) SMap.empty lst

let words = ["apple"; "banana"; "avocado"; "blueberry"; "cherry"; "apricot"]
let grouped = group_by (fun s -> String.make 1 s.[0]) words
let () = SMap.iter (fun k vs ->
  Printf.printf "%s: %s\n" k (String.concat ", " vs)
) grouped
```
**Status:** [ ]

### 096: Hashtbl — LRU Cache Pattern
**Source:** OCaml Standard Library
**Topic:** Simple memoization with Hashtbl
**Difficulty:** Intermediate
**Category:** stdlib-hashtbl
**OCaml:**
```ocaml
let memoize f =
  let cache = Hashtbl.create 16 in
  fun x ->
    match Hashtbl.find_opt cache x with
    | Some v -> v
    | None ->
      let v = f x in
      Hashtbl.add cache x v;
      v

let rec fib_slow n =
  if n <= 1 then n else fib_slow (n-1) + fib_slow (n-2)

(* Need explicit rec + memo for recursive memoization *)
let fib =
  let cache = Hashtbl.create 64 in
  let rec f n =
    match Hashtbl.find_opt cache n with
    | Some v -> v
    | None ->
      let v = if n <= 1 then n else f (n-1) + f (n-2) in
      Hashtbl.add cache n v; v
  in f

let () = Printf.printf "fib(40) = %d\n" (fib 40)
```
**Status:** [ ]

### 097: List — Zip and Unzip
**Source:** OCaml Standard Library
**Topic:** Combine and split parallel lists
**Difficulty:** Beginner
**Category:** stdlib-list
**OCaml:**
```ocaml
let rec zip l1 l2 = match (l1, l2) with
  | ([], _) | (_, []) -> []
  | (x :: xs, y :: ys) -> (x, y) :: zip xs ys

let unzip lst =
  List.fold_right (fun (a, b) (la, lb) -> (a :: la, b :: lb)) lst ([], [])

let names = ["Alice"; "Bob"; "Carol"]
let scores = [95; 87; 92]
let paired = zip names scores
let () = List.iter (fun (n, s) -> Printf.printf "%s: %d\n" n s) paired

let (ns, ss) = unzip paired
let () = Printf.printf "Names: %s\n" (String.concat ", " ns)
```
**Status:** [ ]

### 098: List — Scan (Running Accumulation)
**Source:** OCaml Standard Library
**Topic:** Compute running totals with scan
**Difficulty:** Intermediate
**Category:** stdlib-list
**OCaml:**
```ocaml
(* scan_left: like fold but keeps all intermediate results *)
let scan_left f init lst =
  let rec aux acc last = function
    | [] -> List.rev acc
    | x :: xs ->
      let next = f last x in
      aux (next :: acc) next xs
  in List.rev (init :: List.rev (aux [] init lst))

let running_sum = scan_left ( + ) 0 [1; 2; 3; 4; 5]
let running_max = scan_left max min_int [3; 1; 4; 1; 5; 9; 2; 6]

let () =
  Printf.printf "Running sum: %s\n"
    (String.concat " " (List.map string_of_int running_sum));
  Printf.printf "Running max: %s\n"
    (String.concat " " (List.map string_of_int running_max))
```
**Status:** [ ]

### 099: Recursive Descent — JSON-like Parser
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/interp/parsing.html
**Topic:** Parse a simplified JSON structure
**Difficulty:** Advanced
**Category:** parsing
**OCaml:**
```ocaml
type json =
  | JNull | JBool of bool | JNum of float
  | JStr of string | JList of json list

let rec json_to_string = function
  | JNull -> "null"
  | JBool b -> string_of_bool b
  | JNum n -> Printf.sprintf "%.0f" n
  | JStr s -> "\"" ^ s ^ "\""
  | JList lst ->
    "[" ^ String.concat ", " (List.map json_to_string lst) ^ "]"

let example = JList [JNum 1.0; JStr "hello"; JBool true; JNull;
                     JList [JNum 2.0; JNum 3.0]]
let () = Printf.printf "%s\n" (json_to_string example)
```
**Status:** [ ]

### 100: Applicative Functor Pattern
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/ds/monads.html
**Topic:** Applicative style for combining optional computations
**Difficulty:** Advanced
**Category:** functional-patterns
**OCaml:**
```ocaml
(* Option as applicative *)
let ( <$> ) f x = Option.map f x
let ( <*> ) f x = match f with
  | None -> None
  | Some g -> Option.map g x

let safe_div x y = if y = 0 then None else Some (x / y)

let result =
  (fun a b c -> a + b + c)
  <$> Some 10
  <*> Some 20
  <*> Some 30

let () = match result with
  | Some n -> Printf.printf "Sum: %d\n" n
  | None -> print_endline "Failed"

(* Validate multiple fields *)
let parse name age =
  (fun n a -> (n, a))
  <$> (if name <> "" then Some name else None)
  <*> (if age > 0 && age < 150 then Some age else None)

let () = match parse "Alice" 30 with
  | Some (n, a) -> Printf.printf "%s is %d\n" n a
  | None -> print_endline "Invalid"
```
**Status:** [ ]

### 101: Monad Pattern — Option and Result
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/ds/monads.html
**Topic:** Monadic bind for chaining fallible computations
**Difficulty:** Advanced
**Category:** functional-patterns
**OCaml:**
```ocaml
(* Option monad *)
let ( >>= ) = Option.bind
let return x = Some x

let lookup_user id =
  if id = 1 then Some "Alice" else None

let lookup_email name =
  if name = "Alice" then Some "alice@example.com" else None

let get_email id =
  lookup_user id >>= lookup_email

let () = match get_email 1 with
  | Some e -> Printf.printf "Email: %s\n" e
  | None -> print_endline "Not found"

(* Result monad *)
let ( let* ) = Result.bind
let validate_age age =
  let* a = if age > 0 then Ok age else Error "non-positive" in
  let* _ = if a < 150 then Ok () else Error "too old" in
  Ok a

let () = match validate_age 25 with
  | Ok a -> Printf.printf "Valid age: %d\n" a
  | Error e -> Printf.printf "Error: %s\n" e
```
**Status:** [ ]

### 102: Binding Operators (let* syntax)
**Source:** Real World OCaml — https://dev.realworldocaml.org/error-handling.html
**Topic:** Modern OCaml binding operators for monadic code
**Difficulty:** Intermediate
**Category:** functional-patterns
**OCaml:**
```ocaml
(* Define binding operators for Option *)
let ( let* ) = Option.bind
let ( let+ ) x f = Option.map f x
let ( and+ ) a b = match (a, b) with
  | (Some x, Some y) -> Some (x, y)
  | _ -> None

let parse_pair s1 s2 =
  let+ (a, b) = int_of_string_opt s1 and+ int_of_string_opt s2 in
  a + b

(* Result binding operators *)
module ResultSyntax = struct
  let ( let* ) = Result.bind
  let ( let+ ) x f = Result.map f x
end

let () =
  let open ResultSyntax in
  let result =
    let* x = Ok 10 in
    let* y = Ok 20 in
    let+ z = Ok 30 in
    x + y + z
  in
  match result with Ok n -> Printf.printf "%d\n" n | Error _ -> ()
```
**Status:** [ ]

### 103: Functor — Custom Collection with Map
**Source:** Real World OCaml — https://dev.realworldocaml.org/functors.html
**Topic:** Build a generic interval module using functors
**Difficulty:** Advanced
**Category:** functors
**OCaml:**
```ocaml
module type BOUNDED = sig
  type t
  val compare : t -> t -> int
  val to_string : t -> string
end

module MakeInterval (B : BOUNDED) = struct
  type t = Empty | Range of B.t * B.t

  let create lo hi =
    if B.compare lo hi > 0 then Empty else Range (lo, hi)

  let contains iv x = match iv with
    | Empty -> false
    | Range (lo, hi) -> B.compare x lo >= 0 && B.compare x hi <= 0

  let to_string = function
    | Empty -> "empty"
    | Range (lo, hi) -> Printf.sprintf "[%s, %s]" (B.to_string lo) (B.to_string hi)
end

module IntInterval = MakeInterval(struct
  type t = int
  let compare = compare
  let to_string = string_of_int
end)

let iv = IntInterval.create 1 10
let () = Printf.printf "%s contains 5: %b\n"
  (IntInterval.to_string iv) (IntInterval.contains iv 5)
```
**Status:** [ ]

### 104: Imperative — Ring Buffer
**Source:** Real World OCaml — https://dev.realworldocaml.org/imperative-programming.html
**Topic:** Circular buffer with mutable arrays
**Difficulty:** Intermediate
**Category:** data-structures
**OCaml:**
```ocaml
type 'a ring_buffer = {
  mutable data : 'a array;
  mutable head : int;
  mutable size : int;
  capacity : int;
}

let create capacity default = {
  data = Array.make capacity default;
  head = 0; size = 0; capacity
}

let push rb x =
  let idx = (rb.head + rb.size) mod rb.capacity in
  rb.data.(idx) <- x;
  if rb.size < rb.capacity then rb.size <- rb.size + 1
  else rb.head <- (rb.head + 1) mod rb.capacity

let to_list rb =
  List.init rb.size (fun i -> rb.data.((rb.head + i) mod rb.capacity))

let rb = create 5 0
let () = List.iter (push rb) [1;2;3;4;5;6;7]
let () = List.iter (fun x -> Printf.printf "%d " x) (to_list rb)
(* Output: 3 4 5 6 7 *)
```
**Status:** [ ]

### 105: String Processing — Simple Lexer
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/interp/parsing.html
**Topic:** Tokenize a string into a list of tokens
**Difficulty:** Intermediate
**Category:** parsing
**OCaml:**
```ocaml
type token = TInt of int | TOp of char | TLParen | TRParen

let is_digit c = c >= '0' && c <= '9'
let is_op c = c = '+' || c = '-' || c = '*' || c = '/'

let tokenize s =
  let n = String.length s in
  let rec aux i acc =
    if i >= n then List.rev acc
    else if s.[i] = ' ' then aux (i+1) acc
    else if s.[i] = '(' then aux (i+1) (TLParen :: acc)
    else if s.[i] = ')' then aux (i+1) (TRParen :: acc)
    else if is_op s.[i] then aux (i+1) (TOp s.[i] :: acc)
    else if is_digit s.[i] then
      let j = ref i in
      while !j < n && is_digit s.[!j] do incr j done;
      aux !j (TInt (int_of_string (String.sub s i (!j - i))) :: acc)
    else failwith (Printf.sprintf "unexpected: %c" s.[i])
  in aux 0 []

let tokens = tokenize "(42 + 3) * 7"
let () = Printf.printf "%d tokens\n" (List.length tokens)
```
**Status:** [ ]

### 106: Recursive Types — Symbolic Differentiation
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/algebraic_data_types.html
**Topic:** Symbolic math with algebraic data types
**Difficulty:** Advanced
**Category:** algebraic-types
**OCaml:**
```ocaml
type expr = X | Const of float | Add of expr * expr
          | Mul of expr * expr | Pow of expr * float

let rec deriv = function
  | X -> Const 1.0
  | Const _ -> Const 0.0
  | Add (a, b) -> Add (deriv a, deriv b)
  | Mul (a, b) -> Add (Mul (deriv a, b), Mul (a, deriv b))
  | Pow (e, n) -> Mul (Mul (Const n, Pow (e, n -. 1.0)), deriv e)

let rec simplify = function
  | Add (Const 0.0, e) | Add (e, Const 0.0) -> simplify e
  | Mul (Const 0.0, _) | Mul (_, Const 0.0) -> Const 0.0
  | Mul (Const 1.0, e) | Mul (e, Const 1.0) -> simplify e
  | Add (a, b) -> Add (simplify a, simplify b)
  | Mul (a, b) -> Mul (simplify a, simplify b)
  | e -> e

let rec to_s = function
  | X -> "x" | Const n -> Printf.sprintf "%.0f" n
  | Add (a,b) -> Printf.sprintf "(%s + %s)" (to_s a) (to_s b)
  | Mul (a,b) -> Printf.sprintf "(%s * %s)" (to_s a) (to_s b)
  | Pow (e,n) -> Printf.sprintf "%s^%.0f" (to_s e) n

(* d/dx (x^2 + 3x) *)
let expr = Add (Pow (X, 2.0), Mul (Const 3.0, X))
let d = deriv expr |> simplify
let () = Printf.printf "d/dx %s = %s\n" (to_s expr) (to_s d)
```
**Status:** [ ]

### 107: Unit Type and Side Effects
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/basics/printing.html
**Topic:** Understanding unit type and sequencing side effects
**Difficulty:** Beginner
**Category:** basics
**OCaml:**
```ocaml
(* Unit is the type of side effects *)
let greet name =
  Printf.printf "Hello, %s!\n" name;
  Printf.printf "Welcome to OCaml.\n"
  (* returns unit *)

let () = greet "World"

(* Semicolons sequence unit expressions *)
let count_down n =
  for i = n downto 1 do
    Printf.printf "%d... " i
  done;
  print_endline "Go!"

let () = count_down 5

(* ignore discards non-unit values *)
let () = ignore (1 + 2)
```
**Status:** [ ]

### 108: Map with Accumulator — Numbering Elements
**Source:** OCaml Standard Library
**Topic:** Stateful mapping patterns
**Difficulty:** Intermediate
**Category:** stdlib-list
**OCaml:**
```ocaml
let number_list lst =
  let rec aux n = function
    | [] -> []
    | x :: xs -> (n, x) :: aux (n + 1) xs
  in aux 1 lst

let indexed = number_list ["alpha"; "beta"; "gamma"; "delta"]
let () = List.iter (fun (i, s) -> Printf.printf "%d. %s\n" i s) indexed

(* Using fold for running stats *)
let running_avg lst =
  let (_, avgs) = List.fold_left (fun (sum, acc) x ->
    let sum' = sum +. x in
    let n = float_of_int (List.length acc + 1) in
    (sum', acc @ [sum' /. n])
  ) (0.0, []) lst
  in avgs

let avgs = running_avg [10.0; 20.0; 30.0; 40.0]
let () = List.iter (fun x -> Printf.printf "%.1f " x) avgs
```
**Status:** [ ]

### 109: Functors — Make Comparable Set with Pretty Printing
**Source:** Real World OCaml — https://dev.realworldocaml.org/functors.html
**Topic:** Combine functors with custom printing
**Difficulty:** Advanced
**Category:** functors
**OCaml:**
```ocaml
module type PRINTABLE_COMPARABLE = sig
  type t
  val compare : t -> t -> int
  val to_string : t -> string
end

module MakePrintableSet (E : PRINTABLE_COMPARABLE) = struct
  include Set.Make(E)
  let to_string s =
    "{" ^ String.concat ", " (List.map E.to_string (elements s)) ^ "}"
end

module PIntSet = MakePrintableSet(struct
  type t = int
  let compare = compare
  let to_string = string_of_int
end)

let s = PIntSet.of_list [3; 1; 4; 1; 5; 9]
let () = Printf.printf "Set: %s (size: %d)\n"
  (PIntSet.to_string s) (PIntSet.cardinal s)
```
**Status:** [ ]

### 110: Church Numerals
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/hop/higher_order.html
**Topic:** Encode natural numbers as higher-order functions
**Difficulty:** Advanced
**Category:** lambda-calculus
**OCaml:**
```ocaml
(* Church encoding of naturals *)
let zero _f x = x
let succ n f x = f (n f x)
let one = succ zero
let two = succ one
let three = succ two

let add m n f x = m f (n f x)
let mul m n f = m (n f)

let to_int n = n (fun x -> x + 1) 0

let five = add two three
let six = mul two three

let () =
  Printf.printf "2 + 3 = %d\n" (to_int five);
  Printf.printf "2 * 3 = %d\n" (to_int six);
  Printf.printf "3 + 3 = %d\n" (to_int (add three three))
```
**Status:** [ ]

### 111: List Comprehension via Bind
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/ds/monads.html
**Topic:** Simulate list comprehensions with monadic bind
**Difficulty:** Intermediate
**Category:** functional-patterns
**OCaml:**
```ocaml
let ( >>= ) lst f = List.concat_map f lst
let return x = [x]
let guard b = if b then [()] else []

(* Pythagorean triples *)
let triples n =
  List.init n (fun i -> i + 1) >>= fun a ->
  List.init n (fun i -> i + 1) >>= fun b ->
  List.init n (fun i -> i + 1) >>= fun c ->
  guard (a*a + b*b = c*c && a <= b) >>= fun () ->
  return (a, b, c)

let () = List.iter (fun (a,b,c) ->
  Printf.printf "(%d, %d, %d)\n" a b c
) (triples 20)
```
**Status:** [ ]

### 112: Error Handling — try/with and Option
**Source:** Real World OCaml — https://dev.realworldocaml.org/error-handling.html
**Topic:** Exception handling and conversion to Option/Result
**Difficulty:** Intermediate
**Category:** error-handling
**OCaml:**
```ocaml
(* Convert exception-throwing functions to Option *)
let try_with f x = try Some (f x) with _ -> None

let safe_int_of_string = try_with int_of_string
let safe_hd = try_with List.hd
let safe_find k = try_with (List.assoc k)

let () =
  (match safe_int_of_string "42" with
   | Some n -> Printf.printf "Parsed: %d\n" n
   | None -> print_endline "Failed");
  (match safe_find "x" [("x", 1); ("y", 2)] with
   | Some v -> Printf.printf "Found: %d\n" v
   | None -> print_endline "Not found")
```
**Status:** [ ]

### 113: Abstract Data Types — Rational Numbers
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/modules/encapsulation.html
**Topic:** Encapsulate representation with abstract types
**Difficulty:** Intermediate
**Category:** modules
**OCaml:**
```ocaml
module Rational : sig
  type t
  val make : int -> int -> t
  val add : t -> t -> t
  val mul : t -> t -> t
  val to_string : t -> string
end = struct
  type t = { num: int; den: int }

  let gcd a b =
    let rec aux a b = if b = 0 then a else aux b (a mod b) in
    aux (abs a) (abs b)

  let make n d =
    if d = 0 then failwith "zero denominator";
    let g = gcd n d in
    let sign = if d < 0 then -1 else 1 in
    { num = sign * n / g; den = sign * d / g }

  let add a b = make (a.num * b.den + b.num * a.den) (a.den * b.den)
  let mul a b = make (a.num * b.num) (a.den * b.den)
  let to_string r = Printf.sprintf "%d/%d" r.num r.den
end

let a = Rational.make 1 3
let b = Rational.make 1 6
let () = Printf.printf "%s + %s = %s\n"
  (Rational.to_string a) (Rational.to_string b)
  (Rational.to_string (Rational.add a b))
```
**Status:** [ ]

### 114: Sequence — Windowed Operations
**Source:** OCaml Standard Library
**Topic:** Sliding window over sequences
**Difficulty:** Intermediate
**Category:** stdlib-seq
**OCaml:**
```ocaml
let windows n seq =
  let buf = Array.make n 0 in
  let i = ref 0 in
  seq |> Seq.filter_map (fun x ->
    buf.(!i mod n) <- x;
    incr i;
    if !i >= n then
      Some (Array.to_list (Array.init n (fun j -> buf.((!i - n + j) mod n))))
    else None
  )

let data = List.to_seq [1; 2; 3; 4; 5; 6; 7]
let wins = windows 3 data |> List.of_seq
let () = List.iter (fun w ->
  Printf.printf "[%s] " (String.concat "," (List.map string_of_int w))
) wins
```
**Status:** [ ]

### 115: Polymorphic Functions — Generic Utilities
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/basics/functions.html
**Topic:** Write functions that work for any type
**Difficulty:** Beginner
**Category:** polymorphism
**OCaml:**
```ocaml
let compose f g x = f (g x)
let flip f x y = f y x
let const x _y = x
let tap f x = f x; x

let twice f x = f (f x)
let thrice f x = f (f (f x))

let () =
  let inc = ( + ) 1 in
  Printf.printf "twice inc 5 = %d\n" (twice inc 5);
  Printf.printf "thrice double 3 = %d\n" (thrice (( * ) 2) 3);

  let exclaim = (fun s -> s ^ "!") in
  Printf.printf "%s\n" (thrice exclaim "wow")
```
**Status:** [ ]

### 116: Recursive Data — Natural Numbers (Peano)
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/algebraic_data_types.html
**Topic:** Peano natural numbers as a recursive type
**Difficulty:** Intermediate
**Category:** algebraic-types
**OCaml:**
```ocaml
type nat = Zero | Succ of nat

let rec to_int = function
  | Zero -> 0
  | Succ n -> 1 + to_int n

let rec of_int = function
  | 0 -> Zero
  | n -> Succ (of_int (n - 1))

let rec add a b = match a with
  | Zero -> b
  | Succ a' -> Succ (add a' b)

let rec mul a b = match a with
  | Zero -> Zero
  | Succ a' -> add b (mul a' b)

let three = of_int 3
let four = of_int 4
let () = Printf.printf "3 + 4 = %d\n" (to_int (add three four))
let () = Printf.printf "3 * 4 = %d\n" (to_int (mul three four))
```
**Status:** [ ]

### 117: Higher-Order — Function as Return Value
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/hop/higher_order.html
**Topic:** Functions that return functions
**Difficulty:** Intermediate
**Category:** higher-order
**OCaml:**
```ocaml
let make_adder n = fun x -> x + n
let make_multiplier n = fun x -> x * n

let make_validator ~min ~max =
  fun x -> x >= min && x <= max

let is_valid_age = make_validator ~min:0 ~max:150
let is_valid_score = make_validator ~min:0 ~max:100

(* Function factory *)
let make_counter () =
  let n = ref 0 in
  fun () -> incr n; !n

let c1 = make_counter ()
let c2 = make_counter ()
let () =
  Printf.printf "c1: %d %d %d\n" (c1 ()) (c1 ()) (c1 ());
  Printf.printf "c2: %d %d\n" (c2 ()) (c2 ())
```
**Status:** [ ]

### 118: Records — Functional Update
**Source:** Real World OCaml — https://dev.realworldocaml.org/records.html
**Topic:** Non-destructive record updates with 'with'
**Difficulty:** Beginner
**Category:** records
**OCaml:**
```ocaml
type config = {
  host : string;
  port : int;
  debug : bool;
  max_connections : int;
  timeout_ms : int;
}

let default_config = {
  host = "localhost"; port = 8080;
  debug = false; max_connections = 100; timeout_ms = 5000
}

let dev_config = { default_config with debug = true; port = 3000 }
let prod_config = { default_config with
  host = "0.0.0.0"; max_connections = 10000; timeout_ms = 30000
}

let () = Printf.printf "Dev: %s:%d (debug=%b)\n"
  dev_config.host dev_config.port dev_config.debug
let () = Printf.printf "Prod: %s:%d (max=%d)\n"
  prod_config.host prod_config.port prod_config.max_connections
```
**Status:** [ ]

### 119: Recursive Descent — S-Expression Parser
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/interp/parsing.html
**Topic:** Parse S-expressions from a token stream
**Difficulty:** Advanced
**Category:** parsing
**OCaml:**
```ocaml
type sexp = Atom of string | List of sexp list

let tokenize s =
  let s = String.concat " ( " (String.split_on_char '(' s) in
  let s = String.concat " ) " (String.split_on_char ')' s) in
  String.split_on_char ' ' s |> List.filter (fun t -> t <> "")

let rec parse_sexp = function
  | [] -> failwith "unexpected end"
  | "(" :: rest ->
    let (items, rest) = parse_list rest in
    (List items, rest)
  | ")" :: _ -> failwith "unexpected )"
  | atom :: rest -> (Atom atom, rest)
and parse_list = function
  | ")" :: rest -> ([], rest)
  | tokens ->
    let (item, rest) = parse_sexp tokens in
    let (items, rest) = parse_list rest in
    (item :: items, rest)

let rec to_string = function
  | Atom s -> s
  | List l -> "(" ^ String.concat " " (List.map to_string l) ^ ")"

let (ast, _) = parse_sexp (tokenize "(define (square x) (* x x))")
let () = Printf.printf "%s\n" (to_string ast)
```
**Status:** [ ]

### 120: Map.Make — Frequency Count
**Source:** OCaml Standard Library
**Topic:** Count occurrences using Map
**Difficulty:** Intermediate
**Category:** stdlib-map
**OCaml:**
```ocaml
module CharMap = Map.Make(Char)

let char_freq s =
  String.fold_left (fun m c ->
    let n = match CharMap.find_opt c m with Some n -> n | None -> 0 in
    CharMap.add c (n + 1) m
  ) CharMap.empty s

let freq = char_freq "mississippi"
let sorted = CharMap.bindings freq
  |> List.sort (fun (_,a) (_,b) -> compare b a)

let () = List.iter (fun (c, n) ->
  Printf.printf "'%c': %d\n" c n
) sorted
```
**Status:** [ ]

### 121: Closures and Environments
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/hop/higher_order.html
**Topic:** Understanding closures and captured variables
**Difficulty:** Intermediate
**Category:** higher-order
**OCaml:**
```ocaml
(* A closure captures its environment *)
let make_greeting prefix suffix =
  fun name -> prefix ^ name ^ suffix

let hello = make_greeting "Hello, " "!"
let bye = make_greeting "Goodbye, " "."

let () =
  Printf.printf "%s\n" (hello "Alice");
  Printf.printf "%s\n" (bye "Bob")

(* Accumulator closure *)
let make_accumulator init =
  let total = ref init in
  fun amount ->
    total := !total + amount;
    !total

let acc = make_accumulator 100
let () = Printf.printf "Balance: %d %d %d\n" (acc 50) (acc (-30)) (acc 20)
```
**Status:** [ ]

### 122: Module Include and Open
**Source:** Real World OCaml — https://dev.realworldocaml.org/files-modules-and-programs.html
**Topic:** Extend modules with include and local open
**Difficulty:** Intermediate
**Category:** modules
**OCaml:**
```ocaml
module ExtList = struct
  include List

  let sum = fold_left ( + ) 0
  let product = fold_left ( * ) 1

  let take n lst =
    let rec aux n acc = function
      | [] -> List.rev acc
      | _ when n <= 0 -> List.rev acc
      | x :: xs -> aux (n-1) (x :: acc) xs
    in aux n [] lst

  let drop n lst =
    let rec aux n = function
      | [] -> []
      | _ :: xs as l -> if n <= 0 then l else aux (n-1) xs
    in aux n lst
end

let () =
  let data = [1;2;3;4;5;6;7;8;9;10] in
  Printf.printf "Sum: %d\n" (ExtList.sum data);
  Printf.printf "First 3: %s\n"
    (String.concat " " (List.map string_of_int (ExtList.take 3 data)))
```
**Status:** [ ]

### 123: Recursive Data — Red-Black Tree (simplified)
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/ds/rb.html
**Topic:** Balanced binary search tree with colors
**Difficulty:** Advanced
**Category:** data-structures
**OCaml:**
```ocaml
type color = Red | Black
type 'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree

let balance = function
  | (Black, T (Red, T (Red, a, x, b), y, c), z, d)
  | (Black, T (Red, a, x, T (Red, b, y, c)), z, d)
  | (Black, a, x, T (Red, T (Red, b, y, c), z, d))
  | (Black, a, x, T (Red, b, y, T (Red, c, z, d))) ->
    T (Red, T (Black, a, x, b), y, T (Black, c, z, d))
  | (c, l, v, r) -> T (c, l, v, r)

let insert x t =
  let rec ins = function
    | E -> T (Red, E, x, E)
    | T (c, l, v, r) ->
      if x < v then balance (c, ins l, v, r)
      else if x > v then balance (c, l, v, ins r)
      else T (c, l, v, r)
  in match ins t with T (_, l, v, r) -> T (Black, l, v, r) | E -> E

let tree = List.fold_left (fun t x -> insert x t) E [5;3;7;1;4;6;8;2]
let rec size = function E -> 0 | T(_,l,_,r) -> 1 + size l + size r
let () = Printf.printf "RB tree size: %d\n" (size tree)
```
**Status:** [ ]

### 124: Tail-Recursive Tree Traversal with CPS
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/ds/bst.html
**Topic:** Avoid stack overflow on deep trees with CPS
**Difficulty:** Advanced
**Category:** data-structures
**OCaml:**
```ocaml
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

let rec insert x = function
  | Leaf -> Node (Leaf, x, Leaf)
  | Node (l, v, r) ->
    if x < v then Node (insert x l, v, r)
    else Node (l, v, insert x r)

(* CPS inorder - tail recursive *)
let inorder t =
  let rec aux t k = match t with
    | Leaf -> k []
    | Node (l, v, r) ->
      aux r (fun right ->
        aux l (fun left ->
          k (left @ [v] @ right)))
  in aux t Fun.id

let t = List.fold_left (fun t x -> insert x t) Leaf [5;2;8;1;3;7;9]
let () = List.iter (fun x -> Printf.printf "%d " x) (inorder t)
```
**Status:** [ ]

### 125: Local Exceptions
**Source:** Real World OCaml — https://dev.realworldocaml.org/error-handling.html
**Topic:** Use local exceptions for control flow
**Difficulty:** Intermediate
**Category:** exceptions
**OCaml:**
```ocaml
(* Local exception for early return *)
let find_first pred lst =
  let exception Found of int in
  try
    List.iteri (fun i x -> if pred x then raise (Found i)) lst;
    None
  with Found i -> Some i

let idx = find_first (fun x -> x > 10) [3; 7; 12; 5; 20]
let () = match idx with
  | Some i -> Printf.printf "First > 10 at index %d\n" i
  | None -> print_endline "Not found"

(* Local exception for loop break *)
let sum_until_negative lst =
  let exception Stop in
  let total = ref 0 in
  (try List.iter (fun x ->
    if x < 0 then raise Stop;
    total := !total + x
  ) lst with Stop -> ());
  !total

let () = Printf.printf "Sum: %d\n" (sum_until_negative [1; 2; 3; -1; 5])
```
**Status:** [ ]

### 126: Recursive Types — Rose Tree
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/trees.html
**Topic:** Multi-way tree (rose tree) structure
**Difficulty:** Intermediate
**Category:** algebraic-types
**OCaml:**
```ocaml
type 'a rose = Rose of 'a * 'a rose list

let leaf x = Rose (x, [])

let rec depth (Rose (_, children)) =
  1 + List.fold_left (fun acc c -> max acc (depth c)) 0 children

let rec size (Rose (_, children)) =
  1 + List.fold_left (fun acc c -> acc + size c) 0 children

let rec map f (Rose (x, children)) =
  Rose (f x, List.map (map f) children)

let tree = Rose ("root", [
  Rose ("a", [leaf "a1"; leaf "a2"]);
  Rose ("b", [leaf "b1"]);
  leaf "c"
])

let () = Printf.printf "Depth: %d, Size: %d\n" (depth tree) (size tree)
```
**Status:** [ ]

### 127: Set Operations — Powerset
**Source:** OCaml Standard Library
**Topic:** Compute the powerset of a set
**Difficulty:** Advanced
**Category:** stdlib-set
**OCaml:**
```ocaml
module IntSet = Set.Make(Int)

let powerset s =
  IntSet.fold (fun x acc ->
    List.fold_left (fun acc2 subset ->
      IntSet.add x subset :: acc2
    ) acc acc
  ) s [IntSet.empty]

let s = IntSet.of_list [1; 2; 3]
let ps = powerset s
let () = List.iter (fun sub ->
  Printf.printf "{%s} "
    (IntSet.elements sub |> List.map string_of_int |> String.concat ",")
) ps
```
**Status:** [ ]

### 128: Hashtbl — Two-way Map (Bidirectional)
**Source:** OCaml Standard Library
**Topic:** Maintain a bidirectional mapping with two hash tables
**Difficulty:** Intermediate
**Category:** stdlib-hashtbl
**OCaml:**
```ocaml
type ('a, 'b) bimap = {
  forward : ('a, 'b) Hashtbl.t;
  backward : ('b, 'a) Hashtbl.t;
}

let create n = { forward = Hashtbl.create n; backward = Hashtbl.create n }

let add bm k v =
  Hashtbl.replace bm.forward k v;
  Hashtbl.replace bm.backward v k

let find_forward bm k = Hashtbl.find bm.forward k
let find_backward bm v = Hashtbl.find bm.backward v

let bm = create 8
let () =
  add bm "one" 1; add bm "two" 2; add bm "three" 3;
  Printf.printf "two -> %d\n" (find_forward bm "two");
  Printf.printf "3 -> %s\n" (find_backward bm 3)
```
**Status:** [ ]

### 129: Format — Pretty Printing with Boxes
**Source:** OCaml Standard Library
**Topic:** Use Format module for structured output
**Difficulty:** Intermediate
**Category:** stdlib-printf
**OCaml:**
```ocaml
let pp_list pp_item fmt lst =
  Format.fprintf fmt "[@[<hov 2>";
  List.iteri (fun i x ->
    if i > 0 then Format.fprintf fmt ";@ ";
    pp_item fmt x
  ) lst;
  Format.fprintf fmt "@]]"

let pp_int fmt n = Format.fprintf fmt "%d" n
let pp_string fmt s = Format.fprintf fmt "%S" s

let () =
  Format.printf "Numbers: %a@." (pp_list pp_int) [1;2;3;4;5;6;7;8;9;10];
  Format.printf "Words: %a@." (pp_list pp_string) ["hello"; "world"; "ocaml"]
```
**Status:** [ ]

### 130: Seq — Interleave and Round-Robin
**Source:** OCaml Standard Library
**Topic:** Merge multiple sequences by alternating
**Difficulty:** Intermediate
**Category:** stdlib-seq
**OCaml:**
```ocaml
let rec interleave s1 s2 () = match s1 () with
  | Seq.Nil -> s2 ()
  | Seq.Cons (x, rest) -> Seq.Cons (x, interleave s2 rest)

let s1 = List.to_seq [1; 3; 5; 7]
let s2 = List.to_seq [2; 4; 6; 8]
let merged = interleave s1 s2 |> List.of_seq
let () = List.iter (fun x -> Printf.printf "%d " x) merged
(* Output: 1 2 3 4 5 6 7 8 *)
```
**Status:** [ ]

### 131: Match Guards and Or-Patterns
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/pattern_matching.html
**Topic:** Pattern matching with when guards and combined patterns
**Difficulty:** Beginner
**Category:** pattern-matching
**OCaml:**
```ocaml
let classify_char = function
  | 'a' | 'e' | 'i' | 'o' | 'u'
  | 'A' | 'E' | 'I' | 'O' | 'U' -> "vowel"
  | c when c >= 'a' && c <= 'z' -> "consonant"
  | c when c >= 'A' && c <= 'Z' -> "consonant"
  | c when c >= '0' && c <= '9' -> "digit"
  | _ -> "other"

let fizzbuzz n = match (n mod 3, n mod 5) with
  | (0, 0) -> "FizzBuzz"
  | (0, _) -> "Fizz"
  | (_, 0) -> "Buzz"
  | _ -> string_of_int n

let () = List.init 20 (fun i -> i+1)
  |> List.iter (fun n -> Printf.printf "%s " (fizzbuzz n))
```
**Status:** [ ]

### 132: Recursive Types — Zipper for Lists
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/ds/zippers.html
**Topic:** Navigate a list with a zipper data structure
**Difficulty:** Advanced
**Category:** data-structures
**OCaml:**
```ocaml
type 'a zipper = { left: 'a list; focus: 'a; right: 'a list }

let of_list = function
  | [] -> failwith "empty"
  | x :: xs -> { left = []; focus = x; right = xs }

let move_right z = match z.right with
  | [] -> None
  | x :: xs -> Some { left = z.focus :: z.left; focus = x; right = xs }

let move_left z = match z.left with
  | [] -> None
  | x :: xs -> Some { left = xs; focus = x; right = z.focus :: z.right }

let modify f z = { z with focus = f z.focus }
let to_list z = List.rev z.left @ [z.focus] @ z.right

let z = of_list [1;2;3;4;5]
let z = Option.get (move_right z)  (* focus = 2 *)
let z = Option.get (move_right z)  (* focus = 3 *)
let z = modify (( * ) 10) z        (* focus = 30 *)
let () = List.iter (fun x -> Printf.printf "%d " x) (to_list z)
```
**Status:** [ ]

### 133: Anonymous Functions and Closures
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/basics/functions.html
**Topic:** Lambda expressions (fun keyword)
**Difficulty:** Beginner
**Category:** basics
**OCaml:**
```ocaml
(* Anonymous functions with fun *)
let apply f x = f x
let apply2 f x y = f x y

let () =
  Printf.printf "%d\n" (apply (fun x -> x * x) 5);
  Printf.printf "%d\n" (apply2 (fun x y -> x + y) 3 4);

  (* Multi-argument anonymous function *)
  let result = List.map (fun x -> x * x + 1) [1;2;3;4;5] in
  List.iter (fun x -> Printf.printf "%d " x) result;
  print_newline ();

  (* Nested anonymous functions *)
  let make_pair = fun x -> fun y -> (x, y) in
  let (a, b) = make_pair 1 2 in
  Printf.printf "(%d, %d)\n" a b
```
**Status:** [ ]

### 134: Recursive Data — Trie (Prefix Tree)
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/ds/hash_tables.html
**Topic:** Store strings in a prefix tree
**Difficulty:** Advanced
**Category:** data-structures
**OCaml:**
```ocaml
module CharMap = Map.Make(Char)

type trie = { is_end: bool; children: trie CharMap.t }
let empty = { is_end = false; children = CharMap.empty }

let insert word t =
  let rec aux i t =
    if i = String.length word then { t with is_end = true }
    else
      let c = word.[i] in
      let child = try CharMap.find c t.children with Not_found -> empty in
      { t with children = CharMap.add c (aux (i+1) child) t.children }
  in aux 0 t

let mem word t =
  let rec aux i t =
    if i = String.length word then t.is_end
    else match CharMap.find_opt word.[i] t.children with
      | None -> false | Some child -> aux (i+1) child
  in aux 0 t

let t = List.fold_left (fun t w -> insert w t) empty
  ["cat"; "car"; "card"; "care"; "bat"]
let () = List.iter (fun w ->
  Printf.printf "%s: %b\n" w (mem w t)
) ["cat"; "ca"; "car"; "care"; "dog"]
```
**Status:** [ ]

### 135: Modules — Private Types
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/modules/encapsulation.html
**Topic:** Hide constructors with private types in signatures
**Difficulty:** Intermediate
**Category:** modules
**OCaml:**
```ocaml
module PositiveInt : sig
  type t = private int
  val of_int : int -> t option
  val to_int : t -> int
  val add : t -> t -> t
end = struct
  type t = int
  let of_int n = if n > 0 then Some n else None
  let to_int n = n
  let add a b = a + b
end

let () = match PositiveInt.of_int 42 with
  | Some n ->
    Printf.printf "Positive: %d\n" (PositiveInt.to_int n);
    (* Can read as int: *)
    Printf.printf "As int: %d\n" (n :> int)
  | None -> print_endline "Not positive"
```
**Status:** [ ]

### 136: Effect Handlers Preview (OCaml 5)
**Source:** https://v2.ocaml.org/manual/effects.html
**Topic:** Algebraic effects for controlled side effects (OCaml 5+)
**Difficulty:** Advanced
**Category:** effects
**OCaml:**
```ocaml
(* Note: requires OCaml 5.0+ *)
(* This shows the concept; run on OCaml 5 *)
type _ Effect.t += Ask : string Effect.t
type _ Effect.t += Log : string -> unit Effect.t

let program () =
  let name = Effect.perform Ask in
  Effect.perform (Log ("Got name: " ^ name));
  Printf.printf "Hello, %s!\n" name

(* In OCaml 5 you'd install handlers:
   Effect.Deep.try_with program ()
   { effc = fun (type a) (eff : a Effect.t) ->
     match eff with
     | Ask -> Some (fun k -> Effect.Deep.continue k "World")
     | Log msg -> Some (fun k -> print_endline msg; Effect.Deep.continue k ())
     | _ -> None } *)

(* Simulated version for pre-5: *)
let () = Printf.printf "Hello, World! (effect simulation)\n"
```
**Status:** [ ]

### 137: Recursive Data — Graph as Adjacency List
**Source:** OCaml Standard Library
**Topic:** Represent and traverse graphs using maps
**Difficulty:** Intermediate
**Category:** data-structures
**OCaml:**
```ocaml
module Graph = struct
  module SMap = Map.Make(String)
  type t = string list SMap.t

  let empty = SMap.empty
  let add_edge g u v =
    let neighbors = try SMap.find u g with Not_found -> [] in
    SMap.add u (v :: neighbors) g

  let bfs g start =
    let visited = Hashtbl.create 16 in
    let queue = Queue.create () in
    Queue.add start queue;
    Hashtbl.add visited start true;
    let result = ref [] in
    while not (Queue.is_empty queue) do
      let node = Queue.pop queue in
      result := node :: !result;
      let neighbors = try SMap.find node g with Not_found -> [] in
      List.iter (fun n ->
        if not (Hashtbl.mem visited n) then begin
          Hashtbl.add visited n true;
          Queue.add n queue
        end
      ) neighbors
    done;
    List.rev !result
end

let g = Graph.empty
  |> Graph.add_edge "A" "B" |> Graph.add_edge "A" "C"
  |> Graph.add_edge "B" "D" |> Graph.add_edge "C" "D"
let () = List.iter (fun n -> Printf.printf "%s " n) (Graph.bfs g "A")
```
**Status:** [ ]

### 138: List — Interleave Two Lists
**Source:** OCaml Standard Library
**Topic:** Alternate elements from two lists
**Difficulty:** Beginner
**Category:** stdlib-list
**OCaml:**
```ocaml
let rec interleave l1 l2 = match (l1, l2) with
  | ([], l) | (l, []) -> l
  | (x :: xs, y :: ys) -> x :: y :: interleave xs ys

let transpose matrix =
  match matrix with
  | [] -> []
  | first :: _ ->
    List.mapi (fun i _ ->
      List.map (fun row -> List.nth row i) matrix
    ) first

let r = interleave [1;3;5] [2;4;6]
let () = List.iter (fun x -> Printf.printf "%d " x) r;
  print_newline ()

let t = transpose [[1;2;3]; [4;5;6]; [7;8;9]]
let () = List.iter (fun row ->
  List.iter (fun x -> Printf.printf "%d " x) row;
  print_newline ()
) t
```
**Status:** [ ]

### 139: Parametric Polymorphism — Generic Pair
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/data/algebraic_data_types.html
**Topic:** Generic types with type parameters
**Difficulty:** Beginner
**Category:** polymorphism
**OCaml:**
```ocaml
type ('a, 'b) either = Left of 'a | Right of 'b

let map_left f = function
  | Left x -> Left (f x)
  | Right y -> Right y

let map_right f = function
  | Left x -> Left x
  | Right y -> Right (f y)

let partition_either lst =
  List.fold_right (fun x (lefts, rights) -> match x with
    | Left l -> (l :: lefts, rights)
    | Right r -> (lefts, r :: rights)
  ) lst ([], [])

let items = [Left 1; Right "a"; Left 2; Right "b"; Left 3]
let (nums, strs) = partition_either items
let () = Printf.printf "Left: %s, Right: %s\n"
  (String.concat "," (List.map string_of_int nums))
  (String.concat "," strs)
```
**Status:** [ ]

### 140: Memoization with Lazy Values
**Source:** OCaml Standard Library
**Topic:** Use Lazy for one-shot expensive computations
**Difficulty:** Intermediate
**Category:** lazy-evaluation
**OCaml:**
```ocaml
(* Lazy Fibonacci stream *)
type 'a stream = Cons of 'a * 'a stream Lazy.t

let rec fibs_from a b = Cons (a, lazy (fibs_from b (a + b)))
let fibs = fibs_from 0 1

let rec take n (Cons (x, rest)) =
  if n <= 0 then []
  else x :: take (n - 1) (Lazy.force rest)

let rec nth n (Cons (x, rest)) =
  if n = 0 then x else nth (n - 1) (Lazy.force rest)

let () =
  Printf.printf "First 10 fibs: %s\n"
    (String.concat " " (List.map string_of_int (take 10 fibs)));
  Printf.printf "Fib(20) = %d\n" (nth 20 fibs)
```
**Status:** [ ]

### 141: Module Type Sharing Constraints
**Source:** Real World OCaml — https://dev.realworldocaml.org/functors.html
**Topic:** Share types between modules using with constraints
**Difficulty:** Advanced
**Category:** modules
**OCaml:**
```ocaml
module type KEY = sig
  type t
  val compare : t -> t -> int
  val to_string : t -> string
end

module type STORE = sig
  type key
  type 'a t
  val empty : 'a t
  val set : key -> 'a -> 'a t -> 'a t
  val get : key -> 'a t -> 'a option
end

module MakeStore (K : KEY) : STORE with type key = K.t = struct
  type key = K.t
  module M = Map.Make(K)
  type 'a t = 'a M.t
  let empty = M.empty
  let set k v m = M.add k v m
  let get k m = M.find_opt k m
end

module StringStore = MakeStore(struct
  type t = string let compare = String.compare
  let to_string s = s
end)

let s = StringStore.empty |> StringStore.set "key" 42
let () = match StringStore.get "key" s with
  | Some v -> Printf.printf "Found: %d\n" v
  | None -> ()
```
**Status:** [ ]

### 142: Array — Matrix Operations
**Source:** OCaml Standard Library
**Topic:** Basic matrix math with arrays
**Difficulty:** Intermediate
**Category:** stdlib-array
**OCaml:**
```ocaml
let mat_mul a b =
  let rows = Array.length a and cols = Array.length b.(0) in
  let k = Array.length b in
  Array.init rows (fun i ->
    Array.init cols (fun j ->
      let sum = ref 0 in
      for p = 0 to k - 1 do
        sum := !sum + a.(i).(p) * b.(p).(j)
      done;
      !sum
    )
  )

let print_mat m =
  Array.iter (fun row ->
    Array.iter (fun x -> Printf.printf "%3d " x) row;
    print_newline ()
  ) m

let a = [| [|1;2|]; [|3;4|] |]
let b = [| [|5;6|]; [|7;8|] |]
let c = mat_mul a b
let () = print_mat c
```
**Status:** [ ]

### 143: String — Regular Expression-like Matching
**Source:** OCaml Standard Library
**Topic:** Simple glob-style pattern matching
**Difficulty:** Intermediate
**Category:** stdlib-string
**OCaml:**
```ocaml
(* Simple glob matching: * matches any substring, ? matches one char *)
let rec glob_match pattern str =
  match (pattern, str) with
  | ("", "") -> true
  | ("", _) -> false
  | ("*", _) -> true
  | _ when String.length pattern > 0 && String.length str = 0 ->
    pattern = "*"
  | _ ->
    let pc = pattern.[0] and sc = str.[0] in
    let prest = String.sub pattern 1 (String.length pattern - 1) in
    let srest = String.sub str 1 (String.length str - 1) in
    if pc = '*' then
      glob_match prest str || glob_match pattern srest
    else if pc = '?' || pc = sc then
      glob_match prest srest
    else false

let tests = [("*.ml", "hello.ml"); ("test_?", "test_a"); ("foo*", "bar")]
let () = List.iter (fun (p, s) ->
  Printf.printf "glob(%s, %s) = %b\n" p s (glob_match p s)
) tests
```
**Status:** [ ]

### 144: Hashtbl — Default Dict Pattern
**Source:** OCaml Standard Library
**Topic:** Hash table with default value factory
**Difficulty:** Intermediate
**Category:** stdlib-hashtbl
**OCaml:**
```ocaml
let find_or_add tbl key default_fn =
  match Hashtbl.find_opt tbl key with
  | Some v -> v
  | None ->
    let v = default_fn () in
    Hashtbl.add tbl key v; v

(* Group items by category *)
let group_by key_fn items =
  let tbl = Hashtbl.create 16 in
  List.iter (fun item ->
    let key = key_fn item in
    let lst = find_or_add tbl key (fun () -> ref []) in
    lst := item :: !lst
  ) items;
  Hashtbl.fold (fun k v acc -> (k, List.rev !v) :: acc) tbl []

let data = ["apple"; "banana"; "avocado"; "blueberry"; "cherry"]
let groups = group_by (fun s -> s.[0]) data
let () = List.iter (fun (k, vs) ->
  Printf.printf "%c: %s\n" k (String.concat ", " vs)
) groups
```
**Status:** [ ]

### 145: Recursive Descent — Infix Expression with Parentheses
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/interp/parsing.html
**Topic:** Full expression parser with operator precedence
**Difficulty:** Advanced
**Category:** parsing
**OCaml:**
```ocaml
type expr = Num of int | Binop of char * expr * expr

let rec eval = function
  | Num n -> n
  | Binop ('+', a, b) -> eval a + eval b
  | Binop ('-', a, b) -> eval a - eval b
  | Binop ('*', a, b) -> eval a * eval b
  | Binop ('/', a, b) -> eval a / eval b
  | Binop _ -> failwith "unknown op"

(* Simple evaluator using Dijkstra's shunting yard *)
let calc s =
  let tokens = String.split_on_char ' ' s in
  (* Simple recursive descent *)
  let pos = ref 0 in
  let toks = Array.of_list tokens in
  let peek () = if !pos < Array.length toks then toks.(!pos) else "" in
  let consume () = let t = peek () in incr pos; t in
  let rec expr () =
    let left = term () in
    match peek () with
    | "+" -> ignore (consume ()); Binop ('+', left, expr ())
    | "-" -> ignore (consume ()); Binop ('-', left, expr ())
    | _ -> left
  and term () =
    let left = atom () in
    match peek () with
    | "*" -> ignore (consume ()); Binop ('*', left, term ())
    | "/" -> ignore (consume ()); Binop ('/', left, term ())
    | _ -> left
  and atom () = Num (int_of_string (consume ()))
  in
  eval (expr ())

let () = Printf.printf "2 + 3 * 4 = %d\n" (calc "2 + 3 * 4")
```
**Status:** [ ]

### 146: Map — Inverse Map (Swap Keys and Values)
**Source:** OCaml Standard Library
**Topic:** Create an inverse mapping from values to keys
**Difficulty:** Intermediate
**Category:** stdlib-map
**OCaml:**
```ocaml
module SMap = Map.Make(String)
module IMap = Map.Make(Int)

let invert_map m =
  SMap.fold (fun k v acc ->
    let keys = match IMap.find_opt v acc with
      | Some ks -> k :: ks | None -> [k]
    in IMap.add v keys acc
  ) m IMap.empty

let scores = SMap.of_list [("Alice", 95); ("Bob", 87); ("Carol", 95); ("Dave", 87)]
let by_score = invert_map scores

let () = IMap.iter (fun score names ->
  Printf.printf "Score %d: %s\n" score (String.concat ", " names)
) by_score
```
**Status:** [ ]

### 147: Option — Chaining Multiple Lookups
**Source:** OCaml Standard Library
**Topic:** Chain Option.bind for multi-step lookups
**Difficulty:** Intermediate
**Category:** stdlib-option
**OCaml:**
```ocaml
let users = [("alice", 1); ("bob", 2)]
let profiles = [(1, "Engineer"); (2, "Designer")]
let salaries = [("Engineer", 90000); ("Designer", 85000)]

let get_salary name =
  List.assoc_opt name users
  |> Option.bind (fun id -> List.assoc_opt id profiles)
  |> Option.bind (fun role -> List.assoc_opt role salaries)

let () = List.iter (fun name ->
  match get_salary name with
  | Some s -> Printf.printf "%s earns %d\n" name s
  | None -> Printf.printf "%s: unknown\n" name
) ["alice"; "bob"; "charlie"]
```
**Status:** [ ]

### 148: Seq — Fibonacci and Collatz via Seq
**Source:** OCaml Standard Library
**Topic:** Classic sequences with Seq module
**Difficulty:** Beginner
**Category:** stdlib-seq
**OCaml:**
```ocaml
let collatz n =
  Seq.unfold (fun n ->
    if n = 1 then None
    else let next = if n mod 2 = 0 then n / 2 else 3 * n + 1 in
      Some (n, next)
  ) n

let () =
  Printf.printf "Collatz(27): ";
  let seq = collatz 27 in
  Seq.iter (fun x -> Printf.printf "%d " x) seq;
  Printf.printf "1\n";
  Printf.printf "Length: %d\n" (Seq.length (collatz 27) + 1)
```
**Status:** [ ]

### 149: Record — Deriving Comparison
**Source:** OCaml Standard Library
**Topic:** Compare records field by field
**Difficulty:** Beginner
**Category:** records
**OCaml:**
```ocaml
type date = { year: int; month: int; day: int }

let compare_date a b =
  match compare a.year b.year with
  | 0 -> (match compare a.month b.month with
          | 0 -> compare a.day b.day
          | n -> n)
  | n -> n

let dates = [
  { year=2024; month=3; day=15 };
  { year=2024; month=1; day=20 };
  { year=2023; month=12; day=1 };
  { year=2024; month=3; day=10 };
]

let sorted = List.sort compare_date dates
let () = List.iter (fun d ->
  Printf.printf "%04d-%02d-%02d\n" d.year d.month d.day
) sorted
```
**Status:** [ ]

### 150: Fold — Universal Iterator
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/hop/fold.html
**Topic:** Implement many list functions using fold
**Difficulty:** Intermediate
**Category:** higher-order
**OCaml:**
```ocaml
(* Everything is a fold *)
let my_length lst = List.fold_left (fun acc _ -> acc + 1) 0 lst
let my_rev lst = List.fold_left (fun acc x -> x :: acc) [] lst
let my_map f lst = List.fold_right (fun x acc -> f x :: acc) lst []
let my_filter p lst = List.fold_right (fun x acc -> if p x then x :: acc else acc) lst []
let my_exists p lst = List.fold_left (fun acc x -> acc || p x) false lst
let my_for_all p lst = List.fold_left (fun acc x -> acc && p x) true lst
let my_flatten lst = List.fold_right ( @ ) lst []

let data = [1; 2; 3; 4; 5]
let () =
  Printf.printf "length: %d\n" (my_length data);
  Printf.printf "rev: %s\n" (String.concat " " (List.map string_of_int (my_rev data)));
  Printf.printf "evens: %s\n" (String.concat " " (List.map string_of_int (my_filter (fun x -> x mod 2 = 0) data)))
```
**Status:** [ ]

### 151: Modules — Include for Extension
**Source:** Real World OCaml — https://dev.realworldocaml.org/files-modules-and-programs.html
**Topic:** Extend standard library modules with include
**Difficulty:** Intermediate
**Category:** modules
**OCaml:**
```ocaml
module MyString = struct
  include String

  let starts_with ~prefix s =
    let plen = length prefix in
    plen <= length s && sub s 0 plen = prefix

  let ends_with ~suffix s =
    let slen = length suffix and len = length s in
    slen <= len && sub s (len - slen) slen = suffix

  let repeat n s =
    let buf = Buffer.create (n * length s) in
    for _ = 1 to n do Buffer.add_string buf s done;
    Buffer.contents buf

  let count_char c s =
    fold_left (fun acc ch -> if ch = c then acc + 1 else acc) 0 s
end

let () =
  Printf.printf "starts: %b\n" (MyString.starts_with ~prefix:"hel" "hello");
  Printf.printf "repeat: %s\n" (MyString.repeat 3 "ab");
  Printf.printf "count 'l': %d\n" (MyString.count_char 'l' "hello world")
```
**Status:** [ ]

### 152: Sequence — Drop While and Take While
**Source:** OCaml Standard Library
**Topic:** Conditional prefix operations on sequences
**Difficulty:** Intermediate
**Category:** stdlib-seq
**OCaml:**
```ocaml
let rec take_while p s () = match s () with
  | Seq.Nil -> Seq.Nil
  | Seq.Cons (x, rest) ->
    if p x then Seq.Cons (x, take_while p rest) else Seq.Nil

let rec drop_while p s () = match s () with
  | Seq.Nil -> Seq.Nil
  | Seq.Cons (x, rest) ->
    if p x then drop_while p rest () else Seq.Cons (x, rest)

let data = List.to_seq [1; 2; 3; 10; 20; 1; 2]
let prefix = take_while (fun x -> x < 10) data |> List.of_seq
let suffix = drop_while (fun x -> x < 10) data |> List.of_seq

let () =
  Printf.printf "take_while < 10: %s\n"
    (String.concat " " (List.map string_of_int prefix));
  Printf.printf "drop_while < 10: %s\n"
    (String.concat " " (List.map string_of_int suffix))
```
**Status:** [ ]

### 153: Imperative — Stack with Array
**Source:** Real World OCaml — https://dev.realworldocaml.org/imperative-programming.html
**Topic:** Array-backed stack with dynamic resizing
**Difficulty:** Intermediate
**Category:** data-structures
**OCaml:**
```ocaml
type 'a stack = {
  mutable data : 'a option array;
  mutable top : int;
}

let create () = { data = Array.make 8 None; top = 0 }

let push s x =
  if s.top = Array.length s.data then begin
    let new_data = Array.make (s.top * 2) None in
    Array.blit s.data 0 new_data 0 s.top;
    s.data <- new_data
  end;
  s.data.(s.top) <- Some x;
  s.top <- s.top + 1

let pop s =
  if s.top = 0 then None
  else begin
    s.top <- s.top - 1;
    let v = s.data.(s.top) in
    s.data.(s.top) <- None;
    v
  end

let s = create ()
let () = List.iter (push s) [1;2;3;4;5]
let () =
  let rec drain () = match pop s with
    | Some x -> Printf.printf "%d " x; drain ()
    | None -> print_newline ()
  in drain ()
```
**Status:** [ ]

### 154: Type Classes via Modules — Show and Eq
**Source:** Real World OCaml — https://dev.realworldocaml.org/first-class-modules.html
**Topic:** Simulate type classes with module signatures
**Difficulty:** Advanced
**Category:** modules
**OCaml:**
```ocaml
module type SHOW = sig
  type t
  val show : t -> string
end

module type EQ = sig
  type t
  val equal : t -> t -> bool
end

let print_list (type a) (module S : SHOW with type t = a) (lst : a list) =
  Printf.printf "[%s]\n" (String.concat "; " (List.map S.show lst))

let dedup (type a) (module E : EQ with type t = a) (lst : a list) =
  List.fold_left (fun acc x ->
    if List.exists (E.equal x) acc then acc else x :: acc
  ) [] lst |> List.rev

let () =
  let module IntShow = struct type t = int let show = string_of_int end in
  let module IntEq = struct type t = int let equal = Int.equal end in
  print_list (module IntShow) [1;2;3;4;5];
  let d = dedup (module IntEq) [1;2;1;3;2;4;3;5] in
  print_list (module IntShow) d
```
**Status:** [ ]

### 155: Environment — Substitution Model
**Source:** Cornell CS3110 — https://cs3110.github.io/textbook/chapters/interp/substitution.html
**Topic:** Simple interpreter with environment-based evaluation
**Difficulty:** Advanced
**Category:** interpreters
**OCaml:**
```ocaml
type expr =
  | Var of string | Num of int
  | Let of string * expr * expr
  | Fun of string * expr | App of expr * expr
  | Add of expr * expr

type value = VNum of int | VFun of string * expr * env
and env = (string * value) list

let rec eval env = function
  | Num n -> VNum n
  | Var x -> List.assoc x env
  | Add (a, b) ->
    (match (eval env a, eval env b) with
     | (VNum a, VNum b) -> VNum (a + b) | _ -> failwith "type error")
  | Let (x, e1, e2) -> eval ((x, eval env e1) :: env) e2
  | Fun (x, body) -> VFun (x, body, env)
  | App (f, arg) ->
    (match eval env f with
     | VFun (x, body, cenv) -> eval ((x, eval env arg) :: cenv) body
     | _ -> failwith "not a function")

let prog = Let ("double", Fun ("x", Add (Var "x", Var "x")),
                App (Var "double", Num 21))
let () = match eval [] prog with
  | VNum n -> Printf.printf "Result: %d\n" n | _ -> ()
```
**Status:** [ ]
