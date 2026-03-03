(* Choice profunctor: can be applied to one side of a sum type.
   left  :: p a b -> p (a + c) (b + c)
   right :: p a b -> p (c + a) (c + b)
   Enables building prisms! *)

(* Functions form a choice profunctor *)
let left  f = function Left a  -> Left (f a)  | Right c -> Right c
let right f = function Right a -> Right (f (a)) | Left c -> Left c

(* Prism: focuses on one constructor of a sum type *)
type ('s, 'a) prism = {
  preview : 's -> 'a option;
  review  : 'a -> 's;
}

let preview prism s = prism.preview s
let review  prism a = prism.review a

(* Transform matching elements *)
let over_prism prism f s =
  match prism.preview s with
  | None   -> s
  | Some a -> prism.review (f a)

type json =
  | JNull
  | JBool   of bool
  | JInt    of int
  | JString of string
  | JArray  of json list

let int_prism : (json, int) prism = {
  preview = (function JInt n -> Some n | _ -> None);
  review  = (fun n -> JInt n);
}

let () =
  let v = JInt 42 in
  Printf.printf "preview int: %s\n" (match preview int_prism v with Some n -> string_of_int n | None -> "none");
  Printf.printf "preview str: %s\n" (match preview int_prism (JString "hi") with Some n -> string_of_int n | None -> "none");

  let doubled = over_prism int_prism (fun n -> n * 2) v in
  Printf.printf "doubled: %s\n" (match preview int_prism doubled with Some n -> string_of_int n | None -> "?");

  Printf.printf "review: %s\n" (match review int_prism 99 with JInt n -> string_of_int n | _ -> "?")
