/*
This struct serves as the foundational building block for time series data,
upon which more complex operations can be constructed. Leveraging Rust's
zero-cost abstractions, data scientists can implement algorithms to calculate
moving averages, identify anomalies, or detect seasonal patterns without
sacrificing performance.
Crates such as
`chrono` for date and time handling, `timeseries` for efficient time series
containers, and `ta` for technical analysis, provide the tools necessary to
dissect temporal data. Utilizing these crates, one can seamlessly convert
timestamps, align series from different sources, and apply statistical
methods to extract meaningful information from the noise.
*/

use chrono::{DateTime, Duration, Utc};
use std::collections::VecDeque;

/*
Simply a Queue of Vectors, FIFO
A double-ended queue implemented with a growable ring buffer.
The “default” usage of this type as a queue is to use push_back() to add to the queue, and pop_front() to remove from the queue. 
extend() and append() push onto the back in this manner, and iterating over VecDeque goes front to back.

A VecDeque with a known list of items can be initialized from an array:
let deq = VecDeque::from([-1, 0, 1]);
*/

fn calculating_moving_average(prices: &VecDeque<f64>, window_size: usize) -> f64 {
    prices.iter()
    .take(window_size)
    .sum::<f64>::() / window_size as f64
}


struct StockTick {
    timestamp: DateTime<Utc>,
    price: f64,
    volume: f64,
}

impl StockTick {
    fn new(timestamp: DateTime<Utc>, price: f64, volume: f64) -> Self {
        StockTick {
            timestamp,
            price,
            volume,
        }
    }
}

fn main() {
    // Assume `stock_ticks` is a VecDeque<StockTick> with data sorted by
    // timestamp.
    let mut moving_averages = VecDeque::new();
    let window_size = 5;
    let mut price_window = VecDeque::new();

    // Keeps the queue to a size of 5
    for tick in stock_ticks {
        price_window.push_back(tick_price);
        if price_window.len() > window_size {
            price_window.pop_front()
        }

        if price_window == window_size {
            let ma = calculating_moving_average(&price_window, window_size);
            moving_averages.push_back((tick.timestamp, ma));
        }
    }

    // println!();
}
