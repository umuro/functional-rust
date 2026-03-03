(* A semigroup is a set with an associative binary operation.
   Like a monoid but without requiring an identity element.
   Weaker but more widely applicable. *)

module type SEMIGROUP = sig
  type t
  val append : t -> t -> t
  (* Law: append (append a b) c = append a (append b c) *)
end

(* NonEmpty list semigroup — note: no empty! *)
module NonEmptyList : SEMIGROUP with type t = int list = struct
  type t = int list
  let append = ( @ )
end

(* Min semigroup — take the smaller *)
module MinSemigroup : SEMIGROUP with type t = int = struct
  type t = int
  let append = min
end

(* Max semigroup — take the larger *)
module MaxSemigroup : SEMIGROUP with type t = int = struct
  type t = int
  let append = max
end

(* First semigroup — always keep the first *)
module FirstSemigroup : SEMIGROUP with type t = string = struct
  type t = string
  let append a _ = a
end

(* Reduce non-empty list with semigroup *)
let sconcat (module S : SEMIGROUP) lst =
  match lst with
  | []     -> failwith "sconcat: empty list"
  | x :: xs -> List.fold_left S.append x xs

let () =
  let nums = [3; 1; 4; 1; 5; 9; 2; 6] in
  Printf.printf "min of nums: %d\n" (sconcat (module MinSemigroup) nums);
  Printf.printf "max of nums: %d\n" (sconcat (module MaxSemigroup) nums);

  let words = ["first"; "second"; "third"] in
  Printf.printf "first word: %s\n" (sconcat (module FirstSemigroup) words);

  (* Verify associativity *)
  let a = 3 and b = 1 and c = 4 in
  assert (MinSemigroup.append (MinSemigroup.append a b) c =
          MinSemigroup.append a (MinSemigroup.append b c));
  Printf.printf "Associativity law holds\n"
