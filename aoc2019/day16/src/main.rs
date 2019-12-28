use std::iter;


struct Pattern {
    values: Vec<i32>
}

impl Pattern {
    fn new(values: Vec<i32>) -> Self {
        Self { values }
    }

    fn iteration(&self, iteration: usize) -> impl Iterator<Item=i32> {
        self.values.clone().into_iter()
            .cycle()
            .flat_map(move |value| iter::repeat(value).take(iteration))
            .skip(1)
    }
}


fn fft(n: usize, pattern: &Pattern, data: &Vec<i32>) -> Vec<i32> {
    let mut data = data.clone();
    let mut result = Vec::with_capacity(data.len());

    for _ in 0..n {
        result.clear();

        for i in 1..data.len()+1 {
            let r: i32 = pattern.iteration(i)
                .zip(data.iter())
                .skip(i - 1)
                .map(|(a, b)| (a * b) % 10)
                .sum();

            result.push(r.abs() % 10);
        }

        std::mem::swap(&mut data, &mut result);
    }

    data
}


// I don't like this solution, it only works because the target is > than half
fn fft2(r: usize, n: usize, data: &[i32]) -> Vec<i32> {
    let mut data = data.to_vec();
    let mut result = Vec::with_capacity(data.len());
    result.resize(data.len(), 0);

    for _ in 0..n {
        let mut last = 0;
        for i in (r..data.len()).rev() {
            last = (data[i] + last) % 10;
            result[i] = last;
        }

        std::mem::swap(&mut data, &mut result);
    }

    data
}


fn repeated(n: usize, data: &Vec<i32>) -> Vec<i32> {
    data.iter().cycle().take(data.len() * n).cloned().collect()
}


fn main() {
    let pattern = Pattern::new(vec![0, 1, 0, -1]);

    let _input = "12345678";
    let _input = "80871224585914546619083218645595";
    let _input = "03081770884921959731165446850517";
    let input = std::fs::read_to_string("../input.txt").unwrap();

    let input: Vec<i32> = input.trim().chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();

    // println!("{:?}", input.iter().zip(pattern.iteration(1)).collect::<Vec<_>>());

    let result = fft(100, &pattern, &input);
    let relevant = &result[0..8];
    println!("First eight: {:?}", relevant);

    let start = input[0..7].iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>().join("").parse().unwrap();
    let input = repeated(10_000, &input);

    let result = fft2(start, 100, &input[..]);
    let relevant = &result[start..start+8];
    println!("10_000 Fast FFT: {:?}", relevant);
}

