(* Example 213: Practical Lens Use — Deeply Nested Config Update *)

(* The motivating use case: real-world config management *)

type ('s, 'a) lens = {
  get : 's -> 'a;
  set : 'a -> 's -> 's;
}

let compose outer inner = {
  get = (fun s -> inner.get (outer.get s));
  set = (fun b s -> outer.set (inner.set b (outer.get s)) s);
}

let over l f s = l.set (f (l.get s)) s
let ( |>> ) = compose
let ( %~ ) l f = over l f

(* === The Config === *)
type ssl_config = {
  enabled : bool;
  cert_path : string;
  key_path : string;
}

type pool_config = {
  min_size : int;
  max_size : int;
  timeout_ms : int;
}

type db_config = {
  host : string;
  port : int;
  name : string;
  pool : pool_config;
  ssl : ssl_config;
}

type cache_config = {
  host : string;
  port : int;
  ttl_seconds : int;
}

type server_config = {
  bind_address : string;
  bind_port : int;
  db : db_config;
  cache : cache_config;
}

type app_config = {
  server : server_config;
  debug : bool;
  version : string;
  log_level : string;
}

(* === BEFORE: Without lenses (the pain) === *)
let before_update_pool_max config new_max =
  { config with
    server = { config.server with
      db = { config.server.db with
        pool = { config.server.db.pool with
          max_size = new_max } } } }

let before_enable_ssl config =
  { config with
    server = { config.server with
      db = { config.server.db with
        ssl = { config.server.db.ssl with
          enabled = true } } } }

(* === AFTER: With lenses === *)
let server_l  = { get = (fun c -> c.server); set = (fun s c -> { c with server = s }) }
let db_l      = { get = (fun s -> s.db); set = (fun d s -> { s with db = d }) }
let pool_l    = { get = (fun d -> d.pool); set = (fun p d -> { d with pool = p }) }
let ssl_l     = { get = (fun d -> d.ssl); set = (fun s d -> { d with ssl = s }) }
let cache_l   = { get = (fun s -> s.cache); set = (fun c s -> { s with cache = c }) }

let max_size_l  = { get = (fun p -> p.max_size); set = (fun m p -> { p with max_size = m }) }
let min_size_l  = { get = (fun p -> p.min_size); set = (fun m p -> { p with min_size = m }) }
let timeout_l   = { get = (fun p -> p.timeout_ms); set = (fun t p -> { p with timeout_ms = t }) }
let enabled_l   = { get = (fun s -> s.enabled); set = (fun e s -> { s with enabled = e }) }
let port_l      = { get = (fun d -> d.port); set = (fun p d -> { d with port = p }) }
let ttl_l       = { get = (fun c -> c.ttl_seconds); set = (fun t c -> { c with ttl_seconds = t }) }

(* Composed paths *)
let app_pool_max   = server_l |>> db_l |>> pool_l |>> max_size_l
let app_pool_min   = server_l |>> db_l |>> pool_l |>> min_size_l
let app_ssl_enabled = server_l |>> db_l |>> ssl_l |>> enabled_l
let app_db_port    = server_l |>> db_l |>> port_l
let app_cache_ttl  = server_l |>> cache_l |>> ttl_l

(* Clean updates! *)
let after_update_pool_max config new_max =
  app_pool_max.set new_max config

let after_enable_ssl config =
  app_ssl_enabled.set true config

(* Chain multiple updates cleanly *)
let configure_for_production config =
  config
  |> app_pool_max %~ (fun _ -> 50)
  |> app_pool_min %~ (fun _ -> 10)
  |> app_ssl_enabled %~ (fun _ -> true)
  |> app_cache_ttl %~ (fun t -> t * 2)
  |> (fun c -> { c with debug = false; log_level = "warn" })

(* === Tests === *)
let () =
  let config = {
    server = {
      bind_address = "0.0.0.0"; bind_port = 8080;
      db = {
        host = "localhost"; port = 5432; name = "mydb";
        pool = { min_size = 5; max_size = 20; timeout_ms = 5000 };
        ssl = { enabled = false; cert_path = "/etc/ssl/cert.pem"; key_path = "/etc/ssl/key.pem" };
      };
      cache = { host = "localhost"; port = 6379; ttl_seconds = 300 };
    };
    debug = true; version = "1.0"; log_level = "debug";
  } in

  (* Before and after produce same results *)
  let c1 = before_update_pool_max config 50 in
  let c2 = after_update_pool_max config 50 in
  assert (c1 = c2);

  let c3 = before_enable_ssl config in
  let c4 = after_enable_ssl config in
  assert (c3 = c4);

  (* Production config *)
  let prod = configure_for_production config in
  assert (app_pool_max.get prod = 50);
  assert (app_pool_min.get prod = 10);
  assert (app_ssl_enabled.get prod = true);
  assert (app_cache_ttl.get prod = 600);
  assert (prod.debug = false);
  assert (prod.log_level = "warn");

  (* Original unchanged *)
  assert (app_pool_max.get config = 20);

  print_endline "✓ All tests passed"
