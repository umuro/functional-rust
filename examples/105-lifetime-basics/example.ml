(* 105: Lifetime Basics *)
(* OCaml: GC manages lifetimes automatically. No annotations needed. *)

(* Approach 1: Return reference to data — always safe in OCaml *)
let first_element lst =
  match lst with [] -> None | x :: _ -> Some x

let longest s1 s2 =
  if String.length s1 >= String.length s2 then s1 else s2

(* Approach 2: Closures capturing references — GC keeps them alive *)
let make_greeter name =
  fun greeting -> Printf.sprintf "%s, %s!" greeting name

(* Approach 3: References to local data — GC prevents dangling *)
let create_and_use () =
  let data = [1; 2; 3] in
  let f = fun () -> List.hd data in
  f ()  (* data still alive because GC tracks it *)

(* Tests *)
let () =
  assert (first_element [1; 2; 3] = Some 1);
  assert (longest "hello" "hi" = "hello");
  let greet = make_greeter "World" in
  assert (greet "Hello" = "Hello, World!");
  assert (create_and_use () = 1);
  Printf.printf "✓ All tests passed\n"
