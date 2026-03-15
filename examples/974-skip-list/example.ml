(* 974: Skip List (Simplified) *)
(* Probabilistic sorted structure. O(log n) average search/insert *)
(* Simplified: fixed max levels, random promotion *)

let max_level = 8
let p = 0.5  (* probability of promoting to next level *)

(* Random level generator *)
let random_level () =
  let level = ref 1 in
  while !level < max_level && Random.float 1.0 < p do
    incr level
  done;
  !level

type 'a node = {
  value: 'a;
  mutable forward: 'a node option array;  (* forward pointers per level *)
}

type 'a skip_list = {
  header: 'a node;  (* sentinel head node *)
  mutable level: int;
}

let create_node value level =
  { value; forward = Array.make level None }

let create () =
  let header = { value = Obj.magic (); forward = Array.make max_level None } in
  { header; level = 0 }

let search sl target =
  let current = ref sl.header in
  for i = sl.level - 1 downto 0 do
    let continue_ = ref true in
    while !continue_ do
      match !current.forward.(i) with
      | Some next when next.value < target ->
        current := next
      | _ -> continue_ := false
    done
  done;
  match !current.forward.(0) with
  | Some node when node.value = target -> true
  | _ -> false

let insert sl value =
  let update = Array.make max_level sl.header in
  let current = ref sl.header in
  for i = sl.level - 1 downto 0 do
    let continue_ = ref true in
    while !continue_ do
      match !current.forward.(i) with
      | Some next when next.value < value ->
        current := next
      | _ -> continue_ := false
    done;
    update.(i) <- !current
  done;
  let new_level = random_level () in
  if new_level > sl.level then begin
    for i = sl.level to new_level - 1 do
      update.(i) <- sl.header
    done;
    sl.level <- new_level
  end;
  let new_node = create_node value new_level in
  for i = 0 to new_level - 1 do
    new_node.forward.(i) <- update.(i).forward.(i);
    update.(i).forward.(i) <- Some new_node
  done

let to_list sl =
  let result = ref [] in
  let current = ref sl.header.forward.(0) in
  while !current <> None do
    let node = Option.get !current in
    result := node.value :: !result;
    current := node.forward.(0)
  done;
  List.rev !result

let () =
  Random.self_init ();
  let sl = create () in

  List.iter (insert sl) [5; 3; 7; 1; 9; 4; 6; 2; 8];

  let lst = to_list sl in
  assert (lst = [1;2;3;4;5;6;7;8;9]);

  assert (search sl 5);
  assert (search sl 1);
  assert (search sl 9);
  assert (not (search sl 0));
  assert (not (search sl 10));

  Printf.printf "✓ All tests passed\n"
