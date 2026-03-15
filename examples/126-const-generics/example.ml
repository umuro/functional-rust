(* Example 126: Const Generics *)
(* OCaml doesn't have const generics — we simulate fixed-size arrays with modules *)

(* Approach 1: Functorized fixed-size array *)
module type SIZE = sig
  val n : int
end

module FixedArray (S : SIZE) = struct
  type 'a t = 'a array

  let create default = Array.make S.n default

  let length _ = S.n

  let get arr i =
    if i < 0 || i >= S.n then failwith "index out of bounds"
    else arr.(i)

  let set arr i v =
    if i < 0 || i >= S.n then failwith "index out of bounds"
    else arr.(i) <- v

  let map f arr = Array.map f arr

  let dot a b =
    let sum = ref 0.0 in
    for i = 0 to S.n - 1 do
      sum := !sum +. a.(i) *. b.(i)
    done;
    !sum
end

module Size3 = struct let n = 3 end
module Vec3 = FixedArray(Size3)

(* Approach 2: Matrix with size encoding *)
module type MATRIX_SIZE = sig
  val rows : int
  val cols : int
end

module Matrix (S : MATRIX_SIZE) = struct
  type t = float array array

  let create default =
    Array.init S.rows (fun _ -> Array.make S.cols default)

  let get m r c = m.(r).(c)
  let set m r c v = m.(r).(c) <- v

  let rows = S.rows
  let cols = S.cols
end

module Mat2x3 = Matrix(struct let rows = 2 let cols = 3 end)

(* Approach 3: Simple tuple-based fixed vectors *)
type vec2 = { x: float; y: float }
type vec3 = { x: float; y: float; z: float }

let vec2_add a b = { x = a.x +. b.x; y = a.y +. b.y }
let vec3_add a b = { x = a.x +. b.x; y = a.y +. b.y; z = a.z +. b.z }
let vec3_dot a b = a.x *. b.x +. a.y *. b.y +. a.z *. b.z

(* Tests *)
let () =
  (* Test functorized array *)
  let v = Vec3.create 0.0 in
  Vec3.set v 0 1.0;
  Vec3.set v 1 2.0;
  Vec3.set v 2 3.0;
  assert (Vec3.length v = 3);
  assert (Vec3.get v 1 = 2.0);

  let a = [| 1.0; 2.0; 3.0 |] in
  let b = [| 4.0; 5.0; 6.0 |] in
  assert (Vec3.dot a b = 32.0);

  (* Test matrix *)
  let m = Mat2x3.create 0.0 in
  Mat2x3.set m 0 0 1.0;
  Mat2x3.set m 1 2 5.0;
  assert (Mat2x3.get m 0 0 = 1.0);
  assert (Mat2x3.get m 1 2 = 5.0);

  (* Test record-based vectors *)
  let v1 = { x = 1.0; y = 2.0; z = 3.0 } in
  let v2 = { x = 4.0; y = 5.0; z = 6.0 } in
  let sum = vec3_add v1 v2 in
  assert (sum.x = 5.0);
  assert (vec3_dot v1 v2 = 32.0);

  Printf.printf "✓ All tests passed\n"
