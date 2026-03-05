(* Isomorphism pattern in OCaml *)
type ('a,'b) iso = { to_: 'a -> 'b; from_: 'b -> 'a }

let invert i = { to_=i.from_; from_=i.to_ }

let compose i j = {
  to_   = (fun a -> j.to_   (i.to_   a));
  from_ = (fun c -> i.from_ (j.from_ c));
}

(* Celsius <-> Fahrenheit *)
let celsius_fahrenheit = {
  to_   = (fun c -> c *. 9.0 /. 5.0 +. 32.0);
  from_ = (fun f -> (f -. 32.0) *. 5.0 /. 9.0);
}

(* Radians <-> Degrees *)
let radians_degrees = {
  to_   = (fun r -> r *. 180.0 /. Float.pi);
  from_ = (fun d -> d *. Float.pi /. 180.0);
}

(* String <-> Chars list *)
let string_chars = {
  to_   = (fun s -> List.init (String.length s) (String.get s));
  from_ = (fun cs -> String.concat "" (List.map (String.make 1) cs));
}

(* Law checks *)
let law_roundtrip iso a = iso.from_ (iso.to_ a) = a
let law_roundtrip_inv iso b = iso.to_ (iso.from_ b) = b

let () =
  Printf.printf "100°C = %.1f°F\n" (celsius_fahrenheit.to_ 100.0);
  Printf.printf "212°F = %.1f°C\n" ((invert celsius_fahrenheit).to_ 212.0);
  Printf.printf "law C->F->C at 100: %b\n" (law_roundtrip celsius_fahrenheit 100.0);
  let s = "hello" in
  let chars = string_chars.to_ s in
  Printf.printf "chars: %d\n" (List.length chars);
  Printf.printf "back: %s\n" (string_chars.from_ chars)
