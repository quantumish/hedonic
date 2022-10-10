use std::io::{BufReader, BufRead, Write, BufWriter};
use std::fs::File;
use std::time::Instant;
use anyhow::Result;
use std::collections::HashMap;

const THRESHOLDS: [f32; 5] = [0.0001, 0.00025, 0.0005, 0.001, 0.002];

fn main() -> Result<()> {
	let mut lats = Vec::new();
	let mut longs = Vec::new();	
	// print!("Extracting the latitude and longitude of every tree in New York... ");
	let earlier = Instant::now();
	let f = File::open("../2015_Street_Tree_Census_-_Tree_Data.csv")?;
	let reader = BufReader::new(f);
	let csv = quick_csv::Csv::from_reader(reader);
	std::io::stdout().flush().unwrap();	
    for maybe_row in csv.into_iter() {
        if let Ok(row) = maybe_row {
            let mut cols = row.columns().expect("cannot convert to utf8");
			let lat = cols.nth(37).unwrap().parse::<f32>().unwrap();
			let long = cols.next().unwrap().parse::<f32>().unwrap();
			lats.push(lat);
			longs.push(long);
        } else {
            println!("cannot read next line");
			break;
        }
    }
	let done = Instant::now();
	// println!("done! ({}s)", (done-earlier).as_secs_f32());
	
	let mut tree_count: HashMap<usize, [u8; 5]> = HashMap::new();
	let f = File::open("../homes.csv")?;
	let reader = BufReader::new(f);
	let csv = quick_csv::Csv::from_reader(reader);
	for (i, maybe_row) in csv.into_iter().enumerate() {
        if let Ok(row) = maybe_row {
            let mut cols = row.columns().expect("cannot convert to utf8");
			let lat = cols.next().unwrap().parse::<f32>().unwrap();
			let long = cols.next().unwrap().parse::<f32>().unwrap();			
			let mut counts = [0u8; 5];
			for j in 0..lats.len() {				
				THRESHOLDS.iter().enumerate().map(|(i, t)| {
					if (lat - lats[j]).abs() < *t && (long - longs[j]).abs() < *t {
						counts[i] += 1;
					}
				}).collect::<Vec<()>>();				
			}			
			tree_count.insert(i, counts);			
        } else {
            println!("cannot read next line");
			break;
        }
    }

	std::fs::write("../hashmap.json", serde_json::to_string(&tree_count).unwrap()).unwrap();
	
	Ok(())
}
