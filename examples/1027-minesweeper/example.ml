(* Minesweeper *)
(* 2D grid annotation with neighbor counting *)

let annotate strings =
  let rows = Array.of_list strings in
  let h = Array.length rows in
  if h = 0 then [] else
  let w = String.length rows.(0) in
  let grid = Array.init h (fun r -> Array.init w (fun c -> rows.(r).[c])) in
  let count r c =
    let n = ref 0 in
    for dr = -1 to 1 do for dc = -1 to 1 do
      if not (dr = 0 && dc = 0) then
        let r' = r + dr and c' = c + dc in
        if r' >= 0 && r' < h && c' >= 0 && c' < w && grid.(r').(c') = '*' then
          incr n
    done done; !n
  in
  List.init h (fun r ->
    String.init w (fun c ->
      if grid.(r).(c) = '*' then '*'
      else match count r c with 0 -> ' ' | n -> Char.chr (n + 48)))
