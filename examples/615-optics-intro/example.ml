(* Optics intro in OCaml *)
type address = { street: string; city: string; zip: string }
type person  = { name: string; age: int; address: address }

(* Lens: pair of (get, set) *)
type ('s,'a) lens = {
  get: 's -> 'a;
  set: 'a -> 's -> 's;
}

let name_lens    = { get=(fun p->p.name);    set=(fun v p->{p with name=v}) }
let age_lens     = { get=(fun p->p.age);     set=(fun v p->{p with age=v}) }
let address_lens = { get=(fun p->p.address); set=(fun v p->{p with address=v}) }
let city_lens    = { get=(fun a->a.city);    set=(fun v a->{a with city=v}) }

let compose outer inner = {
  get = (fun s -> inner.get (outer.get s));
  set = (fun a s -> outer.set (inner.set a (outer.get s)) s);
}

let person_city = compose address_lens city_lens

let modify lens f s = lens.set (f (lens.get s)) s

let () =
  let p = { name="Alice"; age=30; address={ street="1 Main St"; city="Boston"; zip="02101" } } in
  Printf.printf "name: %s\n" (name_lens.get p);
  Printf.printf "city: %s\n" (person_city.get p);
  let p2 = person_city.set "Cambridge" p in
  Printf.printf "new city: %s\n" (person_city.get p2);
  let p3 = modify age_lens (fun a -> a+1) p in
  Printf.printf "age+1: %d\n" (age_lens.get p3)
