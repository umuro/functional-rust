(* Suffix Array — O(n log² n) prefix doubling + O(n) LCP (Kasai) *)

let build_sa s =
  let n    = String.length s in
  let sa   = Array.init n (fun i -> i) in
  let rank = Array.init n (fun i -> Char.code s.[i]) in
  let tmp  = Array.make n 0 in

  let gap = ref 1 in
  while !gap < n do
    let g = !gap in
    let cmp i j =
      let ri = rank.(i) and rj = rank.(j) in
      if ri <> rj then compare ri rj
      else
        let ri2 = if i + g < n then rank.(i + g) else (-1) in
        let rj2 = if j + g < n then rank.(j + g) else (-1) in
        compare ri2 rj2
    in
    Array.sort cmp sa;
    (* Update ranks *)
    tmp.(sa.(0)) <- 0;
    for i = 1 to n - 1 do
      tmp.(sa.(i)) <- tmp.(sa.(i-1)) + (if cmp sa.(i-1) sa.(i) = 0 then 0 else 1)
    done;
    Array.blit tmp 0 rank 0 n;
    gap := !gap * 2
  done;
  sa

(* Kasai's LCP array — O(n) *)
let build_lcp s sa =
  let n    = String.length s in
  let rank = Array.make n 0 in
  Array.iteri (fun i v -> rank.(v) <- i) sa;
  let lcp  = Array.make n 0 in
  let k    = ref 0 in
  for i = 0 to n - 1 do
    if rank.(i) > 0 then begin
      let j = sa.(rank.(i) - 1) in
      while i + !k < n && j + !k < n && s.[i + !k] = s.[j + !k] do
        incr k
      done;
      lcp.(rank.(i)) <- !k;
      if !k > 0 then decr k
    end
  done;
  lcp

let sa_search s sa pattern =
  let n = Array.length sa in
  let m = String.length pattern in
  (* Binary search: left bound *)
  let lo = ref 0 and hi = ref n in
  while !lo < !hi do
    let mid = (!lo + !hi) / 2 in
    if String.sub s sa.(mid) (min m (String.length s - sa.(mid))) < pattern then
      lo := mid + 1
    else hi := mid
  done;
  let left = !lo in
  hi := n;
  while !lo < !hi do
    let mid = (!lo + !hi) / 2 in
    let suf = String.sub s sa.(mid) (min m (String.length s - sa.(mid))) in
    if suf <= pattern then lo := mid + 1
    else hi := mid
  done;
  (* Collect positions *)
  let positions = Array.sub sa left (!lo - left) in
  Array.sort compare positions;
  Array.to_list positions

let () =
  let s  = "banana" in
  let sa = build_sa s in
  let lcp = build_lcp s sa in
  Printf.printf "String: %S\n" s;
  Printf.printf "SA:  [%s]\n" (String.concat "," (Array.to_list (Array.map string_of_int sa)));
  Printf.printf "LCP: [%s]\n" (String.concat "," (Array.to_list (Array.map string_of_int lcp)));
  Printf.printf "Suffixes in order:\n";
  Array.iter (fun i -> Printf.printf "  %d: %S\n" i (String.sub s i (String.length s - i))) sa;
  let pos = sa_search s sa "an" in
  Printf.printf "Search 'an': [%s]\n"
    (String.concat "," (List.map string_of_int pos))
