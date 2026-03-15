(* 946: Catamorphism — Generalized Fold on ADTs

   A catamorphism replaces each constructor of an algebraic data type
   with a function. It is the universal way to consume a recursive structure.

   The pattern is: one function argument per constructor, applied bottom-up. *)

(* ── Binary tree ─────────────────────────────────────────────────────────── *)

type 'a tree =
  | Leaf
  | Node of 'a tree * 'a * 'a tree

let node l v r = Node (l, v, r)

(* ── cata: the catamorphism for binary trees ─────────────────────────────── *)

(* Replace Leaf with `leaf_val`; replace Node(l,v,r) with `node_fn l_result v r_result`.
   This is the most general way to fold over a tree — any computation is a cata. *)
let rec cata leaf_val node_fn = function
  | Leaf -> leaf_val
  | Node (l, v, r) ->
    let l_result = cata leaf_val node_fn l in
    let r_result = cata leaf_val node_fn r in
    node_fn l_result v r_result

(* ── All derived via cata — no explicit recursion ────────────────────────── *)

let size   t = cata 0 (fun l _ r -> 1 + l + r) t
let sum    t = cata 0 (fun l v r -> l + v + r) t
let height t = cata 0 (fun l _ r -> 1 + max l r) t

(* Mirror: swap left and right subtrees — use direct recursion here *)
let rec mirror = function
  | Leaf -> Leaf
  | Node (l, v, r) -> Node (mirror r, v, mirror l)

(* In-order traversal: left, root, right — expressed as a cata *)
let to_list t =
  cata [] (fun l v r -> l @ [v] @ r) t

(* Pre-order traversal: root, left, right *)
let preorder t =
  cata [] (fun l v r -> [v] @ l @ r) t

(* Any predicate: does any node value satisfy pred? *)
let tree_any pred t =
  cata false (fun l v r -> l || pred v || r) t

let tree_all pred t =
  cata true (fun l v r -> l && pred v && r) t

(* ── Expr catamorphism ───────────────────────────────────────────────────── *)

type expr =
  | Lit  of int
  | Add  of expr * expr
  | Mul  of expr * expr

(* cata_expr: replace each constructor with a function *)
let rec cata_expr lit_fn add_fn mul_fn = function
  | Lit n      -> lit_fn n
  | Add (l, r) -> add_fn (cata_expr lit_fn add_fn mul_fn l)
                         (cata_expr lit_fn add_fn mul_fn r)
  | Mul (l, r) -> mul_fn (cata_expr lit_fn add_fn mul_fn l)
                         (cata_expr lit_fn add_fn mul_fn r)

let eval_expr e = cata_expr (fun n -> n) ( + ) ( * ) e

let pretty_expr e =
  cata_expr
    string_of_int
    (fun l r -> Printf.sprintf "(%s + %s)" l r)
    (fun l r -> Printf.sprintf "(%s * %s)" l r)
    e

let () =
  (* sample:   2
              / \
             1   3    *)
  let t = node (node Leaf 1 Leaf) 2 (node Leaf 3 Leaf) in

  assert (size t = 3);
  assert (size Leaf = 0);
  assert (sum t = 6);
  assert (height t = 2);
  assert (height Leaf = 0);
  assert (to_list t = [1; 2; 3]);
  assert (preorder t = [2; 1; 3]);

  (* mirror *)
  assert (to_list (mirror t) = [3; 2; 1]);

  (* any / all *)
  assert (tree_any (fun x -> x = 2) t);
  assert (not (tree_any (fun x -> x = 9) t));
  assert (tree_all (fun x -> x > 0) t);
  assert (not (tree_all (fun x -> x > 1) t));

  (* catamorphism is general: product of all node values *)
  let product = cata 1 (fun l v r -> l * v * r) t in
  assert (product = 1 * 1 * 2 * 1 * 3 * 1);  (* = 6 *)

  (* expr catamorphism *)
  let e = Mul (Add (Lit 2, Lit 3), Lit 4) in  (* (2+3)*4 = 20 *)
  assert (eval_expr e = 20);
  assert (pretty_expr e = "((2 + 3) * 4)");

  print_endline "946-catamorphism: all tests passed"
