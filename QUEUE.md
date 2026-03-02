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
**Status:** [ ]

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
**Status:** [ ]

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
**Status:** [ ]

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
**Status:** [ ]

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
**Status:** [ ]

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

