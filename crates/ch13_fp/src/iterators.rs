use log::debug;

pub fn iterators() {
    iterator_example();
    map_example();
}

fn map_example() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

fn iterator_example() {
    debug!("in iterators !!");

    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));

    for val in v1_iter {
        debug!("Value of {val}");
    }

    assert_eq!(v1.iter().sum::<i32>(), 6);
}

#[cfg(test)]
mod tests {
    #[derive(Clone, PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    #[test]
    fn filters_by_size() {
        let sneaker = Shoe {
            size: 10,
            style: String::from("sneaker"),
        };
        let sandal = Shoe {
            size: 13,
            style: String::from("sandal"),
        };
        let boot = Shoe {
            size: 10,
            style: String::from("boot"),
        };

        let shoes = vec![sneaker.clone(), sandal.clone(), boot.clone()];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(in_my_size, vec![sneaker, boot]);
    }
}
