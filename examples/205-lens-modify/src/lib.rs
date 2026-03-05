use std::rc::Rc;

type GetFn<S, A> = Box<dyn Fn(&S) -> A>;
type SetFn<S, A> = Box<dyn Fn(A, &S) -> S>;

/// A Lens<S, A> focuses on a field of type A inside a structure S.
/// `get` extracts the field; `set` returns a new S with the field replaced.
pub struct Lens<S, A> {
    pub get: GetFn<S, A>,
    pub set: SetFn<S, A>,
}

impl<S: 'static, A: 'static> Lens<S, A> {
    pub fn new(get: impl Fn(&S) -> A + 'static, set: impl Fn(A, &S) -> S + 'static) -> Self {
        Lens {
            get: Box::new(get),
            set: Box::new(set),
        }
    }

    /// The key operation: look at the focused value, run it through `f`, put it back.
    ///
    /// `modify lens f s  =  set lens (f (get lens s)) s`
    ///
    /// This is more composable than `set` because you don't need the old value
    /// at the call site — the Lens fetches it for you.
    pub fn modify(&self, f: impl FnOnce(A) -> A, s: &S) -> S {
        (self.set)(f((self.get)(s)), s)
    }

    /// Compose two lenses: `self` focuses S→A, `inner` focuses A→B.
    /// Result is a single Lens<S, B> that traverses both levels at once.
    pub fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
    where
        A: Clone,
    {
        let outer_get = Rc::new(self.get);
        let outer_get2 = Rc::clone(&outer_get);
        let outer_set = self.set;
        let inner_get = inner.get;
        let inner_set = inner.set;

        Lens {
            get: Box::new(move |s| inner_get(&outer_get(s))),
            set: Box::new(move |b, s| {
                let a: A = outer_get2(s);
                let a2 = inner_set(b, &a);
                outer_set(a2, s)
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Counter {
    pub count: i64,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub price: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cart {
    pub item: Item,
    pub quantity: u32,
}

// ---------------------------------------------------------------------------
// Lenses
// ---------------------------------------------------------------------------

pub fn counter_count_lens() -> Lens<Counter, i64> {
    Lens::new(
        |c: &Counter| c.count,
        |n, c| Counter {
            count: n,
            ..c.clone()
        },
    )
}

pub fn counter_label_lens() -> Lens<Counter, String> {
    Lens::new(
        |c: &Counter| c.label.clone(),
        |l, c| Counter {
            label: l,
            ..c.clone()
        },
    )
}

pub fn cart_item_lens() -> Lens<Cart, Item> {
    Lens::new(
        |cart: &Cart| cart.item.clone(),
        |item, cart| Cart {
            item,
            ..cart.clone()
        },
    )
}

pub fn item_price_lens() -> Lens<Item, f64> {
    Lens::new(
        |i: &Item| i.price,
        |p, i| Item {
            price: p,
            ..i.clone()
        },
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_counter() -> Counter {
        Counter {
            count: 5,
            label: "clicks".into(),
        }
    }

    fn sample_cart() -> Cart {
        Cart {
            item: Item {
                name: "widget".into(),
                price: 10.0,
            },
            quantity: 3,
        }
    }

    #[test]
    fn test_modify_increment() {
        let lens = counter_count_lens();
        let c = sample_counter();
        let updated = lens.modify(|n| n + 1, &c);
        assert_eq!(updated.count, 6);
        // Other fields unchanged
        assert_eq!(updated.label, "clicks");
    }

    #[test]
    fn test_modify_double() {
        let lens = counter_count_lens();
        let c = sample_counter();
        let updated = lens.modify(|n| n * 2, &c);
        assert_eq!(updated.count, 10);
    }

    #[test]
    fn test_modify_reset_to_zero() {
        let lens = counter_count_lens();
        let c = sample_counter();
        let updated = lens.modify(|_| 0, &c);
        assert_eq!(updated.count, 0);
        assert_eq!(updated.label, "clicks");
    }

    #[test]
    fn test_modify_string_field() {
        let lens = counter_label_lens();
        let c = sample_counter();
        let updated = lens.modify(|s| s.to_uppercase(), &c);
        assert_eq!(updated.label, "CLICKS");
        assert_eq!(updated.count, 5);
    }

    #[test]
    fn test_modify_negative_count() {
        let lens = counter_count_lens();
        let c = Counter {
            count: 3,
            label: "x".into(),
        };
        let updated = lens.modify(|n| -n, &c);
        assert_eq!(updated.count, -3);
    }

    #[test]
    fn test_modify_chained() {
        // Apply modify twice in sequence — each step is independent
        let lens = counter_count_lens();
        let c = sample_counter();
        let step1 = lens.modify(|n| n + 1, &c); // 5 → 6
        let step2 = lens.modify(|n| n * 2, &step1); // 6 → 12
        assert_eq!(step2.count, 12);
        // original untouched
        assert_eq!(c.count, 5);
    }

    #[test]
    fn test_modify_through_composed_lens() {
        // Lens<Cart, Item> composed with Lens<Item, f64> → Lens<Cart, f64>
        // modify doubles the price inside a cart
        let cart_price = cart_item_lens().compose(item_price_lens());
        let cart = sample_cart(); // price = 10.0
        let updated = cart_price.modify(|p| p * 2.0, &cart);
        assert_eq!(updated.item.price, 20.0);
        // Unchanged fields
        assert_eq!(updated.item.name, "widget");
        assert_eq!(updated.quantity, 3);
    }

    #[test]
    fn test_modify_does_not_mutate_original() {
        let lens = counter_count_lens();
        let original = sample_counter();
        let _updated = lens.modify(|n| n + 100, &original);
        // original is still 5
        assert_eq!(original.count, 5);
    }

    #[test]
    fn test_modify_identity_function() {
        // modify with identity function returns an equivalent struct
        let lens = counter_count_lens();
        let c = sample_counter();
        let updated = lens.modify(|n| n, &c);
        assert_eq!(updated, c);
    }
}
