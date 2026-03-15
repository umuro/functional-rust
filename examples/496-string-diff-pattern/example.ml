(* 496. Edit distance – OCaml *)
let levenshtein s t =
  let m=String.length s and n=String.length t in
  let d=Array.init (m+1) (fun i -> Array.init (n+1) (fun j ->
    if i=0 then j else if j=0 then i else 0)) in
  for i=1 to m do for j=1 to n do
    d.(i).(j) <-
      if s.[i-1]=t.[j-1] then d.(i-1).(j-1)
      else 1 + min d.(i-1).(j) (min d.(i).(j-1) d.(i-1).(j-1))
  done done;
  d.(m).(n)

let () =
  let pairs = [("kitten","sitting");("hello","hello");("abc","xyz");("","abc")] in
  List.iter (fun (a,b) ->
    Printf.printf "dist('%s','%s')=%d\n" a b (levenshtein a b)
  ) pairs
