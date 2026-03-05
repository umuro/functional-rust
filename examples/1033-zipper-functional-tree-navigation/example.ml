(* Zipper (Functional Tree Navigation) *)
(* Zipper data structure for efficient tree traversal and modification *)

type 'a tree = { value : 'a; left : 'a tree option; right : 'a tree option }

type 'a trail =
  | Top
  | Left of 'a * 'a tree option * 'a trail
  | Right of 'a * 'a tree option * 'a trail

type 'a zipper = { focus : 'a tree; trail : 'a trail }

let of_tree t = { focus = t; trail = Top }

let rec to_tree z =
  let t = z.focus in
  match z.trail with
  | Top -> t
  | Left (v, r, up) ->
    to_tree { focus = { value = v; left = Some t; right = r }; trail = up }
  | Right (v, l, up) ->
    to_tree { focus = { value = v; left = l; right = Some t }; trail = up }

let value z = z.focus.value

let left z = match z.focus.left with
  | None -> None
  | Some t -> Some { focus = t; trail = Left (z.focus.value, z.focus.right, z.trail) }

let right z = match z.focus.right with
  | None -> None
  | Some t -> Some { focus = t; trail = Right (z.focus.value, z.focus.left, z.trail) }

let up z = match z.trail with
  | Top -> None
  | Left (v, r, up) ->
    Some { focus = { value = v; left = Some z.focus; right = r }; trail = up }
  | Right (v, l, up) ->
    Some { focus = { value = v; left = l; right = Some z.focus }; trail = up }

let set_value v z = { z with focus = { z.focus with value = v } }
let set_left t z = { z with focus = { z.focus with left = t } }
let set_right t z = { z with focus = { z.focus with right = t } }
