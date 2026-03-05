(* Lenses in OCaml — composable record accessors *)

type ('s,'a) lens = { get: 's -> 'a; set: 'a -> 's -> 's }

let ( |> ) x f = f x
let lens_compose l1 l2 = {
  get = (fun s -> l2.get (l1.get s));
  set = (fun a s -> l1.set (l2.set a (l1.get s)) s);
}

let over l f s = l.set (f (l.get s)) s

(* Domain *)
type coords = { lat: float; lon: float }
type location = { name: string; coords: coords }
type event = { title: string; location: location; attendees: int }

let title_l     = { get=(fun e->e.title);          set=(fun v e->{e with title=v}) }
let location_l  = { get=(fun e->e.location);       set=(fun v e->{e with location=v}) }
let coords_l    = { get=(fun l->l.coords);          set=(fun v l->{l with coords=v}) }
let lat_l       = { get=(fun c->c.lat);             set=(fun v c->{c with lat=v}) }
let attendees_l = { get=(fun e->e.attendees);       set=(fun v e->{e with attendees=v}) }

let event_lat = lens_compose (lens_compose location_l coords_l) lat_l

let () =
  let e = { title="Conf"; location={ name="Hall A"; coords={ lat=42.3;lon= -71.0 } }; attendees=100 } in
  Printf.printf "lat: %.1f\n" (event_lat.get e);
  let e2 = over event_lat (fun lat -> lat +. 1.0) e in
  Printf.printf "new lat: %.1f\n" (event_lat.get e2);
  let e3 = over attendees_l (fun n -> n*2) e in
  Printf.printf "attendees*2: %d\n" (attendees_l.get e3)
