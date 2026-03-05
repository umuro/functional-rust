(* OCaml: Fenwick tree *)

let tree = Array.make 17 0  (* 1-indexed, size n+1 *)
let n = 16

let update i v =
  let i = ref i in
  while !i <= n do tree.(!i) <- tree.(!i) + v; i := !i + (!i land (- !i)) done

let prefix_sum i =
  let i = ref i and s = ref 0 in
  while !i > 0 do s := !s + tree.(!i); i := !i - (!i land (- !i)) done;
  !s

let range_sum l r = prefix_sum r - prefix_sum (l-1)

let () =
  List.iteri (fun i v -> update (i+1) v) [1;2;3;4;5;6;7;8];
  Printf.printf "Prefix(4) = %d\n" (prefix_sum 4);  (* 1+2+3+4=10 *)
  Printf.printf "Range(2,5) = %d\n" (range_sum 2 5) (* 2+3+4+5=14 *)
