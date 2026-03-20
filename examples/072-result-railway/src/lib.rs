#![allow(clippy::all)]
// 072: Railway-Oriented Programming
// Chain Results — stay on happy path or switch to error track

#[derive(Debug, Clone)]
struct Order {
    item: String,
    quantity: i32,
    price: f64,
}

// Individual validation steps
fn validate_quantity(order: Order) -> Result<Order, String> {
    if order.quantity <= 0 {
        Err("Quantity must be positive".into())
    } else {
        Ok(order)
    }
}

fn validate_price(order: Order) -> Result<Order, String> {
    if order.price <= 0.0 {
        Err("Price must be positive".into())
    } else {
        Ok(order)
    }
}

fn validate_item(order: Order) -> Result<Order, String> {
    if order.item.is_empty() {
        Err("Item name required".into())
    } else {
        Ok(order)
    }
}

// Approach 1: and_then chain
fn validate_order_chain(order: Order) -> Result<Order, String> {
    validate_item(order)
        .and_then(validate_quantity)
        .and_then(validate_price)
}

// Approach 2: Using ? operator
fn validate_order_question(order: Order) -> Result<Order, String> {
    let o = validate_item(order)?;
    let o = validate_quantity(o)?;
    validate_price(o)
}

// Approach 3: Full pipeline
fn apply_discount(pct: f64, mut order: Order) -> Result<Order, String> {
    if !(0.0..=100.0).contains(&pct) {
        Err("Invalid discount".into())
    } else {
        order.price *= 1.0 - pct / 100.0;
        Ok(order)
    }
}

fn calculate_total(order: &Order) -> f64 {
    order.quantity as f64 * order.price
}

fn process_order(order: Order, discount: f64) -> Result<f64, String> {
    let o = validate_order_chain(order)?;
    let o = apply_discount(discount, o)?;
    Ok(calculate_total(&o))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_order() {
        let o = Order {
            item: "Widget".into(),
            quantity: 5,
            price: 10.0,
        };
        assert!(validate_order_chain(o).is_ok());
    }

    #[test]
    fn test_invalid_quantity() {
        let o = Order {
            item: "Widget".into(),
            quantity: -1,
            price: 10.0,
        };
        assert_eq!(
            validate_order_chain(o).unwrap_err(),
            "Quantity must be positive"
        );
    }

    #[test]
    fn test_invalid_item() {
        let o = Order {
            item: "".into(),
            quantity: 5,
            price: 10.0,
        };
        assert_eq!(validate_order_chain(o).unwrap_err(), "Item name required");
    }

    #[test]
    fn test_process_order() {
        let o = Order {
            item: "Widget".into(),
            quantity: 5,
            price: 10.0,
        };
        assert_eq!(process_order(o, 10.0), Ok(45.0));
    }

    #[test]
    fn test_bad_discount() {
        let o = Order {
            item: "Widget".into(),
            quantity: 5,
            price: 10.0,
        };
        assert!(process_order(o, 200.0).is_err());
    }
}
