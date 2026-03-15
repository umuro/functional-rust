(* OCaml: ordered association list *)

module OrderedMap = struct
  type ('k,'v) t = { pairs: ('k*'v) list ref; compare: 'k->'k->int }
  let make cmp = { pairs=ref []; compare=cmp }
  let insert m k v =
    m.pairs := (k,v) :: List.filter (fun (k2,_) -> m.compare k k2 <> 0) !(m.pairs)
  let find m k = List.assoc_opt k !(m.pairs)
  let to_list m = List.rev !(m.pairs)
end

let () =
  let m = OrderedMap.make compare in
  OrderedMap.insert m "b" 2; OrderedMap.insert m "a" 1; OrderedMap.insert m "c" 3;
  List.iter (fun (k,v) -> Printf.printf "%s: %d\n" k v) (OrderedMap.to_list m)
