(* 964: Union-Find / Disjoint Set *)
(* Path compression + union by rank *)

type union_find = {
  parent: int array;
  rank: int array;
  mutable components: int;
}

let create n =
  { parent = Array.init n (fun i -> i);  (* each node is its own root *)
    rank = Array.make n 0;
    components = n }

(* Find with path compression *)
let rec find uf i =
  if uf.parent.(i) = i then i
  else begin
    uf.parent.(i) <- find uf uf.parent.(i);  (* path compression *)
    uf.parent.(i)
  end

(* Union by rank *)
let union uf a b =
  let ra = find uf a in
  let rb = find uf b in
  if ra = rb then false  (* already connected *)
  else begin
    uf.components <- uf.components - 1;
    if uf.rank.(ra) < uf.rank.(rb) then
      uf.parent.(ra) <- rb
    else if uf.rank.(ra) > uf.rank.(rb) then
      uf.parent.(rb) <- ra
    else begin
      uf.parent.(rb) <- ra;
      uf.rank.(ra) <- uf.rank.(ra) + 1
    end;
    true
  end

let connected uf a b = find uf a = find uf b

let num_components uf = uf.components

let () =
  let uf = create 6 in
  assert (num_components uf = 6);

  (* Initially all disconnected *)
  assert (not (connected uf 0 1));
  assert (not (connected uf 2 3));

  (* Union some nodes *)
  assert (union uf 0 1 = true);
  assert (union uf 2 3 = true);
  assert (union uf 4 5 = true);
  assert (num_components uf = 3);

  assert (connected uf 0 1);
  assert (connected uf 2 3);
  assert (not (connected uf 0 2));

  (* Union already connected *)
  assert (union uf 0 1 = false);

  (* Merge two components *)
  assert (union uf 1 2 = true);
  assert (num_components uf = 2);
  assert (connected uf 0 3);  (* transitive *)
  assert (connected uf 0 2);

  (* All connected *)
  assert (union uf 3 4 = true);
  assert (num_components uf = 1);
  assert (connected uf 0 5);

  Printf.printf "✓ All tests passed\n"
