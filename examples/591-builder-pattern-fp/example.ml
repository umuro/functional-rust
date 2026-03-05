(* Functional builder via record update in OCaml *)
type config = { host:string; port:int; timeout:float; retries:int; tls:bool }

let default_config = { host="localhost"; port=80; timeout=30.0; retries=3; tls=false }

let with_host    h c = { c with host=h }
let with_port    p c = { c with port=p }
let with_timeout t c = { c with timeout=t }
let with_tls     b c = { c with tls=b }

let () =
  let cfg =
    default_config
    |> with_host "api.example.com"
    |> with_port 443
    |> with_tls  true
    |> with_timeout 60.0
  in
  Printf.printf "host=%s port=%d tls=%b\n" cfg.host cfg.port cfg.tls;
  (* Reuse base *)
  let dev  = default_config |> with_host "dev.local" in
  let prod = default_config |> with_host "prod.example.com" |> with_tls true in
  Printf.printf "dev=%s prod=%s\n" dev.host prod.host
