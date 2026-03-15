(* 736: TCP connection as typestate — compile-time protocol enforcement in OCaml *)
(* Rust uses PhantomData<State> to ensure send/recv are only called on
   a Connected connection. OCaml achieves the same with phantom type parameters. *)

(* ── State markers ────────────────────────────────────────────────────────── *)
type disconnected  = private Disconnected_
type connected     = private Connected_
type closed        = private Closed_

(* ── Connection record ───────────────────────────────────────────────────── *)
type 'state conn = {
  host        : string;
  port        : int;
  bytes_sent  : int;
  bytes_recv  : int;
}

(* ── Disconnected operations ─────────────────────────────────────────────── *)

let new_conn host port : disconnected conn =
  { host; port; bytes_sent = 0; bytes_recv = 0 }

(* Disconnected → Connected *)
let connect (c : disconnected conn) : (connected conn, string) result =
  Printf.printf "Connecting to %s:%d ...\n" c.host c.port;
  Ok { c with bytes_sent = 0; bytes_recv = 0 }

(* ── Connected operations ────────────────────────────────────────────────── *)

(* Send data — only available when Connected *)
let send data (c : connected conn) : (connected conn, string) result =
  Printf.printf "[%s:%d] → %d bytes\n" c.host c.port (Bytes.length data);
  Ok { c with bytes_sent = c.bytes_sent + Bytes.length data }

(* Receive data — only available when Connected *)
let recv (c : connected conn) : (bytes * connected conn, string) result =
  let fake = Bytes.of_string "HTTP/1.1 200 OK\r\n" in
  Printf.printf "[%s:%d] ← %d bytes\n" c.host c.port (Bytes.length fake);
  Ok (fake, { c with bytes_recv = c.bytes_recv + Bytes.length fake })

(* Connected → Closed *)
let close (c : connected conn) : closed conn =
  Printf.printf "Closing %s:%d (sent=%d, recv=%d)\n"
    c.host c.port c.bytes_sent c.bytes_recv;
  { c with bytes_sent = c.bytes_sent; bytes_recv = c.bytes_recv }

let peer (c : connected conn) =
  Printf.sprintf "%s:%d" c.host c.port

(* ── Closed operations ───────────────────────────────────────────────────── *)
let bytes_sent (c : closed conn) = c.bytes_sent
let bytes_recv (c : closed conn) = c.bytes_recv

let () =
  (* connect then close *)
  let conn = new_conn "localhost" 8080 in
  let conn = Result.get_ok (connect conn) in
  let closed = close conn in
  assert (bytes_sent closed = 0);
  assert (bytes_recv closed = 0);
  print_endline "connect_then_close: ok";

  (* send, recv, accumulate bytes *)
  let conn2 = new_conn "localhost" 8080 in
  let conn2 = Result.get_ok (connect conn2) in
  let conn2 = Result.get_ok (send (Bytes.of_string "hello world") conn2) in
  let (_, conn2) = Result.get_ok (recv conn2) in
  let closed2 = close conn2 in
  assert (bytes_sent closed2 = 11);
  assert (bytes_recv closed2 > 0);
  print_endline "send_recv_bytes: ok";

  (* peer *)
  let conn3 = new_conn "example.com" 443 in
  let conn3 = Result.get_ok (connect conn3) in
  assert (peer conn3 = "example.com:443");
  ignore (close conn3);
  print_endline "peer: ok";

  (* The following would be a COMPILE ERROR:
       send data (new_conn "h" 80)     (* disconnected conn has no send *)
       recv (close conn)               (* closed conn has no recv *)
  *)

  print_endline "All assertions passed."
