(* Example 211: Optics Hierarchy — Iso ⊂ Lens ⊂ Traversal, Iso ⊂ Prism ⊂ Traversal *)

(* The hierarchy of optics, from most powerful to least:
   
   Iso ─── can do everything (bidirectional, exactly 1 focus)
    ├── Lens ── exactly 1 focus, product types (can't reverse)
    │    └── Traversal ── 0-to-many focuses
    └── Prism ── 0-or-1 focus, sum types (can reverse/construct)
         └── Traversal ── 0-to-many focuses
   
   Affine sits between Lens/Prism and Traversal (exactly 0 or 1)
*)

(* Unified optic type that demonstrates the hierarchy *)
type ('s, 'a) optic =
  | Iso_op     of { get : 's -> 'a; reverse_get : 'a -> 's }
  | Lens_op    of { get : 's -> 'a; set : 'a -> 's -> 's }
  | Prism_op   of { preview : 's -> 'a option; review : 'a -> 's }
  | Traversal_op of { over : ('a -> 'a) -> 's -> 's; to_list : 's -> 'a list }

(* Approach 1: Converting down the hierarchy *)

(* Iso → Lens *)
let iso_to_lens get reverse_get =
  Lens_op { get; set = (fun a _s -> reverse_get a) }

(* Iso → Prism *)
let iso_to_prism get reverse_get =
  Prism_op { preview = (fun s -> Some (get s)); review = reverse_get }

(* Lens → Traversal *)
let lens_to_traversal get set =
  Traversal_op {
    over = (fun f s -> set (f (get s)) s);
    to_list = (fun s -> [get s]);
  }

(* Prism → Traversal *)
let prism_to_traversal preview review =
  Traversal_op {
    over = (fun f s -> match preview s with Some a -> review (f a) | None -> s);
    to_list = (fun s -> match preview s with Some a -> [a] | None -> []);
  }

(* Approach 2: Demonstrate with concrete types *)
type celsius = Celsius of float
type color = Red | Green | Blue

let celsius_iso = (fun (Celsius c) -> c), (fun c -> Celsius c)

type point = { x : float; y : float }
let x_lens = (fun p -> p.x), (fun x p -> { p with x })

let red_prism = (function Red -> Some () | _ -> None), (fun () -> Red)

let list_traversal =
  (fun f xs -> List.map f xs), (fun xs -> xs)

(* Approach 3: Verify hierarchy properties *)

(* Every Iso can be used as a Lens *)
let use_as_lens (get, reverse_get) value s =
  let set a _s = reverse_get a in
  set value s

(* Every Lens can be used as a Traversal *)
let use_as_traversal (get, set) f s =
  set (f (get s)) s

(* === Tests === *)
let () =
  (* Iso used as Lens *)
  let (get, rev) = celsius_iso in
  let c = Celsius 100.0 in
  assert (get c = 100.0);
  let c2 = use_as_lens celsius_iso 0.0 c in
  assert (get c2 = 0.0);

  (* Lens used as Traversal *)
  let p = { x = 3.0; y = 4.0 } in
  let (get, set) = x_lens in
  let p2 = use_as_traversal x_lens (fun x -> x *. 2.0) p in
  assert (get p2 = 6.0);
  assert (p2.y = 4.0);

  (* Prism used as Traversal *)
  let (preview, review) = red_prism in
  assert (preview Red = Some ());
  assert (preview Blue = None);
  assert (review () = Red);

  (* Hierarchy: Iso → Lens → Traversal *)
  let iso_as_traversal =
    let get, rev = celsius_iso in
    lens_to_traversal get (fun a _s -> rev a) in
  (match iso_as_traversal with
   | Traversal_op t ->
     assert (t.to_list (Celsius 42.0) = [42.0]);
     let c3 = t.over (fun x -> x +. 1.0) (Celsius 42.0) in
     assert (get c3 = 43.0)
   | _ -> assert false);

  print_endline "✓ All tests passed"
