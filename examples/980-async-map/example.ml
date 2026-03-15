(* 980: Map over Async *)
(* OCaml: Lwt.map f promise → transform a resolved value *)

(* --- Approach 1: map over a simple future thunk --- *)

type 'a future = unit -> 'a

let return_ x : 'a future = fun () -> x
let map f fut = fun () -> f (fut ())
let bind fut k = fun () -> k (fut ()) ()
let run f = f ()

let () =
  let fut = return_ 5 in
  let doubled = map (fun x -> x * 2) fut in
  let stringed = map string_of_int doubled in
  assert (run doubled = 10);
  assert (run stringed = "10");
  Printf.printf "Approach 1 (map chain): %s\n" (run stringed)

(* --- Approach 2: functor laws on future --- *)
(* map id = id, map (f . g) = map f . map g *)

let () =
  let fut = return_ 42 in
  (* map id law *)
  let id_mapped = map (fun x -> x) fut in
  assert (run id_mapped = run fut);
  (* composition law *)
  let f x = x + 1 in
  let g x = x * 3 in
  let composed = map (fun x -> f (g x)) fut in
  let chained  = map f (map g fut) in
  assert (run composed = run chained);
  Printf.printf "Approach 2 (functor laws): ✓\n"

(* --- Approach 3: map as derived from bind --- *)

let map_from_bind f fut =
  bind fut (fun x -> return_ (f x))

let () =
  let fut = return_ 7 in
  let r1 = map (fun x -> x * x) fut in
  let r2 = map_from_bind (fun x -> x * x) fut in
  assert (run r1 = run r2);
  Printf.printf "Approach 3 (map via bind): %d = %d\n" (run r1) (run r2)

let () = Printf.printf "✓ All tests passed\n"
