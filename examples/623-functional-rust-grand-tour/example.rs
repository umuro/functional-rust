use std::collections::HashMap;

// ── Domain Types ─────────────────────────────────────────────────────────────
#[derive(Debug,Clone,Copy,PartialEq)]
enum Currency { USD, EUR, GBP }

#[derive(Debug,Clone)]
struct Price { amount: f64, currency: Currency }

#[derive(Debug,Clone)]
struct Product { id: String, name: String, price: Price, stock: u32 }

#[derive(Debug,Clone)]
struct OrderItem { product: Product, qty: u32 }

#[derive(Debug,Clone)]
struct Order { id: String, items: Vec<OrderItem>, discount: f64 }

// ── Error Types ───────────────────────────────────────────────────────────────
#[derive(Debug)]
enum OrderError {
    InsufficientStock { product: String, requested: u32, available: u32 },
    InvalidDiscount(f64),
    EmptyOrder,
}
impl std::fmt::Display for OrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OrderError::InsufficientStock{product,requested,available} =>
                write!(f, "{}: need {}, have {}", product, requested, available),
            OrderError::InvalidDiscount(d) => write!(f, "discount {} out of [0,1]", d),
            OrderError::EmptyOrder => write!(f, "empty order"),
        }
    }
}

// ── Pure Functions ────────────────────────────────────────────────────────────
fn item_subtotal(item: &OrderItem) -> f64 {
    item.product.price.amount * item.qty as f64
}

fn order_subtotal(order: &Order) -> f64 {
    order.items.iter().map(item_subtotal).sum()
}

fn order_total(order: &Order) -> f64 {
    order_subtotal(order) * (1.0 - order.discount)
}

// ── Validation ────────────────────────────────────────────────────────────────
fn validate_order(order: &Order) -> Result<(), OrderError> {
    if order.items.is_empty() { return Err(OrderError::EmptyOrder); }
    if order.discount < 0.0 || order.discount > 1.0 {
        return Err(OrderError::InvalidDiscount(order.discount));
    }
    for item in &order.items {
        if item.qty > item.product.stock {
            return Err(OrderError::InsufficientStock {
                product: item.product.name.clone(),
                requested: item.qty,
                available: item.product.stock,
            });
        }
    }
    Ok(())
}

// ── Builder Pattern ───────────────────────────────────────────────────────────
#[derive(Default)]
struct OrderBuilder { id: String, items: Vec<OrderItem>, discount: f64 }

impl OrderBuilder {
    fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    fn add_item(mut self, item: OrderItem) -> Self { self.items.push(item); self }
    fn discount(mut self, d: f64) -> Self { self.discount = d; self }
    fn build(self) -> Order { Order { id:self.id, items:self.items, discount:self.discount } }
}

// ── Command Pattern ───────────────────────────────────────────────────────────
#[derive(Debug,Clone)]
enum OrderCommand {
    AddItem(OrderItem),
    RemoveItem(String),
    ApplyDiscount(f64),
    Submit,
}

fn apply_command(mut order: Order, cmd: OrderCommand) -> Result<Order, OrderError> {
    match cmd {
        OrderCommand::AddItem(item) => { order.items.push(item); Ok(order) }
        OrderCommand::RemoveItem(id) => {
            order.items.retain(|i| i.product.id != id);
            Ok(order)
        }
        OrderCommand::ApplyDiscount(d) => {
            if d < 0.0 || d > 1.0 { return Err(OrderError::InvalidDiscount(d)); }
            order.discount = d;
            Ok(order)
        }
        OrderCommand::Submit => {
            validate_order(&order)?;
            Ok(order)
        }
    }
}

// ── Iterator chains (FP data processing) ─────────────────────────────────────
fn summarize_by_currency(orders: &[Order]) -> HashMap<String, f64> {
    orders.iter()
        .flat_map(|o| o.items.iter().map(|i| {
            let currency = format!("{:?}", i.product.price.currency);
            (currency, item_subtotal(i))
        }))
        .fold(HashMap::new(), |mut map, (c, amount)| {
            *map.entry(c).or_default() += amount;
            map
        })
}

