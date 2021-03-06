extern crate png;
extern crate djset;

use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use djset::{DjSetId, DjSetContainer};

fn gray(buf: &[u8], w: usize, x:usize, y:usize) -> u8 {
    let pos = (x+ y*w) * 4;
    let r = buf[pos] as u32;
    let g = buf[pos+1] as u32;
    let b = buf[pos+2] as u32;
    let _a = buf[pos+3] as u32;

    ((r + g + b)/ 3) as u8
}

fn black(buf: &[u8], w: usize, x:usize, y:usize, threshold: u8) -> bool {
    gray(buf, w, x, y) > threshold
}

fn main() {
    let image_path = "resources/tux.png";
    let threshold = 128;
    let decoder = png::Decoder::new(File::open(image_path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let mut ink_map: HashMap<(i32, i32), DjSetId> = HashMap::new();
    let mut dj = DjSetContainer::default();

    for x in 0..(info.width as usize) {
        for y in 0..(info.height as usize) {
            if black(&buf, info.width as usize, x, y, threshold) {
                let n = dj.add();
                ink_map.insert((x as i32, y as i32), n);
                let (xx, yy) = (x as i32 - 1, y as i32);
                ink_map.get(&(xx, yy)).map(|d| dj.union(n, d.clone()));
                let (xx, yy) = (x as i32 - 1, y as i32 - 1);
                ink_map.get(&(xx, yy)).map(|d| dj.union(n, d.clone()));
                let (xx, yy) = (x as i32, y as i32 - 1);
                ink_map.get(&(xx, yy)).map(|d| dj.union(n, d.clone()));
            }
        }
    }

    let mut connected: HashSet<DjSetId> = HashSet::new();
    for (_coord, v) in ink_map {
        let d = dj.find(v);
        connected.insert(d);
    }
    println!("Image at {} contains {} connected components [threshold={}]",
             image_path, connected.len(), threshold);
}