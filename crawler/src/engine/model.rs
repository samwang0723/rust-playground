use chrono::{Duration, NaiveDate};
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
                for (index, value) in parsed_data.iter().rev().enumerate() {
                    println!("{}: {}", index, value);
                    total += value;
                    let now: u32 = (index as u32) + 1;
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
        let look_back_periods = [1, 5, 10, 20, 60];
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

        // Initialize a counter for active trading days
        let mut active_trading_days = 0;

        // Iterate through the date range and count active trading days
        let mut current_date = start_date;
        while current_date <= parsed_end_date {
            if !non_trading_dates.contains(&current_date) {
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
        let look_back_days = 5;
        // Calculate the expected result
        let expected_result = 4; // Assuming there are 4 active trading days within the given range
                                 // Call the method and get the actual result
        let actual_result = Model::total_trade_days(end_date, look_back_days);
        // Assert that the actual result matches the expected result
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn concentration() {
        let raw_data = "0829,0830,0831,0901,0904,0905,0906,0907,0908,0911,0912,0913,0914,0915,0918,0919,0920,0921,0922,0925,0926,0927,0928,1002,1003,1004,1005,1006,1011,1012,1013,1016,1017,1018,1019,1020,1023,1024,1025,1026,1027,1030,1031,1101,1102 46.65,46.8,47.4,46.95,46.55,48.15,50.3,52,52.2,51.8,55.5,54.5,54.5,52.8,51.7,51,50.2,49.2,50.4,50.7,50.1,48.4,50,50.5,49,48.3,50.1,49.1,48,47.7,46.9,46.35,47.8,49.15,51.7,51.1,49.7,52.3,53.8,53.2,54.2,54.5,54.6,53.6,54.5 -40,17,127,-261,-164,78,-283,-5876,-502,-1276,3546,-1689,751,-1009,290,105,136,404,68,-354,-94,-1476,-98,-260,-1138,-658,553,-1009,-715,-618,-353,-174,-176,-685,5004,-1779,-1435,1576,3518,-2273,1969,1752,352,-321,916";

        let model = Model {
            stock_id: "2330".to_string(),
            exchange_date: "20231013".to_string(),
            concentration: None,
        };

        let m = model
            .concentration(raw_data)
            .concentration
            .unwrap_or(Concentration(0, 0, 0, 0, 0));
        assert_eq!(m.0, 916);
    }
}
