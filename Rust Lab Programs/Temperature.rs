fn average_temperature(temperatures: &Vec<f64>) -> f64 {
    temperatures.iter().sum::<f64>() / temperatures.len() as f64
}

fn find_extremes(temperatures: &Vec<f64>) -> (f64, f64) {
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    for temp in temperatures {
        if *temp < min {
            min = *temp;
        }
        if *temp > max {
            max = *temp;
        }
    }

    (min, max)
}

fn main() {
    let temperatures = vec![20.0, 22.0, 21.0, 23.0, 24.0, 25.0, 26.0];

    let avg_temp = average_temperature(&temperatures);
    let (min_temp, max_temp) = find_extremes(&temperatures);

    println!("Average Temperature: {:.2}", avg_temp);
    println!("Minimum Temperature: {:.2}", min_temp);
    println!("Maximum Temperature: {:.2}", max_temp);
}