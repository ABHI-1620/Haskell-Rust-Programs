fn calculate_average_temperature(temperatures: &[f64]) -> f64 {
    let sum: f64 = temperatures.iter().sum();
    sum / temperatures.len() as f64
}

fn main() {
    let temperatures = [
        22.5, 23.0, 21.7, 24.0, 22.3, 20.5, 19.8,
    ];
    println!("Full week's temperatures: {:?}", temperatures);
    let last_three_days = &temperatures[4..7];
    println!("Last 3 days' temperatures: {:?}", last_three_days);
    let avg_temperature = calculate_average_temperature(last_three_days);
    println!("Average temperature of the last 3 days: {:.2}°C", avg_temperature);
}