(* Example 116: Box<T> for Heap Allocation *)

(* OCaml allocates everything on the GC heap automatically.
   Rust's Box<T> is the explicit way to heap-allocate. *)

(* Approach 1: Large data structures *)
let approach1 () =
  let big_array = Array.init 1000 (fun i -> i * i) in
  assert (big_array.(999) = 999 * 999);
  Printf.printf "big_array[999] = %d\n" big_array.(999)

(* Approach 2: Indirection for recursive types (see example 117) *)
type expr =
  | Num of int
  | Add of expr * expr
  | Mul of expr * expr

let rec eval = function
  | Num n -> n
  | Add (a, b) -> eval a + eval b
  | Mul (a, b) -> eval a * eval b

let approach2 () =
  let e = Add (Num 1, Mul (Num 2, Num 3)) in
  let result = eval e in
  assert (result = 7);
  Printf.printf "1 + 2*3 = %d\n" result

(* Approach 3: Trait-like polymorphism with first-class modules *)
module type Shape = sig
  val area : unit -> float
  val name : unit -> string
end

let circle r : (module Shape) = (module struct
  let area () = Float.pi *. r *. r
  let name () = "circle"
end)

let square s : (module Shape) = (module struct
  let area () = s *. s
  let name () = "square"
end)

let approach3 () =
  let shapes = [circle 5.0; square 4.0] in
  List.iter (fun (module S : Shape) ->
    Printf.printf "%s: area=%.2f\n" (S.name ()) (S.area ())
  ) shapes

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
