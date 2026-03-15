(* 973: Finger Tree (simplified)
   A functional sequence with O(1) amortized push/pop at both ends and
   O(log n) split/concat. We implement a simplified 2-3 finger tree that
   supports efficient front/back operations.

   Full finger trees are complex; this version uses the digit/spine structure
   from Hinze & Paterson to demonstrate the key ideas in idiomatic OCaml. *)

(* Digits hold 1-4 elements at each end of the tree *)
type 'a digit =
  | One   of 'a
  | Two   of 'a * 'a
  | Three of 'a * 'a * 'a
  | Four  of 'a * 'a * 'a * 'a

(* 2-3 internal nodes *)
type 'a node =
  | N2 of 'a * 'a
  | N3 of 'a * 'a * 'a

(* The spine — recursively holds nodes as elements of the inner tree *)
type 'a finger_tree =
  | Empty
  | Single of 'a
  | Deep of { left : 'a digit; spine : 'a node finger_tree; right : 'a digit }

(* Measure = size (can be generalised to any monoid) *)
let digit_to_list = function
  | One a         -> [a]
  | Two (a,b)     -> [a;b]
  | Three (a,b,c) -> [a;b;c]
  | Four (a,b,c,d)-> [a;b;c;d]

let node_to_list = function N2(a,b) -> [a;b] | N3(a,b,c) -> [a;b;c]

let rec to_list : 'a. 'a finger_tree -> 'a list = function
  | Empty    -> []
  | Single x -> [x]
  | Deep { left; spine; right } ->
    digit_to_list left
    @ List.concat_map node_to_list (to_list spine)
    @ digit_to_list right

let of_list lst =
  List.fold_left (fun t x ->
    (* push_back each element *)
    match t with
    | Empty    -> Single x
    | Single a -> Deep { left = One a; spine = Empty; right = One x }
    | Deep d ->
      match d.right with
      | One a         -> Deep { d with right = Two (a, x) }
      | Two (a,b)     -> Deep { d with right = Three (a, b, x) }
      | Three (a,b,c) -> Deep { d with right = Four (a, b, c, x) }
      | Four (a,b,c,e) ->
        (* overflow: push N3(b,c,e) into spine, keep a as right=One *)
        Deep { d with
          spine = (match of_list (to_list d.spine @ [N3(b,c,e)]) with
                   | t -> t);
          right = Two (e, x) }
  ) Empty lst

(* push_front: add element to the left *)
let rec push_front : 'a. 'a -> 'a finger_tree -> 'a finger_tree =
  fun x t -> match t with
  | Empty    -> Single x
  | Single a -> Deep { left = One x; spine = Empty; right = One a }
  | Deep d ->
    match d.left with
    | One a         -> Deep { d with left = Two (x, a) }
    | Two (a,b)     -> Deep { d with left = Three (x, a, b) }
    | Three (a,b,c) -> Deep { d with left = Four (x, a, b, c) }
    | Four (a,b,c,e) ->
      Deep { d with left = Two (x, a); spine = push_front (N3(b,c,e)) d.spine }

(* push_back: add element to the right *)
let rec push_back : 'a. 'a finger_tree -> 'a -> 'a finger_tree =
  fun t x -> match t with
  | Empty    -> Single x
  | Single a -> Deep { left = One a; spine = Empty; right = One x }
  | Deep d ->
    match d.right with
    | One a         -> Deep { d with right = Two (a, x) }
    | Two (a,b)     -> Deep { d with right = Three (a, b, x) }
    | Three (a,b,c) -> Deep { d with right = Four (a, b, c, x) }
    | Four (a,b,c,e) ->
      Deep { d with right = Two (e, x); spine = push_back d.spine (N3(a,b,c)) }

(* pop_front: remove from left end *)
let digit_tail = function
  | One _         -> None
  | Two (_,b)     -> Some (One b)
  | Three (_,b,c) -> Some (Two (b,c))
  | Four (_,b,c,d)-> Some (Three (b,c,d))

let digit_head = function
  | One a | Two (a,_) | Three (a,_,_) | Four (a,_,_,_) -> a

let rec pop_front : 'a. 'a finger_tree -> ('a * 'a finger_tree) option = function
  | Empty    -> None
  | Single x -> Some (x, Empty)
  | Deep d   ->
    let head = digit_head d.left in
    let tree' = match digit_tail d.left with
      | Some new_left -> Deep { d with left = new_left }
      | None ->
        (* left digit exhausted: pull from spine *)
        (match pop_front d.spine with
         | None ->
           (* spine empty: collapse to right digit *)
           (match d.right with
            | One a         -> Single a
            | Two (a,b)     -> Deep { left=One a; spine=Empty; right=One b }
            | Three (a,b,c) -> Deep { left=Two(a,b); spine=Empty; right=One c }
            | Four (a,b,c,e)-> Deep { left=Three(a,b,c); spine=Empty; right=One e })
         | Some (node, spine') ->
           let new_left = match node with
             | N2(a,b)   -> Two (a,b)
             | N3(a,b,c) -> Three (a,b,c)
           in
           Deep { d with left = new_left; spine = spine' })
    in
    Some (head, tree')

let length t = List.length (to_list t)

let () =
  Printf.printf "=== Finger Tree ===\n";
  let t = List.fold_left push_back Empty [1;2;3;4;5;6;7;8] in
  Printf.printf "to_list: [%s]\n"
    (String.concat "; " (List.map string_of_int (to_list t)));
  Printf.printf "length = %d\n" (length t);

  let t2 = push_front 0 t in
  Printf.printf "push_front 0: [%s]\n"
    (String.concat "; " (List.map string_of_int (to_list t2)));

  let t3 = push_back t2 9 in
  Printf.printf "push_back 9: [%s]\n"
    (String.concat "; " (List.map string_of_int (to_list t3)));

  (match pop_front t3 with
   | Some (x, rest) ->
     Printf.printf "pop_front = %d, remaining: [%s]\n" x
       (String.concat "; " (List.map string_of_int (to_list rest)))
   | None -> ());

  (* Demonstrate immutability: t unchanged after operations *)
  Printf.printf "original t: [%s]\n"
    (String.concat "; " (List.map string_of_int (to_list t)))
