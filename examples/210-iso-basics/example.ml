(* Example 210: Iso Basics — Lossless Bidirectional Transformations *)

(* An isomorphism is a lossless, reversible conversion between two types.
   get . reverseGet = id  AND  reverseGet . get = id *)

type ('s, 'a) iso = {
  get : 's -> 'a;
  reverse_get : 'a -> 's;
}

(* Approach 1: Simple isomorphisms *)
let celsius_fahrenheit : (float, float) iso = {
  get = (fun c -> c *. 9.0 /. 5.0 +. 32.0);
  reverse_get = (fun f -> (f -. 32.0) *. 5.0 /. 9.0);
}

let string_chars : (string, char list) iso = {
  get = (fun s -> List.init (String.length s) (String.get s));
  reverse_get = (fun cs -> String.init (List.length cs) (List.nth cs));
}

(* Approach 2: Iso from newtype wrappers *)
type meters = Meters of float
type kilometers = Kilometers of float

let meters_iso : (meters, float) iso = {
  get = (fun (Meters m) -> m);
  reverse_get = (fun m -> Meters m);
}

let km_to_m : (kilometers, meters) iso = {
  get = (fun (Kilometers km) -> Meters (km *. 1000.0));
  reverse_get = (fun (Meters m) -> Kilometers (m /. 1000.0));
}

(* Approach 3: Iso combinators *)
let reverse (i : ('s, 'a) iso) : ('a, 's) iso = {
  get = i.reverse_get;
  reverse_get = i.get;
}

let compose_iso (outer : ('s, 'a) iso) (inner : ('a, 'b) iso) : ('s, 'b) iso = {
  get = (fun s -> inner.get (outer.get s));
  reverse_get = (fun b -> outer.reverse_get (inner.reverse_get b));
}

(* An iso IS a lens *)
let iso_to_lens (i : ('s, 'a) iso) = {|
  get = i.get;
  set = (fun a _s -> i.reverse_get a);
|}

(* === Tests === *)
let () =
  (* Celsius/Fahrenheit roundtrip *)
  let c = 100.0 in
  let f = celsius_fahrenheit.get c in
  assert (abs_float (f -. 212.0) < 0.001);
  let c2 = celsius_fahrenheit.reverse_get f in
  assert (abs_float (c2 -. c) < 0.001);

  (* String/chars roundtrip *)
  let s = "hello" in
  let cs = string_chars.get s in
  assert (cs = ['h'; 'e'; 'l'; 'l'; 'o']);
  assert (string_chars.reverse_get cs = s);

  (* Reverse iso *)
  let fahrenheit_celsius = reverse celsius_fahrenheit in
  assert (abs_float (fahrenheit_celsius.get 212.0 -. 100.0) < 0.001);

  (* Composition *)
  let km_raw = compose_iso km_to_m meters_iso in
  let (Kilometers k) = Kilometers 5.0 in
  assert (abs_float (km_raw.get (Kilometers 5.0) -. 5000.0) < 0.001);
  let back = km_raw.reverse_get 5000.0 in
  assert (back = Kilometers 5.0);

  print_endline "✓ All tests passed"
