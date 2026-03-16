(* Simplified persistent vector using balanced binary tree *)
type 'a pvec = Nil | One of 'a | Two of 'a pvec * 'a pvec

let rec size = function
  | Nil -> 0 | One _ -> 1
  | Two (l, r) -> size l + size r

let rec get i = function
  | One x -> if i = 0 then x else failwith "index"
  | Two (l, r) ->
    let sl = size l in
    if i < sl then get i l else get (i - sl) r

let rec set i x = function
  | One y -> if i = 0 then One x else failwith "index"
  | Two (l, r) ->
    let sl = size l in
    if i < sl then Two (set i x l, r) else Two (l, set (i - sl) x r)

let rec push x = function
  | Nil -> One x
  | One y -> Two (One y, One x)
  | Two (l, r) ->
    if size l <= size r then Two (push x l, r) else Two (l, push x r)

let () =
  let v = Nil |> push 1 |> push 2 |> push 3 |> push 4 |> push 5 in
  Printf.printf "size = %d\n" (size v);
  for i = 0 to size v - 1 do
    Printf.printf "v[%d] = %d\n" i (get i v)
  done