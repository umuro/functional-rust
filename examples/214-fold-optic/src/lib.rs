// Example 214: Fold Optic — Read-Only Multi-Focus Aggregation
//
// A Fold is a read-only optic that focuses on multiple values and aggregates them
// without modifying the structure. Unlike a Traversal (which can also write),
// a Fold only supports reading — which simplifies its implementation considerably.
//
// The core intuition: a Fold is "give me all the values you focus on as a list,
// and I'll provide all aggregation operations (sum, count, max, any, all) for free."
//
// Composition works via flat-map: if a Fold<Team, Member> gives you Members, and a
// Fold<Member, i32> gives you scores, composing them gives Fold<Team, i32> — all
// member scores across the whole team with one composed optic.

// ---------------------------------------------------------------------------
// Approach 1: Struct-based Fold (mirrors OCaml record directly)
// ---------------------------------------------------------------------------

type ToListFn<S, A> = Box<dyn Fn(&S) -> Vec<A>>;

/// A read-only optic that focuses on zero or more values of type `A` inside `S`.
///
/// OCaml equivalent:
/// ```ocaml
/// type ('s, 'a) fold_simple = { to_list : 's -> 'a list }
/// ```
pub struct Fold<S, A> {
    to_list: ToListFn<S, A>,
}

impl<S: 'static, A: 'static> Fold<S, A> {
    /// Build a Fold from a function that extracts all focused values.
    pub fn new(to_list: impl Fn(&S) -> Vec<A> + 'static) -> Self {
        Fold {
            to_list: Box::new(to_list),
        }
    }

    /// Collect all focused values into a `Vec`.
    ///
    /// This is the primitive from which all other combinators are derived.
    pub fn collect(&self, s: &S) -> Vec<A> {
        (self.to_list)(s)
    }

    /// Count how many values are focused.
    pub fn length_of(&self, s: &S) -> usize {
        self.collect(s).len()
    }

    /// `true` iff at least one focused value satisfies `pred`.
    pub fn any_of(&self, s: &S, pred: impl Fn(&A) -> bool) -> bool {
        self.collect(s).iter().any(pred)
    }

    /// `true` iff every focused value satisfies `pred`.
    pub fn all_of(&self, s: &S, pred: impl Fn(&A) -> bool) -> bool {
        self.collect(s).iter().all(pred)
    }

    /// Find the first focused value satisfying `pred`, if any.
    pub fn find_of(&self, s: &S, pred: impl Fn(&A) -> bool) -> Option<A>
    where
        A: Clone,
    {
        self.collect(s).into_iter().find(pred)
    }

    /// Return the first focused value, if any.
    pub fn first_of(&self, s: &S) -> Option<A>
    where
        A: Clone,
    {
        self.collect(s).into_iter().next()
    }

    /// Return the last focused value, if any.
    pub fn last_of(&self, s: &S) -> Option<A>
    where
        A: Clone,
    {
        self.collect(s).into_iter().last()
    }

    /// Compose this Fold with another: Fold<S, A> + Fold<A, B> → Fold<S, B>.
    ///
    /// Composition is flat-map: for each `A` the outer fold finds, apply the
    /// inner fold to get `Vec<B>`, then concatenate all results.
    ///
    /// OCaml equivalent:
    /// ```ocaml
    /// let compose outer inner = { to_list = fun s ->
    ///   List.concat_map inner.to_list (outer.to_list s) }
    /// ```
    pub fn compose<B: 'static>(self, other: Fold<A, B>) -> Fold<S, B> {
        Fold::new(move |s: &S| {
            self.collect(s)
                .iter()
                .flat_map(|a| other.collect(a))
                .collect()
        })
    }
}

impl<S: 'static> Fold<S, i32> {
    /// Sum all focused `i32` values.
    pub fn sum_of(&self, s: &S) -> i32 {
        self.collect(s).into_iter().sum()
    }

    /// Product of all focused `i32` values.
    pub fn product_of(&self, s: &S) -> i32 {
        self.collect(s).into_iter().product()
    }

    /// Maximum of all focused `i32` values, or `None` if no values are focused.
    pub fn max_of(&self, s: &S) -> Option<i32> {
        self.collect(s).into_iter().max()
    }

    /// Minimum of all focused `i32` values, or `None` if no values are focused.
    pub fn min_of(&self, s: &S) -> Option<i32> {
        self.collect(s).into_iter().min()
    }
}

impl<S: 'static> Fold<S, f64> {
    /// Sum all focused `f64` values.
    pub fn sum_of_float(&self, s: &S) -> f64 {
        self.collect(s).into_iter().sum()
    }

    /// Maximum of all focused `f64` values, or `None` if no values are focused.
    pub fn max_of_float(&self, s: &S) -> Option<f64> {
        self.collect(s).into_iter().reduce(f64::max)
    }
}

// ---------------------------------------------------------------------------
// Approach 2: Trait-based Fold (zero-cost compile-time dispatch)
// ---------------------------------------------------------------------------
//
// The trait exposes `fold_collect`, and the blanket impl on slices/iterators
// provides `sum_all`, `any_all`, etc. for free.

