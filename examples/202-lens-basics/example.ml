(* Example 202: Lens Basics — Lens as a Pair of Get and Set *)

(* A lens focuses on one part of a larger structure *)
type ('s, 'a) lens = {
  get : 's -> 'a;
  set : 'a -> 's -> 's;
}

(* === Approach 1: Manual lens construction === *)

type person = {
  name : string;
  age : int;
}

let name_lens : (person, string) lens = {
  get = (fun p -> p.name);
  set = (fun n p -> { p with name = n });
}

let age_lens : (person, int) lens = {
  get = (fun p -> p.age);
  set = (fun a p -> { p with age = a });
}

(* Use a lens *)
let view l s = l.get s
let set l a s = l.set a s
let over l f s = l.set (f (l.get s)) s

(* === Approach 2: Lens via functor (more general) === *)
module type FUNCTOR = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
end

(* Identity functor for get *)
module Identity = struct
  type 'a t = Id of 'a
  let map f (Id x) = Id (f x)
  let run (Id x) = x
end

(* Const functor for set *)
module Const = struct
  type ('a, 'b) t = Const of 'a
  let map _ (Const x) = Const x
  let run (Const x) = x
end

(* === Approach 3: Lens for nested types === *)

type address = {
  street : string;
  city : string;
  zip : string;
}

type employee = {
  emp_name : string;
  address : address;
}

let address_lens : (employee, address) lens = {
  get = (fun e -> e.address);
  set = (fun a e -> { e with address = a });
}

let street_lens : (address, string) lens = {
  get = (fun a -> a.street);
  set = (fun s a -> { a with street = s });
}

let city_lens : (address, string) lens = {
  get = (fun a -> a.city);
  set = (fun c a -> { a with city = c });
}

(* === Tests === *)
let () =
  let alice = { name = "Alice"; age = 30 } in

  (* Test get *)
  assert (view name_lens alice = "Alice");
  assert (view age_lens alice = 30);

  (* Test set *)
  let alice2 = set name_lens "Alicia" alice in
  assert (view name_lens alice2 = "Alicia");
  assert (view age_lens alice2 = 30);

  (* Test over (modify) *)
  let alice3 = over age_lens (fun a -> a + 1) alice in
  assert (view age_lens alice3 = 31);

  (* Test with nested types *)
  let emp = {
    emp_name = "Bob";
    address = { street = "123 Main St"; city = "Springfield"; zip = "62701" };
  } in
  assert (view address_lens emp |> view city_lens = "Springfield");

  let emp2 = set address_lens { (view address_lens emp) with city = "Shelbyville" } emp in
  assert (view address_lens emp2 |> view city_lens = "Shelbyville");

  print_endline "✓ All tests passed"
