use std::error::Error;

use crate::util::reading::read_lines;

pub fn idk(path: &str) -> Result<i32, Box<dyn Error>> {
    let pairs = read_lines(path)?
        .iter()
        .into_iter()
        .filter_map(|line| {
            let pairs = line.split(",").collect::<Vec<&str>>();

            let pair1 = pairs[0].split("-").collect::<Vec<&str>>();
            let pair2 = pairs[1].split("-").collect::<Vec<&str>>();
            
            let pair1_l = pair1[0];
            let pair1_u = pair1[1];

            let pair2_l = pair2[0];
            let pair2_u = pair2[1];

            let pair1_l: i32 = pair1_l.parse().unwrap();
            let pair1_u: i32 = pair1_u.parse().unwrap();

            let pair2_l: i32 = pair2_l.parse().unwrap();
            let pair2_u: i32 = pair2_u.parse().unwrap();

            if pair1_l == 5{
                let idk = "534w5";
            }

            if pair2_l <= pair1_l && pair1_u <= pair2_u {
                Some(true)
            } else if pair1_l <= pair2_l && pair2_u <= pair2_u {
                println!("idk: {}");
                Some(true)
            } else {
                None
            }
        })
        .collect::<Vec<bool>>();
    
    println!("Debug: {:?}", pairs);
    

    Ok(pairs.len() as i32)
}
