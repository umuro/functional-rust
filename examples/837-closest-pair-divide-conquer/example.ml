(* Closest Pair of Points O(n log n) in OCaml *)

type point = { x: float; y: float }

let dist a b =
  let dx = a.x -. b.x and dy = a.y -. b.y in
  sqrt (dx *. dx +. dy *. dy)

(* Brute force for n ≤ 3 *)
let brute_force pts =
  let n = Array.length pts in
  let best = ref infinity in
  let pair = ref (pts.(0), pts.(0)) in
  for i = 0 to n - 1 do
    for j = i + 1 to n - 1 do
      let d = dist pts.(i) pts.(j) in
      if d < !best then begin best := d; pair := (pts.(i), pts.(j)) end
    done
  done;
  (!best, !pair)

(* Strip scan: check points within delta of the dividing line, sorted by y *)
let strip_closest (strip : point list) (delta : float) : float =
  let arr = Array.of_list (List.sort (fun a b -> compare a.y b.y) strip) in
  let n = Array.length arr in
  let best = ref delta in
  for i = 0 to n - 1 do
    let j = ref (i + 1) in
    while !j < n && arr.(!j).y -. arr.(i).y < !best do
      let d = dist arr.(i) arr.(!j) in
      if d < !best then best := d;
      incr j
    done
  done;
  !best

(* Divide and conquer *)
let rec closest_pair_rec (pts_x : point array) : float =
  let n = Array.length pts_x in
  if n <= 3 then fst (brute_force pts_x)
  else begin
    let mid = n / 2 in
    let mid_x = pts_x.(mid).x in
    let left = Array.sub pts_x 0 mid in
    let right = Array.sub pts_x mid (n - mid) in
    let dl = closest_pair_rec left in
    let dr = closest_pair_rec right in
    let delta = min dl dr in
    (* Build strip: points within delta of the dividing line *)
    let strip = Array.to_list pts_x
      |> List.filter (fun p -> abs_float (p.x -. mid_x) < delta) in
    min delta (strip_closest strip delta)
  end

let closest_pair (points : point list) : float =
  let pts = Array.of_list (List.sort (fun a b -> compare a.x b.x) points) in
  closest_pair_rec pts

let () =
  let points = [
    {x=2.0;y=3.0}; {x=12.0;y=30.0}; {x=40.0;y=50.0};
    {x=5.0;y=1.0};  {x=12.0;y=10.0}; {x=3.0;y=4.0};
  ] in
  let d = closest_pair points in
  Printf.printf "Closest pair distance: %.4f\n" d;
  (* Brute force verification *)
  let arr = Array.of_list points in
  let (bf, _) = brute_force arr in
  Printf.printf "Brute force distance:  %.4f\n" bf;
  Printf.printf "Match: %b\n" (abs_float (d -. bf) < 1e-9)
