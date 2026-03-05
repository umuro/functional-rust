(* Example 204: Lens Composition — Zoom Into Nested Structs *)

(* A lens from 's to 'a: read and functionally update one field *)
type ('s, 'a) lens = {
  get : 's -> 'a;
  set : 'a -> 's -> 's;
}

(* Approach 1: compose two lenses into one *)
let compose (outer : ('s, 'a) lens) (inner : ('a, 'b) lens) : ('s, 'b) lens = {
  get = (fun s -> inner.get (outer.get s));
  set = (fun b s ->
    let a = outer.get s in
    let a' = inner.set b a in
    outer.set a' s);
}

(* Infix alias for readability *)
let ( |>> ) = compose

(* Domain types *)
type street  = { number : int; name : string }
type address = { street : street; city : string }
type person  = { pname : string; address : address }

(* Individual lenses *)
let address_l : (person,  address) lens = {
  get = (fun p -> p.address);
  set = (fun a p -> { p with address = a });
}

let street_l : (address, street) lens = {
  get = (fun a -> a.street);
  set = (fun s a -> { a with street = s });
}

let number_l : (street, int) lens = {
  get = (fun s -> s.number);
  set = (fun n s -> { s with number = n });
}

let name_l : (street, string) lens = {
  get = (fun s -> s.name);
  set = (fun n s -> { s with name = n });
}

(* Two-level composed lens: person -> street *)
let person_street_l = address_l |>> street_l

(* Three-level composed lens: person -> street number *)
let person_number_l = person_street_l |>> number_l

(* Sample data *)
let alice = {
  pname = "Alice";
  address = {
    city = "Wonderland";
    street = { number = 42; name = "Main St" };
  }
}

let () =
  (* get tests *)
  assert (person_street_l.get alice = { number = 42; name = "Main St" });
  assert (person_number_l.get alice = 42);

  (* set tests *)
  let updated = person_number_l.set 99 alice in
  assert (updated.address.street.number = 99);
  assert (updated.pname = "Alice");
  assert (updated.address.city = "Wonderland");

  (* composition is associative *)
  let left  = (address_l |>> street_l) |>> number_l in
  let right = address_l |>> (street_l  |>> number_l) in
  assert (left.get alice = right.get alice);
  assert ((left.set 7 alice).address.street.number
        = (right.set 7 alice).address.street.number);

  print_endline "ok"
