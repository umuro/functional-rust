(* Persistent vector using a balanced binary tree *)
(* Idiomatic OCaml — algebraic data type with GC-managed sharing *)

type 'a pvec = Nil | One of 'a | Two of 'a pvec * 'a pvec

let rec size = function
  | Nil -> 0 | One _ -> 1
  | Two (l, r) -> size l + size r

let rec get i = function
  | One x -> if i = 0 then x else failwith "index out of bounds"
  | Two (l, r) ->
    let ls = size l in
    if i < ls then get i l else get (i - ls) r
  | Nil -> failwith "get on empty"

let rec set i v = function
  | One _ -> if i = 0 then One v else failwith "index out of bounds"
  | Two (l, r) ->
    let ls = size l in
    if i < ls then Two (set i v l, r)
    else Two (l, set (i - ls) v r)
  | Nil -> failwith "set on empty"

(* Build a balanced tree from a list *)
let of_list lst =
  let rec build = function
    | [] -> Nil | [x] -> One x
    | lst ->
      let n = List.length lst in
      let left  = List.filteri (fun i _ -> i < n / 2) lst in
      let right = List.filteri (fun i _ -> i >= n / 2) lst in
      Two (build left, build right)
  in build lst

(* Flatten back to a list for inspection *)
let rec to_list = function
  | Nil -> []
  | One x -> [x]
  | Two (l, r) -> to_list l @ to_list r

let () =
  let v1 = of_list [10; 20; 30; 40; 50] in
  assert (get 2 v1 = 30);

  let v2 = set 2 99 v1 in
  assert (get 2 v2 = 99);
  assert (get 2 v1 = 30);   (* original unchanged — persistence *)

  assert (to_list v1 = [10; 20; 30; 40; 50]);
  assert (to_list v2 = [10; 20; 99; 40; 50]);

  Printf.printf "v1[2] = %d\n" (get 2 v1);
  Printf.printf "v2[2] = %d\n" (get 2 v2);
  print_endline "ok"
