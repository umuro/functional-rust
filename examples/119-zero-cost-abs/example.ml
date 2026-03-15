(* Example 119: Zero-Cost Abstractions *)

(* OCaml closures and higher-order functions have some overhead
   (allocation, indirect calls). Rust compiles them away. *)

(* Approach 1: Iterator-style processing *)
let approach1 () =
  let data = List.init 1000 (fun i -> i) in
  let result = List.fold_left ( + ) 0
    (List.map (fun x -> x * x)
      (List.filter (fun x -> x mod 2 = 0) data)) in
  Printf.printf "Sum of even squares (0..999): %d\n" result

(* Approach 2: Closure-based computation *)
let make_polynomial coeffs =
  fun x ->
    let rec eval acc power = function
      | [] -> acc
      | c :: rest -> eval (acc +. c *. (x ** power)) (power +. 1.0) rest
    in
    eval 0.0 0.0 coeffs

let approach2 () =
  let poly = make_polynomial [1.0; 2.0; 3.0] in  (* 1 + 2x + 3x^2 *)
  let result = poly 2.0 in  (* 1 + 4 + 12 = 17 *)
  assert (result = 17.0);
  Printf.printf "p(2) = %.1f\n" result

(* Approach 3: Newtype pattern *)
type meters = Meters of float
type seconds = Seconds of float

let speed (Meters d) (Seconds t) = d /. t

let approach3 () =
  let d = Meters 100.0 in
  let t = Seconds 9.58 in
  let s = speed d t in
  Printf.printf "Speed: %.2f m/s\n" s;
  assert (s > 10.0)

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
