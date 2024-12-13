fn main() {
    {
        let original_array = vec![1, 2, 3];
        let mut new_array = Vec::new();
        for i in 0..original_array.len() {
            new_array.push(original_array[i] * 2);
        }
        println!("for: original_array: {:?}, new_array: {:?}", original_array, new_array);
    }

    {
        let original_array = vec![1, 2, 3];
        let mut new_array = Vec::new();
        for &value in &original_array {
            new_array.push(value * 2);
        }
        println!("for...in: original_array: {:?}, new_array: {:?}", original_array, new_array);
    }

    {
        let original_array = vec![1, 2, 3];
        let mut new_array = Vec::new();
        let mut i = 0;
        while i < original_array.len() {
            new_array.push(original_array[i] * 2);
            i += 1;
        }
        println!("while: original_array: {:?}, new_array: {:?}", original_array, new_array);
    }

    {
        let original_array = vec![1, 2, 3];
        let mut new_array = Vec::new();
        let mut i = 0;
        loop {
            new_array.push(original_array[i] * 2);
            i += 1;
            if i >= original_array.len() {
                break;
            }
        }
        println!("infinite loop: original_array: {:?}, new_array: {:?}", original_array, new_array);
    }

    {
        let original_array = vec![1, 2, 3, 4];
        let mut new_array = Vec::new();
        original_array.iter().for_each(|&value| {
            new_array.push(value * 2);
        });
        println!("forEach: original_array: {:?}, new_array: {:?}", original_array, new_array);
    }

    {
        let original_array = vec![1, 2, 3, 4];
        let new_array: Vec<_> = original_array.iter()
            .map(|&value| value * 2).collect();
        println!("map: original_array: {:?}, new_array: {:?}", original_array, new_array);
    }

    {
        let original_array = vec![1, -2, 3, 4, -5];
        let result: Vec<_> = original_array.iter()
            .filter(|&&value| value > 0).cloned().collect();
        println!("filter: original_array: {:?}, result: {:?}", original_array, result);
    }

    {
        let original_array = vec![-1, 3, 6, 7, 8];
        let result = original_array.iter()
            .find(|&&value| value > 0 && value % 2 == 0);
        println!("find: original_array: {:?}, result: {:?}", original_array, result);
    }

    {
        let original_array = vec![2, 4, 6];
        let result = original_array.iter().all(|&value| value % 2 == 0);
        println!("all: original_array: {:?}, result: {:?}", original_array, result);
    }

    {
        let original_array = vec![1, 3, 5, 6];
        let result = original_array.iter().any(|&value| value % 2 == 0);
        println!("any: original_array: {:?}, result: {:?}", original_array, result);
    }

    // Reduce (Sum)
    {
        let original_array = vec![1, 2, 3, 4];
        let result: i32 = original_array
            .iter().cloned()
            .reduce(|accumulator , current| accumulator  + current)
            .unwrap_or(0);

        println!("reduce...sumArray: original_array: {:?}, result: {:?}", original_array, result);
    }

    // Fold (Sum) Alternative
    {
        let original_array = vec![1, 2, 3, 4];
        let result: i32 = original_array.iter()
            .fold(0, |accumulator, current| accumulator + current);
        println!("fold:...sumArray: original_array: {:?}, result: {:?}", original_array, result);
    }

    // Reduce (Max)
    {
        let original_array = vec![3, 7, 2, 9, 5];
        let result = original_array.iter().cloned()
            .reduce(i32::max).unwrap_or(i32::MIN);
        println!("reduce...maxVal: original_array: {:?}, result: {:?}", original_array, result);
    }

     // Fold (Max) Alternative
     {
        let original_array = vec![3, 7, 2, 9, 5];
        let result = original_array.iter().cloned()
            .fold(i32::MIN, i32::max);
        println!("fold...maxVal: original_array: {:?}, result: {:?}", original_array, result);
    }
    
    // Reduce (Flatten)
    {
        let original_array = vec![vec![1, 2], vec![3, 4], vec![5]];
        let result: Vec<_> = original_array
            .iter().cloned()
            .reduce(|mut accumulator , current| {
                accumulator.extend(current);
                accumulator
            })
            .unwrap_or_else(Vec::new);

        println!("reduce...flattenArray: original_array: {:?}, result: {:?}", original_array, result);
    }

    // Fold (Flatten) Alternative
    {
        let original_array = vec![vec![1, 2], vec![3, 4], vec![5]];
        let result: Vec<_> = original_array
            .iter().cloned()
            .fold(Vec::new(),|mut accumulator , current| {
                accumulator.extend(current);
                accumulator
            });

        println!("fold...flattenArray: original_array: {:?}, result: {:?}", original_array, result);
    }
}
