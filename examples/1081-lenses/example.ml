type ('s, 'a) lens = {
  get: 's -> 'a;
  set: 'a -> 's -> 's;
}

let compose outer inner = {
  get = (fun s -> inner.get (outer.get s));
  set = (fun a s -> outer.set (inner.set a (outer.get s)) s);
}

let over lens f s = lens.set (f (lens.get s)) s

type address = { street: string; city: string }
type person = { name: string; addr: address }

let addr_lens = { get = (fun p -> p.addr); set = (fun a p -> { p with addr = a }) }
let city_lens = { get = (fun a -> a.city); set = (fun c a -> { a with city = c }) }
let person_city = compose addr_lens city_lens

let () =
  let p = { name = "Alice"; addr = { street = "Main St"; city = "NYC" } } in

  (* Test get through composed lens *)
  assert (person_city.get p = "NYC");

  (* Test set through composed lens *)
  let p2 = person_city.set "Boston" p in
  assert (p2.addr.city = "Boston");
  assert (p2.name = "Alice");
  assert (p2.addr.street = "Main St");

  (* Test over — apply function to focused value *)
  let p3 = over person_city String.uppercase_ascii p in
  assert (person_city.get p3 = "NYC");

  (* Original unchanged *)
  assert (person_city.get p = "NYC");

  (* Lens law: set-get *)
  let p4 = person_city.set "Denver" p in
  assert (person_city.get p4 = "Denver");

  (* Lens law: get-set *)
  let city = person_city.get p in
  let p5 = person_city.set city p in
  assert (p5 = p);

  (* Lens law: set-set *)
  let p6 = person_city.set "B" (person_city.set "A" p) in
  let p7 = person_city.set "B" p in
  assert (p6 = p7);

  Printf.printf "City: %s\n" (person_city.get p);
  let p_upper = over person_city String.uppercase_ascii p in
  Printf.printf "City: %s\n" (person_city.get p_upper);
  print_endline "ok"