/// A read-only optic trait: any type implementing this can extract a `Vec<A>`
/// from an `S`. Generic aggregation functions work uniformly for any impl.
pub trait FoldOptic<S, A> {
    /// The core focusing operation: extract all values of type `A` from `s`.
    fn fold_collect(&self, s: &S) -> Vec<A>;

    fn fold_length(&self, s: &S) -> usize {
        self.fold_collect(s).len()
    }

    fn fold_any(&self, s: &S, pred: impl Fn(&A) -> bool) -> bool {
        self.fold_collect(s).iter().any(pred)
    }

    fn fold_all(&self, s: &S, pred: impl Fn(&A) -> bool) -> bool {
        self.fold_collect(s).iter().all(pred)
    }
}

// ---------------------------------------------------------------------------
// Domain model: Team → Members → Scores
// ---------------------------------------------------------------------------

/// A team member with a name and performance scores.
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub scores: Vec<i32>,
}

/// A team composed of members.
#[derive(Debug, Clone, PartialEq)]
pub struct Team {
    pub name: String,
    pub members: Vec<Member>,
}

/// Fold<Team, Member>: focuses on all members of a team.
pub fn team_members_fold() -> Fold<Team, Member> {
    Fold::new(|team: &Team| team.members.clone())
}

/// Fold<Member, i32>: focuses on all scores of a member.
pub fn member_scores_fold() -> Fold<Member, i32> {
    Fold::new(|m: &Member| m.scores.clone())
}

/// Fold<Team, i32>: focuses on every score of every member in a team.
///
/// Built by composing `team_members_fold` with `member_scores_fold`.
/// This is the key power of Fold composition: one optic, all aggregations.
pub fn team_scores_fold() -> Fold<Team, i32> {
    team_members_fold().compose(member_scores_fold())
}

// ---------------------------------------------------------------------------
// Approach 2: Concrete trait implementations
// ---------------------------------------------------------------------------

/// Marker type for focusing on all members of a Team (zero-cost, no heap).
pub struct TeamMembersFold;

impl FoldOptic<Team, Member> for TeamMembersFold {
    fn fold_collect(&self, s: &Team) -> Vec<Member> {
        s.members.clone()
    }
}

/// Marker type for focusing on all scores of a Member.
pub struct MemberScoresFold;

impl FoldOptic<Member, i32> for MemberScoresFold {
    fn fold_collect(&self, s: &Member) -> Vec<i32> {
        s.scores.clone()
    }
}

// ---------------------------------------------------------------------------
// Generic aggregation helpers
// ---------------------------------------------------------------------------

/// Sum all `i32` values from any FoldOptic.
pub fn sum_via_fold<S>(fold: &impl FoldOptic<S, i32>, s: &S) -> i32 {
    fold.fold_collect(s).into_iter().sum()
}

