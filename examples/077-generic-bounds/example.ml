(* 077: Generic Bounds — OCaml parametric polymorphism *)
(* OCaml doesn't need bounds — polymorphism is structural *)

(* Approach 1: Generic identity and pair — no bounds needed *)
let identity x = x
let make_pair a b = (a, b)
let swap (a, b) = (b, a)

(* Approach 2: Functors as "bounded" generics *)
module type PRINTABLE = sig
  type t
  val to_string : t -> string
end

module type COMPARABLE = sig
  type t
  val compare : t -> t -> int
end

module Printer (P : PRINTABLE) = struct
  let print_list lst =
    let strs = List.map P.to_string lst in
    "[" ^ String.concat "; " strs ^ "]"
end

module IntPrint = Printer(struct
  type t = int
  let to_string = string_of_int
end)

(* Approach 3: Using polymorphic comparison *)
let find_max lst =
  match lst with
  | [] -> None
  | x :: xs -> Some (List.fold_left max x xs)

let contains lst x = List.exists (fun e -> e = x) lst

(* Tests *)
let () =
  assert (identity 42 = 42);
  assert (identity "hello" = "hello");
  assert (make_pair 1 "a" = (1, "a"));
  assert (swap (1, 2) = (2, 1));
  assert (IntPrint.print_list [1; 2; 3] = "[1; 2; 3]");
  assert (find_max [3; 1; 4; 1; 5] = Some 5);
  assert (find_max [] = None);
  assert (contains [1; 2; 3] 2);
  assert (not (contains [1; 2; 3] 4));
  Printf.printf "✓ All tests passed\n"
