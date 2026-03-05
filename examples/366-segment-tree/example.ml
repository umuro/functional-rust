(* OCaml: segment tree for range sum *)

type tree = { mutable data: int array; n: int }

let build arr =
  let n = Array.length arr in
  let t = { data=Array.make (4*n) 0; n } in
  let rec build_ v l r =
    if l=r then t.data.(v) <- arr.(l)
    else begin
      let m = (l+r)/2 in
      build_ (2*v) l m; build_ (2*v+1) (m+1) r;
      t.data.(v) <- t.data.(2*v) + t.data.(2*v+1)
    end
  in build_ 1 0 (n-1); t

let rec query t v l r ql qr =
  if qr < l || r < ql then 0
  else if ql <= l && r <= qr then t.data.(v)
  else
    let m = (l+r)/2 in
    query t (2*v) l m ql qr + query t (2*v+1) (m+1) r ql qr

let query_range t l r = query t 1 0 (t.n-1) l r

let () =
  let arr = [|1;3;5;7;9;11|] in
  let t = build arr in
  Printf.printf "Sum[1..3]: %d\n" (query_range t 1 3);  (* 3+5+7=15 *)
  Printf.printf "Sum[0..5]: %d\n" (query_range t 0 5)   (* 36 *)
