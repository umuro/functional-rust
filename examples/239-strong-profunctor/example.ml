(* A strong profunctor can carry extra information through computation.
   first  :: p a b -> p (a * c) (b * c)
   second :: p a b -> p (c * a) (c * b)
   Enables building lenses! *)

(* Functions form a strong profunctor *)
let first  f (a, c) = (f a, c)
let second f (c, a) = (c, f a)

(* Lens from strong profunctor *)
(* A lens s a = forall p. Strong p => p a a -> p s s *)
(* Simplified: a pair of get/set *)
type ('s, 'a) lens = {
  get : 's -> 'a;
  set : 'a -> 's -> 's;
}

let view lens s = lens.get s
let over lens f s = lens.set (f (lens.get s)) s
let set  lens a s = lens.set a s

(* Compose lenses *)
let compose l1 l2 = {
  get = (fun s -> l2.get (l1.get s));
  set = (fun a s -> l1.set (l2.set a (l1.get s)) s);
}

type person = { name: string; age: int }
type company = { ceo: person; revenue: int }

let name_lens = { get = (fun p -> p.name); set = (fun n p -> { p with name = n }) }
let ceo_lens  = { get = (fun c -> c.ceo);  set = (fun p c -> { c with ceo = p }) }
let ceo_name  = compose ceo_lens name_lens

let () =
  let c = { ceo = { name = "Alice"; age = 45 }; revenue = 1_000_000 } in
  Printf.printf "CEO: %s\n" (view ceo_name c);
  let c' = set ceo_name "Bob" c in
  Printf.printf "New CEO: %s\n" (view ceo_name c');
  let c'' = over ceo_lens (fun p -> { p with age = p.age + 1 }) c in
  Printf.printf "CEO age after bday: %d\n" c''.ceo.age
