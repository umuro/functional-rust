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
  Printf.printf "City: %s\n" (person_city.get p);
  let p = over person_city String.uppercase_ascii p in
  Printf.printf "City: %s\n" (person_city.get p)