(* Bowling *)
(* Complex state machine for bowling score calculation *)

type game = { rolls: int list; frame: int }

let new_game = { rolls = []; frame = 0 }

let roll pins game =
  if pins < 0 then Error "Negative roll is invalid"
  else if pins > 10 then Error "Pin count exceeds pins on the lane"
  else Ok { game with rolls = game.rolls @ [pins] }

let score game =
  let rolls = Array.of_list game.rolls in
  let len = Array.length rolls in
  let get i = if i < len then rolls.(i) else 0 in
  let rec go frame i =
    if frame >= 10 then Ok 0
    else if i >= len then Error "Score cannot be taken until the end of the game"
    else
      let first = get i in
      if first = 10 then (* strike *)
        match go (frame + 1) (i + 1) with
        | Error e -> Error e
        | Ok rest -> Ok (10 + get (i+1) + get (i+2) + rest)
      else
        let second = get (i + 1) in
        if first + second > 10 then Error "Pin count exceeds pins on the lane"
        else if first + second = 10 then (* spare *)
          match go (frame + 1) (i + 2) with
          | Error e -> Error e
          | Ok rest -> Ok (10 + get (i+2) + rest)
        else
          match go (frame + 1) (i + 2) with
          | Error e -> Error e
          | Ok rest -> Ok (first + second + rest)
  in
  go 0 0
