(* A profunctor p a b is contravariant in a, covariant in b.
   dimap :: (c -> a) -> (b -> d) -> p a b -> p c d *)

(* Functions are the canonical profunctor *)
let dimap f g h = fun x -> g (h (f x))

let lmap f h = dimap f (fun x -> x) h
let rmap g h = dimap (fun x -> x) g h

(* Profunctor identity laws *)
let id x = x
let ( *** ) f g x = (f (fst x), g (snd x))

(* Star: wraps f: a -> m b as a profunctor *)
(* For functors that have map *)
let star_dimap f g h = fun a ->
  let mb = h (f a) in
  List.map g mb    (* using list as the functor *)

(* Forget: always forgets the output, keeps input *)
type ('a, 'b) forget = Forget of ('a -> 'b)
let dimap_forget f _ (Forget h) = Forget (fun c -> h (f c))

let () =
  let proc = fun (s : string) -> String.length s in

  (* rmap: adapt output *)
  let proc2 = rmap (fun n -> n > 3) proc in
  Printf.printf "length > 3 of 'hello': %b\n" (proc2 "hello");
  Printf.printf "length > 3 of 'hi': %b\n" (proc2 "hi");

  (* lmap: adapt input *)
  let proc3 = lmap string_of_int proc in
  Printf.printf "length of 42: %d\n" (proc3 42);

  (* dimap: adapt both *)
  let proc4 = dimap string_of_int (fun n -> n * 2) proc in
  Printf.printf "dimap on 123: %d\n" (proc4 123)
