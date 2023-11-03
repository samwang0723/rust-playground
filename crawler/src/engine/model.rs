use chrono::{Datelike, Duration, NaiveDate, Weekday};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Model {
    stock_id: String,
    exchange_date: String,
    concentration: Option<Concentration>,
}

#[derive(Debug)]
struct Concentration(i32, i32, i32, i32, i32);

impl Model {
    pub fn concentration(mut self, value: &str) -> Self {
        // Step 1: Split the input string by spaces and get the last string
        let last_string = value.split_whitespace().last().expect("No data found");

        // Step 2: Split the last string by commas to get an array of strings
        let string_array: Vec<&str> = last_string.split(',').collect();

        // Step 3: Iterate through the array and parse each string into an i32
        let result: Result<Vec<i32>, _> = string_array.iter().map(|s| s.parse::<i32>()).collect();

        match result {
            Ok(parsed_data) => {
                // Successfully parsed all strings into i32
                let mut current = 0;
                let mut total = 0;
                let mut sum = [0; 5];
                let trade_days = Model::trade_days(&self.exchange_date);
                println!("{:?}", trade_days);
                for (index, value) in parsed_data.iter().rev().enumerate() {
                    total += value;
                    let now: u32 = (index as u32) + 1;
                    println!("{}: {}: {}: {}", index, value, total, now);
                    if trade_days.contains(&now) {
                        sum[current] = total;
                        current += 1;
                    }
                }
                self.concentration = Some(Concentration(sum[0], sum[1], sum[2], sum[3], sum[4]));
            }
            Err(e) => {
                // Handle parse error
                eprintln!("Failed to parse string to i32: {}", e);
            }
        }
        self
    }

    fn trade_days(end_date: &str) -> [u32; 5] {
        let look_back_periods = [0, 4, 9, 19, 59];
        let mut trading_days_array = [0; 5]; // Initialize an array to hold the results

        for (i, &period) in look_back_periods.iter().enumerate() {
            trading_days_array[i] = Model::total_trade_days(end_date, period);
        }

        trading_days_array
    }

    fn total_trade_days(end_date: &str, look_back_days: u32) -> u32 {
        // List of non-trading dates as strings
        let non_trading_dates_str = ["20231010", "20231008"];

        // Convert date string back to NaiveDate
        let parsed_end_date = NaiveDate::parse_from_str(end_date, "%Y%m%d").unwrap();

        // Parse non-trading dates and store in HashSet for efficient lookup
        let non_trading_dates: HashSet<_> = non_trading_dates_str
            .iter()
            .map(|date_str| NaiveDate::parse_from_str(date_str, "%Y%m%d").unwrap())
            .collect();

        // Calculate the start date
        let start_date = parsed_end_date - Duration::days(look_back_days as i64);
        println!("{}: {}", start_date, parsed_end_date);

        // Initialize a counter for active trading days
        let mut active_trading_days = 0;

        // Iterate through the date range and count active trading days
        let mut current_date = start_date;
        while current_date <= parsed_end_date {
            if !non_trading_dates.contains(&current_date)
                && current_date.weekday() != Weekday::Sat
                && current_date.weekday() != Weekday::Sun
            {
                active_trading_days += 1;
            }
            current_date += Duration::days(1);
        }

        active_trading_days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_trade_days() {
        // Define the end date and look back days
        let end_date = "20231013";
        let look_back_days = 8;
        // Calculate the expected result
        let expected_result = 6; // Assuming there are 4 active trading days within the given range
                                 // Call the method and get the actual result
        let actual_result = Model::total_trade_days(end_date, look_back_days);
        // Assert that the actual result matches the expected result
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn concentration() {
        let raw_data = "0829,0830,0831,0901,0904,0905,0906,0907,0908,0911,0912,0913,0914,0915,0918,0919,0920,0921,0922,0925,0926,0927,0928,1002,1003,1004,1005,1006,1011,1012,1013,1016,1017,1018,1019,1020,1023,1024,1025,1026,1027,1030,1031,1101,1102 552,555,549,548,557,552,550,542,539,536,544,541,550,558,540,538,535,527,522,525,519,522,523,533,529,520,528,532,544,550,553,545,551,540,546,556,544,544,544,531,533,532,529,528,547 -428,2418,-4000,-677,2973,-2626,-1831,-8478,-6884,-6360,-1847,-1518,5168,2712,-17717,-11724,-12536,-17889,-13464,-3626,-12418,-1125,-1729,3858,-1162,-9581,866,2608,17231,11984,9583,-2257,4446,-3918,1300,12924,-6163,-2148,1185,-10261,-1739,-5803,-5392,-2397,10537";

        let model = Model {
            stock_id: "2330".to_string(),
            exchange_date: "20231102".to_string(),
            concentration: None,
        };

        let m = model
            .concentration(raw_data)
            .concentration
            .unwrap_or(Concentration(0, 0, 0, 0, 0));
        assert_eq!(m.0, 916);
        assert_eq!(m.1, 5173);
        assert_eq!(m.2, 4502);
        assert_eq!(m.3, 916);
        assert_eq!(m.4, 916);
    }
}
