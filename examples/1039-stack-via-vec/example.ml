(* 1039: Stack Using a List
   OCaml lists naturally implement a LIFO stack: cons = push, hd/tl = peek/pop.
   Also demonstrates RPN evaluator and bracket balancing. *)

(* Approach 1: OCaml list as a pure functional stack *)
let push x stack = x :: stack
let pop = function [] -> (None, []) | x :: rest -> (Some x, rest)
let peek = function [] -> None | x :: _ -> Some x
let is_empty = function [] -> true | _ -> false
let size = List.length

let functional_stack_demo () =
  let s = [] in
  assert (is_empty s);
  let s = push 10 (push 20 (push 30 s)) in
  (* Stack top is last-pushed = 10 (since push prepends) *)
  assert (size s = 3);
  assert (peek s = Some 10);
  let (v, s) = pop s in assert (v = Some 10);
  let (v, s) = pop s in assert (v = Some 20);
  let (v, s) = pop s in assert (v = Some 30);
  let (v, _) = pop s in assert (v = None)

(* Approach 2: Mutable stack backed by array (like Rust's Vec) *)
type 'a stack_mut = { mutable items : 'a list }

let make_stack () = { items = [] }
let push_mut s x = s.items <- x :: s.items
let pop_mut s = match s.items with
  | [] -> None
  | x :: rest -> s.items <- rest; Some x
let peek_mut s = match s.items with [] -> None | x :: _ -> Some x

(* Approach 3: RPN calculator *)
let eval_rpn tokens =
  let stack = ref [] in
  List.iter (fun token ->
    match token with
    | "+" | "-" | "*" ->
      let b = List.hd !stack in stack := List.tl !stack;
      let a = List.hd !stack in stack := List.tl !stack;
      let result = match token with
        | "+" -> a + b
        | "-" -> a - b
        | "*" -> a * b
        | _   -> assert false
      in
      stack := result :: !stack
    | n -> stack := int_of_string n :: !stack
  ) tokens;
  List.hd !stack

(* Approach 4: Balanced bracket checker *)
let is_balanced s =
  let stack = ref [] in
  let ok = ref true in
  String.iter (fun c ->
    match c with
    | '(' | '[' | '{' -> stack := c :: !stack
    | ')' -> (match !stack with '(' :: rest -> stack := rest | _ -> ok := false)
    | ']' -> (match !stack with '[' :: rest -> stack := rest | _ -> ok := false)
    | '}' -> (match !stack with '{' :: rest -> stack := rest | _ -> ok := false)
    | _   -> ()
  ) s;
  !ok && !stack = []

let () =
  functional_stack_demo ();

  let s = make_stack () in
  push_mut s 10; push_mut s 20; push_mut s 30;
  assert (peek_mut s = Some 30);
  assert (pop_mut s = Some 30);
  assert (pop_mut s = Some 20);
  assert (pop_mut s = Some 10);
  assert (pop_mut s = None);

  (* RPN: (3 + 4) * 2 = 14 *)
  assert (eval_rpn ["3";"4";"+";"2";"*"] = 14);
  (* 5 + (1+2)*4 - 3 = 14 *)
  assert (eval_rpn ["5";"1";"2";"+";"4";"*";"+";"3";"-"] = 14);

  assert (is_balanced "({[]})");
  assert (is_balanced "");
  assert (not (is_balanced "({[})"));
  assert (not (is_balanced "(("));

  Printf.printf "All stack tests passed.\n"
