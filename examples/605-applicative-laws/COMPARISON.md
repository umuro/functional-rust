# Applicative Laws
- Identity: pure id <*> v == v
- Composition: pure (.) <*> u <*> v <*> w == u <*> (v <*> w)
- Homomorphism: pure f <*> pure x == pure (f x)
- Interchange: u <*> pure y == pure ($ y) <*> u
