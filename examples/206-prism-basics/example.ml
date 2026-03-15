(* Example 206: Prism Basics — Optics for Sum Types *)

(* A prism focuses on one variant of a sum type.
   Unlike a lens, the focus might not exist (hence Option). *)

type ('s, 'a) prism = {
  preview : 's -> 'a option;  (* try to extract *)
  review  : 'a -> 's;          (* inject back *)
}

(* Approach 1: Prism for a simple variant *)
type shape =
  | Circle of float
  | Rectangle of float * float
  | Triangle of float * float * float

let circle_prism : (shape, float) prism = {
  preview = (function Circle r -> Some r | _ -> None);
  review = (fun r -> Circle r);
}

let rectangle_prism : (shape, float * float) prism = {
  preview = (function Rectangle (w, h) -> Some (w, h) | _ -> None);
  review = (fun (w, h) -> Rectangle (w, h));
}

(* Approach 2: Prism for Option *)
let some_prism : ('a option, 'a) prism = {
  preview = (fun x -> x);
  review = (fun a -> Some a);
}

(* Approach 3: Prism for Result *)
type ('a, 'e) result = Ok of 'a | Error of 'e

let ok_prism : (('a, 'e) result, 'a) prism = {
  preview = (function Ok a -> Some a | Error _ -> None);
  review = (fun a -> Ok a);
}

let error_prism : (('a, 'e) result, 'e) prism = {
  preview = (function Error e -> Some e | Ok _ -> None);
  review = (fun e -> Error e);
}

(* Prism combinators *)
let over_prism (p : ('s, 'a) prism) (f : 'a -> 'a) (s : 's) : 's =
  match p.preview s with
  | Some a -> p.review (f a)
  | None -> s

let set_prism (p : ('s, 'a) prism) (a : 'a) (s : 's) : 's =
  over_prism p (fun _ -> a) s

(* === Tests === *)
let () =
  let c = Circle 5.0 in
  let r = Rectangle (3.0, 4.0) in

  (* preview succeeds for matching variant *)
  assert (circle_prism.preview c = Some 5.0);
  assert (circle_prism.preview r = None);
  assert (rectangle_prism.preview r = Some (3.0, 4.0));

  (* review constructs the variant *)
  assert (circle_prism.review 10.0 = Circle 10.0);

  (* over modifies if matching *)
  let c2 = over_prism circle_prism (fun r -> r *. 2.0) c in
  assert (c2 = Circle 10.0);

  (* over is identity for non-matching *)
  let r2 = over_prism circle_prism (fun r -> r *. 2.0) r in
  assert (r2 = r);

  (* Option prism *)
  assert (some_prism.preview (Some 42) = Some 42);
  assert (some_prism.preview None = None);
  assert (some_prism.review 42 = Some 42);

  (* Result prism *)
  assert (ok_prism.preview (Ok 1) = Some 1);
  assert (ok_prism.preview (Error "fail") = None);
  assert (error_prism.preview (Error "fail") = Some "fail");

  print_endline "✓ All tests passed"
