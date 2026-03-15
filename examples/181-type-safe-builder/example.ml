(* Example 181: Type-Safe SQL-like Query Builder *)
(* Enforce SELECT before WHERE, FROM before JOIN at the type level *)

(* Approach 1: GADT-based query builder *)
type empty_q
type has_select
type has_from
type has_where

type ('select, 'from, 'where_) query = {
  select_clause: string option;
  from_clause: string option;
  where_clause: string option;
}

let empty_query : (empty_q, empty_q, empty_q) query =
  { select_clause = None; from_clause = None; where_clause = None }

(* select requires nothing *)
let select cols (q : (empty_q, 'f, 'w) query) : (has_select, 'f, 'w) query =
  { q with select_clause = Some cols }

(* from requires select *)
let from table (q : (has_select, empty_q, 'w) query) : (has_select, has_from, 'w) query =
  { q with from_clause = Some table }

(* where_ requires from *)
let where_ cond (q : (has_select, has_from, empty_q) query) : (has_select, has_from, has_where) query =
  { q with where_clause = Some cond }

let to_sql (q : (has_select, has_from, _) query) : string =
  let base = Printf.sprintf "SELECT %s FROM %s"
    (Option.get q.select_clause)
    (Option.get q.from_clause) in
  match q.where_clause with
  | None -> base
  | Some w -> base ^ " WHERE " ^ w

(* Approach 2: Functor-based builder *)
module type BUILDER_STATE = sig
  type select_state
  type from_state
end

module Query (S : BUILDER_STATE) = struct
  type t = { select: string; from_: string; where_: string option }
end

(* Approach 3: Simple fluent builder with runtime checks for comparison *)
module FluentQuery = struct
  type t = {
    select: string option;
    from_: string option;
    where_: string option;
    order_by: string option;
  }

  let create () = { select = None; from_ = None; where_ = None; order_by = None }
  let select cols q = { q with select = Some cols }
  let from_ table q = { q with from_ = Some table }
  let where_ cond q = { q with where_ = Some cond }
  let order_by col q = { q with order_by = Some col }

  let build q =
    match q.select, q.from_ with
    | Some s, Some f ->
      let base = Printf.sprintf "SELECT %s FROM %s" s f in
      let base = match q.where_ with Some w -> base ^ " WHERE " ^ w | None -> base in
      let base = match q.order_by with Some o -> base ^ " ORDER BY " ^ o | None -> base in
      Ok base
    | _ -> Error "SELECT and FROM are required"
end

let () =
  (* Test Approach 1 *)
  let q = empty_query |> select "*" |> from "users" in
  assert (to_sql q = "SELECT * FROM users");

  let q2 = empty_query |> select "name, age" |> from "users" |> where_ "age > 18" in
  assert (to_sql q2 = "SELECT name, age FROM users WHERE age > 18");

  (* This would NOT compile:
     let bad = empty_query |> from "users"  -- needs select first
     let bad = empty_query |> select "*" |> where_ "x=1"  -- needs from first
  *)

  (* Test Approach 3 *)
  let q3 = FluentQuery.(create () |> select "*" |> from_ "products" |> where_ "price > 10" |> order_by "name") in
  (match FluentQuery.build q3 with
   | Ok sql -> assert (sql = "SELECT * FROM products WHERE price > 10 ORDER BY name")
   | Error _ -> assert false);

  (match FluentQuery.(build (create ())) with
   | Ok _ -> assert false
   | Error _ -> ());

  print_endline "✓ All tests passed"
