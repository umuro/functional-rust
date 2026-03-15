(* Example 180: PhantomData for API Safety *)
(* Connection<Closed> vs Connection<Open> — can't query closed connection *)

(* Approach 1: GADT connection states *)
type open_state = Open_tag
type closed_state = Closed_tag

type _ connection =
  | Closed : string -> closed_state connection
  | Open   : string * int -> open_state connection

let connect (Closed host) : open_state connection =
  Printf.printf "Connecting to %s...\n" host;
  Open (host, 42)  (* 42 = fake handle *)

let query (Open (host, _handle)) sql =
  Printf.printf "Query on %s: %s\n" host sql;
  "result: " ^ sql

let close (Open (host, _)) : closed_state connection =
  Printf.printf "Closing %s\n" host;
  Closed host

(* Approach 2: Module with abstract types *)
module SafeConn : sig
  type 'a conn
  type opened
  type closed
  val create : string -> closed conn
  val open_conn : closed conn -> opened conn
  val query : opened conn -> string -> string
  val close : opened conn -> closed conn
  val host : _ conn -> string
end = struct
  type opened = Open_t
  type closed = Closed_t
  type 'a conn = { host: string; handle: int option }
  let create host = { host; handle = None }
  let open_conn c = { c with handle = Some 1 }
  let query c sql = "result(" ^ c.host ^ "): " ^ sql
  let close c = { c with handle = None }
  let host c = c.host
end

(* Approach 3: Functor-based state machine *)
module type STATE = sig type t end
module Closed_s : STATE = struct type t = unit end
module Open_s : STATE = struct type t = unit end

module Connection (S : STATE) = struct
  type state = S.t
  type t = { host: string; port: int }
  let create host port = { host; port }
end

let () =
  (* Test Approach 1 *)
  let c = Closed "localhost" in
  let o = connect c in
  let result = query o "SELECT 1" in
  assert (result = "result: SELECT 1");
  let _c2 = close o in
  (* Can't do: query c "SELECT 1"  — type error! *)

  (* Test Approach 2 *)
  let c = SafeConn.create "db.example.com" in
  let o = SafeConn.open_conn c in
  let r = SafeConn.query o "SELECT * FROM users" in
  assert (String.length r > 0);
  let _c2 = SafeConn.close o in
  (* Can't do: SafeConn.query c "..."  — type error! *)

  print_endline "✓ All tests passed"
