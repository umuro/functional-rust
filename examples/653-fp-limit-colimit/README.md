# Limits and Colimits

Universal constructions that generalize many common patterns.

## Limits (meet/AND constructions)

- **Product**: `(A, B)` - pair of types
- **Terminal**: `()` - unique element
- **Equalizer**: Elements where two functions agree
- **Pullback**: Square that commutes

## Colimits (join/OR constructions)

- **Coproduct**: `Either<A, B>` - sum type
- **Initial**: `!` - uninhabited type
- **Coequalizer**: Quotient by equivalence
- **Pushout**: Dual of pullback

## Applications

- **Pullback = JOIN**: Database join operation
- **Coproduct = OR**: Pattern matching
- **Equalizer = WHERE**: Filter by condition
