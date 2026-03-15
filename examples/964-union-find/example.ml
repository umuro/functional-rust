(* 964: Union-Find (Disjoint Set Union)
   OCaml: mutable arrays for parent and rank.
   Path compression + union by rank → near-O(1) amortized per operation.
   Used to track connected components efficiently. *)

type union_find = {
  parent : int array;
  rank   : int array;
  mutable count : int;   (* number of disjoint sets *)
}

let create n =
  assert (n > 0);
  { parent = Array.init n (fun i -> i);  (* each element is its own root *)
    rank   = Array.make n 0;
    count  = n }

(* Find with path compression (iterative) *)
let find uf x =
  let root = ref x in
  (* walk to root *)
  while uf.parent.(!root) <> !root do
    root := uf.parent.(!root)
  done;
  (* path compression: point all nodes on the path directly to root *)
  let cur = ref x in
  while !cur <> !root do
    let next = uf.parent.(!cur) in
    uf.parent.(!cur) <- !root;
    cur := next
  done;
  !root

(* Union by rank — returns true if they were in different sets *)
let union uf a b =
  let ra = find uf a and rb = find uf b in
  if ra = rb then false
  else begin
    (* attach smaller-rank tree under larger-rank root *)
    if uf.rank.(ra) < uf.rank.(rb) then
      uf.parent.(ra) <- rb
    else if uf.rank.(ra) > uf.rank.(rb) then
      uf.parent.(rb) <- ra
    else begin
      uf.parent.(rb) <- ra;
      uf.rank.(ra) <- uf.rank.(ra) + 1
    end;
    uf.count <- uf.count - 1;
    true
  end

let connected uf a b = find uf a = find uf b

let component_count uf = uf.count

(* Collect all components as a list of lists *)
let components uf =
  let n = Array.length uf.parent in
  let map = Hashtbl.create 8 in
  for i = 0 to n - 1 do
    let r = find uf i in
    let lst = try Hashtbl.find map r with Not_found -> [] in
    Hashtbl.replace map r (i :: lst)
  done;
  Hashtbl.fold (fun _ lst acc -> List.sort compare lst :: acc) map []
  |> List.sort compare

let () =
  (* --- Basic connectivity --- *)
  let uf = create 7 in
  ignore (union uf 0 1);
  ignore (union uf 1 2);
  ignore (union uf 3 4);
  ignore (union uf 5 6);

  Printf.printf "connected(0,2) = %b\n" (connected uf 0 2);  (* true *)
  Printf.printf "connected(0,3) = %b\n" (connected uf 0 3);  (* false *)
  Printf.printf "components = %d\n" (component_count uf);     (* 4: {0,1,2},{3,4},{5,6},{6} — wait, that's 3 *)

  ignore (union uf 2 3);
  Printf.printf "after union 2-3: components = %d\n" (component_count uf); (* 3 *)

  let comps = components uf in
  Printf.printf "component sets:\n";
  List.iter (fun lst ->
    Printf.printf "  {%s}\n" (String.concat "," (List.map string_of_int lst))
  ) comps;

  (* --- Graph cycle detection: union returns false if already connected --- *)
  Printf.printf "\nCycle detection:\n";
  let g = create 4 in
  let edges = [(0,1);(1,2);(2,3);(3,0)] in
  List.iter (fun (a,b) ->
    let new_union = union g a b in
    if not new_union then
      Printf.printf "  edge %d-%d creates a cycle!\n" a b
  ) edges
