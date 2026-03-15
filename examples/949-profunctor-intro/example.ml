(* Profunctor: a type p a b that is:
   - Contravariant in a (input): dimap maps "backwards" on input
   - Covariant in b (output): maps forwards on output
   Classic example: functions 'a -> 'b *)

(* dimap f g p = g . p . f — adapt input with f, output with g *)
let dimap (f : 'c -> 'a) (g : 'b -> 'd) (p : 'a -> 'b) : 'c -> 'd =
  fun c -> g (p (f c))

(* Convenience: map only input or output *)
let lmap f p = dimap f (fun x -> x) p  (* contramap on input *)
let rmap g p = dimap (fun x -> x) g p  (* map on output *)

let () =
  (* A simple string-processing function *)
  let upper : string -> string = String.uppercase_ascii in

  (* lmap: adapt the input (int -> string, then uppercase) *)
  let int_upper = lmap string_of_int upper in
  Printf.printf "lmap int->string->upper: %s\n" (int_upper 42);

  (* rmap: adapt the output *)
  let upper_len = rmap String.length upper in
  Printf.printf "rmap string->upper->len: %d\n" (upper_len "hello");

  (* dimap: adapt both *)
  let int_upper_len = dimap string_of_int String.length upper in
  Printf.printf "dimap int->string->upper->len: %d\n" (int_upper_len 42);

  (* Profunctor identity law: dimap id id = id *)
  let id x = x in
  let p = dimap id id upper in
  assert (p "hello" = upper "hello");
  Printf.printf "identity law holds\n"
