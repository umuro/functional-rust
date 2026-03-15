(* 736: TCP Connection as Typestate — OCaml simulation *)

type disconnected = Disconnected
type connecting   = Connecting
type connected    = Connected
type closed       = Closed

(* We simulate state using a phantom wrapper *)
type 'state conn = {
  host: string;
  port: int;
  state_label: string;
}

let new_conn host port : disconnected conn =
  { host; port; state_label = "Disconnected" }

let connect (c: disconnected conn) : connected conn =
  Printf.printf "Connecting to %s:%d...\n" c.host c.port;
  { c with state_label = "Connected" }

let send (c: connected conn) msg : connected conn =
  Printf.printf "[%s:%d] Sent: %s\n" c.host c.port msg;
  c

let recv (c: connected conn) : (string * connected conn) =
  Printf.printf "[%s:%d] Recv: <data>\n" c.host c.port;
  ("response", c)

let close (c: connected conn) : closed conn =
  Printf.printf "Closing connection to %s:%d\n" c.host c.port;
  { c with state_label = "Closed" }

let () =
  let conn = new_conn "example.com" 80 in
  let conn = connect conn in
  let conn = send conn "GET / HTTP/1.1" in
  let (_data, conn) = recv conn in
  let _closed = close conn in
  ()
  (* Invalid: send conn "another message" -- OCaml would flag this for closed conn *)