fn main() {
    let widget = Product { id:"W1".into(), name:"Widget".into(), price:Price{amount:9.99,currency:Currency::USD}, stock:10 };
    let gadget = Product { id:"G1".into(), name:"Gadget".into(), price:Price{amount:24.99,currency:Currency::USD}, stock:2 };

    // Build order with builder pattern
    let order = OrderBuilder::default()
        .id("ORD-001")
        .add_item(OrderItem{product:widget.clone(), qty:2})
        .add_item(OrderItem{product:gadget.clone(), qty:1})
        .discount(0.10)
        .build();

    println!("=== Order {} ===", order.id);
    for item in &order.items {
        println!("  {} x{} = ${:.2}", item.product.name, item.qty, item_subtotal(item));
    }
    println!("  subtotal: ${:.2}", order_subtotal(&order));
    println!("  discount: {:.0}%", order.discount*100.0);
    println!("  total:    ${:.2}", order_total(&order));

    // Validate order
    match validate_order(&order) {
        Ok(()) => println!("  ✓ Valid"),
        Err(e) => println!("  ✗ Error: {}", e),
    }

    // Command pattern: process commands
    let empty = Order { id:"ORD-002".into(), items:vec![], discount:0.0 };
    let commands = vec![
        OrderCommand::AddItem(OrderItem{product:widget.clone(), qty:1}),
        OrderCommand::ApplyDiscount(0.05),
        OrderCommand::Submit,
    ];
    let result = commands.into_iter().try_fold(empty, apply_command);
    match result {
        Ok(o)  => println!("
Command-built order total: ${:.2}", order_total(&o)),
        Err(e) => println!("
Command error: {}", e),
    }

    // Invalid stock order
    let bad_order = OrderBuilder::default()
        .id("BAD-001")
        .add_item(OrderItem{product:gadget, qty:100})  // 100 > stock of 2
        .build();
    match validate_order(&bad_order) {
        Ok(()) => println!("Expected error!"),
        Err(e) => println!("
Expected error: {}", e),
    }

    // FP data processing
    let orders = vec![order.clone()];
    let summary = summarize_by_currency(&orders);
    for (currency, total) in &summary {
        println!("
{} total: ${:.2}", currency, total);
    }

    // Pure iterator pipeline: top 5 most expensive items
    let mut items: Vec<(&str, f64)> = order.items.iter()
        .map(|i| (i.product.name.as_str(), item_subtotal(i)))
        .collect();
    items.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
    println!("
Items by cost:");
    for (name, cost) in items.iter().take(5) {
        println!("  {} ${:.2}", name, cost);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_product(id: &str, price: f64, stock: u32) -> Product {
        Product { id:id.into(), name:id.into(), price:Price{amount:price, currency:Currency::USD}, stock }
    }
    #[test] fn order_total_with_discount() {
        let p = make_product("P1", 10.0, 5);
        let o = Order { id:"T".into(), items:vec![OrderItem{product:p,qty:2}], discount:0.1 };
        assert!((order_total(&o) - 18.0).abs() < 1e-10);
    }
    #[test] fn validate_empty() {
        let o = Order{id:"".into(),items:vec![],discount:0.0};
        assert!(matches!(validate_order(&o), Err(OrderError::EmptyOrder)));
    }
    #[test] fn validate_bad_discount() {
        let p = make_product("P",1.0,10);
        let o = Order{id:"".into(),items:vec![OrderItem{product:p,qty:1}],discount:1.5};
        assert!(matches!(validate_order(&o), Err(OrderError::InvalidDiscount(_))));
    }
    #[test] fn validate_insufficient_stock() {
        let p = make_product("P",1.0,1);
        let o = Order{id:"".into(),items:vec![OrderItem{product:p,qty:5}],discount:0.0};
        assert!(matches!(validate_order(&o), Err(OrderError::InsufficientStock{..})));
    }
}