/// Check if all focused values satisfy `pred`.
pub fn all_via_fold<S, A>(fold: &impl FoldOptic<S, A>, s: &S, pred: impl Fn(&A) -> bool) -> bool {
    fold.fold_collect(s).iter().all(pred)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_team() -> Team {
        Team {
            name: "Alpha".to_string(),
            members: vec![
                Member {
                    name: "Alice".to_string(),
                    scores: vec![10, 20, 30],
                },
                Member {
                    name: "Bob".to_string(),
                    scores: vec![5, 15],
                },
                Member {
                    name: "Carol".to_string(),
                    scores: vec![100],
                },
            ],
        }
    }

    // ── Approach 1: struct-based Fold ──────────────────────────────────────

    #[test]
    fn test_fold_collect_members() {
        let fold = team_members_fold();
        let team = sample_team();
        let members = fold.collect(&team);
        assert_eq!(members.len(), 3);
        assert_eq!(members[0].name, "Alice");
        assert_eq!(members[2].name, "Carol");
    }

    #[test]
    fn test_fold_length_of_counts_members() {
        let fold = team_members_fold();
        let team = sample_team();
        assert_eq!(fold.length_of(&team), 3);
    }

    #[test]
    fn test_fold_sum_of_member_scores() {
        let fold = member_scores_fold();
        let alice = Member {
            name: "Alice".to_string(),
            scores: vec![10, 20, 30],
        };
        assert_eq!(fold.sum_of(&alice), 60);
    }

    #[test]
    fn test_fold_sum_empty_is_zero() {
        let fold = member_scores_fold();
        let empty = Member {
            name: "Empty".to_string(),
            scores: vec![],
        };
        assert_eq!(fold.sum_of(&empty), 0);
    }

    #[test]
    fn test_fold_max_of_scores() {
        let fold = member_scores_fold();
        let alice = Member {
            name: "Alice".to_string(),
            scores: vec![10, 20, 30],
        };
        assert_eq!(fold.max_of(&alice), Some(30));
    }

    #[test]
    fn test_fold_max_empty_is_none() {
        let fold = member_scores_fold();
        let empty = Member {
            name: "Empty".to_string(),
            scores: vec![],
        };
        assert_eq!(fold.max_of(&empty), None);
    }

    #[test]
    fn test_fold_any_of_member_names() {
        let fold = team_members_fold();
        let team = sample_team();
        assert!(fold.any_of(&team, |m| m.name == "Bob"));
        assert!(!fold.any_of(&team, |m| m.name == "Dave"));
    }

    #[test]
    fn test_fold_all_of_members_have_scores() {
        let fold = team_members_fold();
        let team = sample_team();
        assert!(fold.all_of(&team, |m| !m.scores.is_empty()));
    }

    #[test]
    fn test_fold_find_of_returns_first_match() {
        let fold = team_members_fold();
        let team = sample_team();
        let found = fold.find_of(&team, |m| m.scores.len() == 1);
        assert_eq!(found.map(|m| m.name), Some("Carol".to_string()));
    }

    #[test]
    fn test_fold_find_of_returns_none_when_no_match() {
        let fold = team_members_fold();
        let team = sample_team();
        let found = fold.find_of(&team, |m| m.name == "Zara");
        assert!(found.is_none());
    }

    #[test]
    fn test_fold_first_of_and_last_of() {
        let fold = team_members_fold();
        let team = sample_team();
        assert_eq!(
            fold.first_of(&team).map(|m| m.name),
            Some("Alice".to_string())
        );
        assert_eq!(
            fold.last_of(&team).map(|m| m.name),
            Some("Carol".to_string())
        );
    }

    #[test]
    fn test_fold_product_of_scores() {
        let fold = member_scores_fold();
        let alice = Member {
            name: "Alice".to_string(),
            scores: vec![2, 3, 5],
        };
        assert_eq!(fold.product_of(&alice), 30);
    }

    // ── Fold composition: Team → Member → i32 ─────────────────────────────

    #[test]
    fn test_composed_fold_sum_all_team_scores() {
        // Alice: 10+20+30=60, Bob: 5+15=20, Carol: 100 → total 180
        let fold = team_scores_fold();
        let team = sample_team();
        assert_eq!(fold.sum_of(&team), 180);
    }

    #[test]
    fn test_composed_fold_max_across_all_team_scores() {
        let fold = team_scores_fold();
        let team = sample_team();
        assert_eq!(fold.max_of(&team), Some(100));
    }

    #[test]
    fn test_composed_fold_length_counts_all_scores() {
        // Alice has 3, Bob has 2, Carol has 1 → total 6
        let fold = team_scores_fold();
        let team = sample_team();
        assert_eq!(fold.length_of(&team), 6);
    }

    #[test]
    fn test_composed_fold_any_score_above_threshold() {
        let fold = team_scores_fold();
        let team = sample_team();
        assert!(fold.any_of(&team, |s| *s > 50));
        assert!(!fold.any_of(&team, |s| *s > 200));
    }

    #[test]
    fn test_composed_fold_all_scores_positive() {
        let fold = team_scores_fold();
        let team = sample_team();
        assert!(fold.all_of(&team, |s| *s > 0));
    }

    #[test]
    fn test_composed_fold_min_across_all_scores() {
        let fold = team_scores_fold();
        let team = sample_team();
        assert_eq!(fold.min_of(&team), Some(5));
    }

    #[test]
    fn test_composed_fold_empty_team() {
        let fold = team_scores_fold();
        let team = Team {
            name: "Empty".to_string(),
            members: vec![],
        };
        assert_eq!(fold.sum_of(&team), 0);
        assert_eq!(fold.length_of(&team), 0);
        assert_eq!(fold.max_of(&team), None);
    }

    // ── Approach 2: trait-based Fold ──────────────────────────────────────

    #[test]
    fn test_trait_fold_collect_members() {
        let fold = TeamMembersFold;
        let team = sample_team();
        assert_eq!(fold.fold_collect(&team).len(), 3);
    }

    #[test]
    fn test_trait_fold_length() {
        let fold = TeamMembersFold;
        let team = sample_team();
        assert_eq!(fold.fold_length(&team), 3);
    }

    #[test]
    fn test_trait_fold_any_and_all() {
        let fold = TeamMembersFold;
        let team = sample_team();
        assert!(fold.fold_any(&team, |m| m.name == "Bob"));
        assert!(fold.fold_all(&team, |m| !m.scores.is_empty()));
    }

    #[test]
    fn test_trait_member_scores_fold() {
        let fold = MemberScoresFold;
        let alice = Member {
            name: "Alice".to_string(),
            scores: vec![10, 20, 30],
        };
        assert_eq!(fold.fold_collect(&alice), vec![10, 20, 30]);
        assert_eq!(sum_via_fold(&fold, &alice), 60);
    }

    #[test]
    fn test_generic_all_via_fold() {
        let fold = MemberScoresFold;
        let alice = Member {
            name: "Alice".to_string(),
            scores: vec![10, 20, 30],
        };
        assert!(all_via_fold(&fold, &alice, |s| *s > 0));
        assert!(!all_via_fold(&fold, &alice, |s| *s > 15));
    }
}
