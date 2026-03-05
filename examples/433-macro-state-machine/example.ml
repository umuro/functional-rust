(* State machine via macros in OCaml with phantom types *)

(* Phantom types for state tracking *)
type unconnected
type connected
type closed

type 'state connection = {
  host: string;
  port: int;
  mutable data: string list;
}

(* Each function returns the "next state" *)
let connect host port : connected connection =
  Printf.printf "Connecting to %s:%d\n" host port;
  { host; port; data = [] }

let send (conn : connected connection) msg : connected connection =
  Printf.printf "Sending: %s\n" msg;
  { conn with data = msg :: conn.data }

let disconnect (conn : connected connection) : closed connection =
  Printf.printf "Disconnecting from %s\n" conn.host;
  { conn with data = [] }

(* Cannot send on closed connection — type error! *)
(* let bad = send (disconnect (connect "localhost" 80)) "oops" *)

let () =
  let conn = connect "api.example.com" 443 in
  let conn2 = send conn "GET / HTTP/1.1" in
  let conn3 = send conn2 "Host: api.example.com" in
  let _closed = disconnect conn3 in
  Printf.printf "Protocol followed correctly\n"
