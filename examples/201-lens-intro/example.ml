(* Example 201: The Nested Update Problem — Why Lenses Exist *)

(* === The Problem: Deeply Nested Record Updates === *)

type db_config = {
  host : string;
  port : int;
  name : string;
}

type server_config = {
  db : db_config;
  max_connections : int;
}

type app_config = {
  server : server_config;
  debug : bool;
  version : string;
}

(* Approach 1: Manual nested update — the pain *)
let update_db_port_manual config new_port =
  { config with
    server = { config.server with
      db = { config.server.db with
        port = new_port
      }
    }
  }

(* Look at that nesting! And it gets worse with deeper structures. *)

(* Approach 2: Helper functions for each level *)
let map_server f config = { config with server = f config.server }
let map_db f server = { server with db = f server.db }
let set_port port db = { db with port }

let update_db_port_helpers config new_port =
  config |> map_server (map_db (set_port new_port))

(* Better! But we need a helper for every field at every level. *)

(* Approach 3: Lenses — composable getters and setters *)
type ('s, 'a) lens = {
  get : 's -> 'a;
  set : 'a -> 's -> 's;
}

let compose outer inner = {
  get = (fun s -> inner.get (outer.get s));
  set = (fun a s -> outer.set (inner.set a (outer.get s)) s);
}

let server_lens = {
  get = (fun c -> c.server);
  set = (fun s c -> { c with server = s });
}

let db_lens = {
  get = (fun s -> s.db);
  set = (fun d s -> { s with db = d });
}

let port_lens = {
  get = (fun d -> d.port);
  set = (fun p d -> { d with port = p });
}

(* Compose to zoom all the way in *)
let app_db_port = compose (compose server_lens db_lens) port_lens

let update_db_port_lens config new_port =
  app_db_port.set new_port config

(* Now ANY depth is just composition! *)

(* === Tests === *)
let () =
  let config = {
    server = {
      db = { host = "localhost"; port = 5432; name = "mydb" };
      max_connections = 100;
    };
    debug = false;
    version = "1.0";
  } in

  (* Test manual *)
  let c1 = update_db_port_manual config 5433 in
  assert (c1.server.db.port = 5433);
  assert (c1.server.max_connections = 100);
  assert (c1.debug = false);

  (* Test helpers *)
  let c2 = update_db_port_helpers config 5433 in
  assert (c2.server.db.port = 5433);

  (* Test lens *)
  let c3 = update_db_port_lens config 5433 in
  assert (c3.server.db.port = 5433);
  assert (app_db_port.get c3 = 5433);

  (* All three produce same result *)
  assert (c1 = c2);
  assert (c2 = c3);

  print_endline "✓ All tests passed"
