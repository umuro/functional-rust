(* Bucket Sort in OCaml *)

let bucket_sort arr =
  let n = Array.length arr in
  if n <= 1 then () else
  let buckets = Array.make n [] in
  Array.iter (fun x ->
    let idx = min (int_of_float (x *. float_of_int n)) (n - 1) in
    buckets.(idx) <- x :: buckets.(idx)
  ) arr;
  Array.iteri (fun i lst ->
    buckets.(i) <- List.sort compare lst
  ) buckets;
  let i = ref 0 in
  Array.iter (fun lst ->
    List.iter (fun x -> arr.(!i) <- x; incr i) lst
  ) buckets

let () =
  let arr = [|0.42; 0.32; 0.23; 0.52; 0.25; 0.47; 0.51|] in
  bucket_sort arr;
  Printf.printf "Sorted: [|%s|]\n"
    (String.concat "; " (Array.to_list (Array.map string_of_float arr)))
