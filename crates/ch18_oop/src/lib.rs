mod state_pattern;
mod traits;

#[cfg(test)]
mod tests {
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn average(&self) -> f64 {
            self.average
        }

        pub fn add(&mut self, value: i32) {
            self.list.push(value);

            self.update_average();
        }

        pub fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();

            self.average = total as f64 / self.list.len() as f64;
        }
    }

    #[test]
    fn test_average() {
        let mut average_collection = AveragedCollection {
            list: vec![],
            average: 0.0,
        };

        average_collection.add(1);
        assert_eq!(1f64, average_collection.average());

        average_collection.add(2);
        assert_eq!(1.5, average_collection.average());
    }
}
