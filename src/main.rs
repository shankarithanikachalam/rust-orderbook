#[derive(Debug)]
enum OrderType {
    Buy,
    Sell,
}
#[derive(Debug)]
struct Order {
    id: u64,
    order_type: OrderType,
    amount: u128,
    price: f64,
}
#[derive(Debug)]
struct OrderBook {
    buy_orders: Vec<Order>,
    sell_orders: Vec<Order>,
    next_id: u64,
}
impl OrderBook {
    fn new() -> Self {
        OrderBook {
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
            next_id: 1,
        }
    }
    fn add_orders(&mut self, order_type: OrderType, amount: u128, price: f64) {
        let order = Order {
            id: self.next_id,
            order_type,
            amount,
            price,
        };
        self.next_id += 1;

        match order.order_type {
            OrderType::Buy => self.buy_orders.push(order),
            OrderType::Sell => self.sell_orders.push(order),
        }
    }

    fn show_order_book(&self) {
        for order in &self.buy_orders {
            println!("Buy Orders: {:?}", order);
        }
        for order in &self.sell_orders {
            println!("Sell Orders: {:?}", order);
        }
    }
}
fn main() {
    let enum_ordertype = OrderType::Buy;
    println!("Order Type:\n {:?}", enum_ordertype);

    let order_struct = Order {
        id: 1,
        order_type: enum_ordertype,
        amount: 100,
        price: 200.0,
    };
    println!("Order Structure:\n {:?}", order_struct);

    // let order_book = OrderBook {
    //     buy_orders: vec![Order {
    //         id: 5,
    //         order_type: OrderType::Buy,
    //         amount: 100,
    //         price: 200.0,
    //     }],
    //     sell_orders: vec![order_struct],
    //     next_id: 6,
    // };
    // println!("Order Book:\nBuy Orders: {:?} \nSell Orders: {:?} \nNext ID: {}", order_book.buy_orders,order_book.sell_orders,order_book.next_id);
    let mut order_book = OrderBook::new();
    order_book.add_orders(OrderType::Sell, 123, 12.7);
    order_book.add_orders(OrderType::Buy, 123, 12.7);

    println!(
        "My orderbook: \nBuy Orders: {:?} \nSell Orders: {:?} ",
        order_book.buy_orders, order_book.sell_orders
    );

    order_book.show_order_book();

 }
