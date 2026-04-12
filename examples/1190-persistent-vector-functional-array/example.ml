(* Simplified persistent vector using balanced binary tree *)
type 'a pvec = Nil | One of 'a | Two of 'a pvec * 'a pvec

let rec size = function
  | Nil -> 0 | One _ -> 1
  | Two (l, r) -> size l + size r

let rec get i = function
  | One x -> if i = 0 then x else failwith "index"
  | Two (l, r) ->
    let ls = size l in
    if i < ls then get i l else get (i - ls) r
  | Nil -> failwith "empty"

let rec set i v = function
  | One _ -> if i = 0 then One v else failwith "index"
  | Two (l, r) ->
    let ls = size l in
    if i < ls then Two (set i v l, r)
    else Two (l, set (i - ls) v r)
  | Nil -> failwith "empty"

let of_list lst =
  let rec build = function
    | [] -> Nil | [x] -> One x
    | lst ->
      let n = List.length lst in
      let left = List.filteri (fun i _ -> i < n/2) lst in
      let right = List.filteri (fun i _ -> i >= n/2) lst in
      Two (build left, build right)
  in build lst

let () =
  let v = of_list [10;20;30;40;50] in
  assert (size v = 5);
  assert (get 0 v = 10);
  assert (get 2 v = 30);
  assert (get 4 v = 50);
  let v2 = set 2 99 v in
  (* Original unchanged — persistent semantics *)
  assert (get 2 v = 30);
  (* New version has updated element *)
  assert (get 2 v2 = 99);
  assert (of_list [] = Nil);
  print_endline "ok"
