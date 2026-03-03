(* Phone Number Parser — Validation Pipeline *)

let digits_only s =
  String.to_seq s |> Seq.filter (fun c -> c >= '0' && c <= '9')
  |> String.of_seq

(* Version 1: Result.bind chain *)
let validate s =
  let d = digits_only s in
  let n = String.length d in
  if n = 11 && d.[0] = '1' then Ok (String.sub d 1 10)
  else if n = 10 then Ok d
  else Error "wrong number of digits"
  |> Result.bind (fun d ->
    if d.[0] = '0' || d.[0] = '1' then Error "invalid area code"
    else Ok d)
  |> Result.bind (fun d ->
    if d.[3] = '0' || d.[3] = '1' then Error "invalid exchange"
    else Ok d)

let () =
  assert (validate "(223) 456-7890" = Ok "2234567890");
  assert (validate "(023) 456-7890" = Error "invalid area code")
