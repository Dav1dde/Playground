use std::fs;

#[derive(Debug)]
enum ImageError {
    InvalidData()
}

struct Image {
    data: Vec<i32>,
    width: usize,
    height: usize,
    depth: usize
}

impl Image {
    fn new(data: Vec<i32>, width: usize, height: usize) -> Result<Self, ImageError> {
        let layer_len = width * height;
        if layer_len <= 0 || data.len() % layer_len != 0 {
            return Err(ImageError::InvalidData())
        }
        let layers = data.len() / layer_len;

        Ok(Image {
            data: data,
            width: width,
            height: height,
            depth: layers
        })
    }

    fn layers(&self) -> impl Iterator<Item=&[i32]> {
        Layers::new(&self)
    }

    fn values<'a>(&'a self, x: usize, y: usize) -> impl Iterator<Item=i32> + 'a {
        let position = x + y * self.width;
        self.layers()
            .map(move |layer| layer[position])
    }

    fn decoded_value(&self, x: usize, y: usize) -> Option<i32> {
        self.values(x, y)
            .filter(|value| *value != 2)
            .nth(0)
    }

    fn decoded(&self) -> Vec<i32> {
        let mut result = Vec::with_capacity(self.width * self.height);
        result.resize(self.width * self.height, 2);

        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(value) = self.decoded_value(x, y) {
                    result[x + y * self.width] = value;
                }
            }
        }

        result
    }
}


struct Layers<'a> {
    image: &'a Image,
    current_layer: usize
}

impl<'a> Iterator for Layers<'a> {
    type Item = &'a [i32];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_layer >= self.image.depth {
            return None;
        }

        let layer_size = self.image.width * self.image.height;
        let start = layer_size * self.current_layer;
        let end = start + layer_size;

        self.current_layer = self.current_layer + 1;

        Some(&self.image.data[start..end])
    }
}

impl<'a> Layers<'a> {
    fn new(image: &'a Image) -> Self {
        Layers { image: image, current_layer: 0 }
    }
}


trait CountIf<T> {
    fn count_if<P>(&self, predicate: P) -> usize
        where P: FnMut(&T) -> bool;
}

impl<'a, T> CountIf<T> for &'a [T] {
    fn count_if<P>(&self, mut predicate: P) -> usize
        where P: FnMut(&T) -> bool
    {
        self.iter().filter(|x| predicate(x)).count()
    }
}


fn main() {
    let input = fs::read_to_string("../input.txt").unwrap()
        .trim()
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();

    let image = Image::new(input, 25, 6).unwrap();

    let a: &[i32] = image.layers()
        .min_by_key(|layer| layer.count_if(|x| *x == 0))
        .unwrap();

    let ones = a.count_if(|x| *x == 1);
    let twos = a.count_if(|x| *x == 2);

    println!("1s * 2s of layer with min 0s: {}", ones * twos);
    println!("");

    let decoded = image.decoded();
    for row in 0..image.height {
        let start = row * image.width;
        let end = start + image.width;
        let data = &decoded[start..end];

        let p: Vec<&str> = data.iter()
            .map(|x| if *x == 0 { " " } else { "\u{2588}" })
            .collect();

        println!("{}", p.join(""));
    }
}

