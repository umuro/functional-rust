(* Example 078: Where Clauses *)
(* OCaml doesn't have where clauses — constraints are structural *)

(* Approach 1: Module signature constraints *)
module type Mappable = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
  val to_list : 'a t -> 'a list
end

module type Foldable = sig
  type 'a t
  val fold : ('b -> 'a -> 'b) -> 'b -> 'a t -> 'b
end

(* Approach 2: Functor with multiple constraints *)
module MakeProcessor (M : Mappable) = struct
  let double_all xs = M.map (fun x -> x * 2) xs
  let stringify xs = M.map string_of_int xs
end

(* Approach 3: Plain polymorphic functions *)
let transform_and_combine ~transform ~combine ~init items =
  List.fold_left (fun acc x -> combine acc (transform x)) init items

let filter_map_fold ~pred ~transform ~combine ~init items =
  List.fold_left
    (fun acc x -> if pred x then combine acc (transform x) else acc)
    init items

(* Complex constraint: needs both compare and string conversion *)
let sorted_summary items to_str =
  let sorted = List.sort compare items in
  String.concat ", " (List.map to_str sorted)

let bounded_transform ~lo ~hi ~transform items =
  List.map (fun x ->
    let y = transform x in
    if y < lo then lo
    else if y > hi then hi
    else y
  ) items

(* Tests *)
let () =
  let result = transform_and_combine
    ~transform:(fun x -> x * x)
    ~combine:(+) ~init:0 [1; 2; 3; 4] in
  assert (result = 30);

  let result2 = filter_map_fold
    ~pred:(fun x -> x mod 2 = 0)
    ~transform:(fun x -> x * x)
    ~combine:(+) ~init:0 [1; 2; 3; 4; 5; 6] in
  assert (result2 = 56);

  let summary = sorted_summary [3; 1; 4; 1; 5] string_of_int in
  assert (summary = "1, 1, 3, 4, 5");

  let bounded = bounded_transform ~lo:0 ~hi:10
    ~transform:(fun x -> x * 3) [1; 2; 3; 4; 5] in
  assert (bounded = [3; 6; 9; 10; 10]);

  Printf.printf "✓ All tests passed\n"
