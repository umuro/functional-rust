(* Change (Coin Change) *)
(* Minimum coins with dynamic programming *)

let make_change ~target ~coins =
  if target = 0 then Ok []
  else if target < 0 then Error "target can't be negative"
  else
    let cache = Array.make (target + 1) None in
    cache.(0) <- Some [];
    for i = 1 to target do
      List.iter (fun coin ->
        if coin <= i then
          match cache.(i - coin) with
          | None -> ()
          | Some prev ->
            let candidate = coin :: prev in
            match cache.(i) with
            | None -> cache.(i) <- Some candidate
            | Some curr when List.length candidate < List.length curr ->
              cache.(i) <- Some candidate
            | _ -> ()
      ) coins
    done;
    match cache.(target) with
    | Some coins -> Ok (List.sort compare coins)
    | None -> Error "can't make target with given coins"
