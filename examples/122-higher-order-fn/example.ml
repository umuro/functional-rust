(* Example 122: Higher-Order Functions with Lifetime Constraints *)

(* OCaml HOFs work without worrying about lifetimes.
   Rust HOFs that take/return references need lifetime annotations. *)

(* Approach 1: Function that takes a predicate and returns matching slice *)
let find_first pred lst =
  try Some (List.find pred lst)
  with Not_found -> None

let approach1 () =
  let data = ["apple"; "banana"; "cherry"; "date"] in
  let long = find_first (fun s -> String.length s > 5) data in
  assert (long = Some "banana");
  Printf.printf "First long: %s\n" (Option.get long)

(* Approach 2: Composing functions *)
let compose f g x = f (g x)
let pipe x f = f x

let approach2 () =
  let double = ( * ) 2 in
  let add1 = ( + ) 1 in
  let double_then_add = compose add1 double in
  assert (double_then_add 5 = 11);
  let result = pipe 5 double |> add1 in
  assert (result = 11);
  Printf.printf "compose(add1, double)(5) = %d\n" (double_then_add 5)

(* Approach 3: Applying transformations to borrowed data *)
let transform_all f items =
  List.map f items

let approach3 () =
  let words = ["hello"; "WORLD"; "Rust"] in
  let lower = transform_all String.lowercase_ascii words in
  assert (lower = ["hello"; "world"; "rust"]);
  Printf.printf "Lowered: %s\n" (String.concat ", " lower)

let () =
  approach1 ();
  approach2 ();
  approach3 ();
  Printf.printf "✓ All tests passed\n"
