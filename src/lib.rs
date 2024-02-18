pub fn frac_to_scaled(frac: String, scale: i32) -> String {
    let mut exploded: Vec<String> = frac
        .split_whitespace()
        .map(|z| z.to_string())
        .collect();
    //println!("{:?}", exploded);

    let index: usize;
    if exploded.get(1).is_some() {
        index = 1
    } else {
        index = 0
    }

    let mut exp_frac: Vec<char> = exploded[index].chars().collect();

    let numerator: i32 = exp_frac[0]
        .to_string()
        .parse::<i32>()
        .unwrap();

    exp_frac[0] = (numerator * scale).to_string().parse().unwrap();
    //println!("{:?}", exp_frac[0]);
    //println!("{:?}", exp_frac);

    let numerator: i32 = exp_frac[0]
        .to_string()
        .parse::<i32>()
        .unwrap();

    let denominator: i32 = exp_frac[2]
        .to_string()
        .parse::<i32>()
        .unwrap();

    let quotient = numerator / denominator;
    let modulo = numerator % denominator;
    let gcd = gcd(denominator, numerator);
    //println!("The quotient is {:?}", quotient);
    //println!("The modulo is {:?}", modulo);
    //println!("The gcd is {:?}", gcd);

    if quotient != 0 && index == 0 {

        if modulo != 0 {
            let result = format!("{} {}/{}", quotient, (modulo / gcd), (denominator / gcd));

            //println!("{:?}", result);
            result.to_string()

        } else {
            exploded[0] = quotient.to_string();
            //println!("{:?}", exploded);
            let result = &exploded[0];
            
            //println!("{:?}", result);
            result.to_string()

        }

    } else if quotient != 0 && index == 1 {

        let whole = exploded[0].parse::<i32>().unwrap();
        //println!("The whole is: {:?}", whole);

        if modulo != 0 {
            let result = format!("{} {}/{}", ((whole * scale) + quotient), (modulo / gcd), (denominator / gcd));
            
            //println!("{:?}", result);
            result.to_string()

        } else {
            exploded[0] = ((whole * scale) + quotient).to_string();
            //println!("{:?}", exploded);
            let result = &exploded[0];
            
            //println!("{:?}", result);
            result.to_string()

        }

    } else if quotient == 0 && index == 0 {

        if modulo != 0 {
            let result = format!("{}/{}", (modulo / gcd), (denominator / gcd));

            //println!("{:?}", result);
            result.to_string()

        } else {

            let collapsed: String = exp_frac.iter().collect();
            //println!("{:?}", collapsed);
            exploded[index] = collapsed;
            //println!("{:?}", exploded);
            let result = &exploded[0];
            
            //println!("{:?}", result);
            result.to_string()
        }

    } else if quotient == 0 && index == 1 {

        let whole = exploded[0].parse::<i32>().unwrap();
        //println!("The whole is: {:?}", whole);
        exploded[0] = (whole * scale).to_string();
        let collapsed: String = exp_frac.iter().collect();
        //println!("{:?}", collapsed);
        exploded[index] = collapsed;
        //println!("{:?}", exploded);
        let result = exploded.join(" ");
        
        //println!("{:?}", result);
        result.to_string()

    } else {
        unreachable!("This is unreachable!");
    }
    //println!("");
}

// pseudocode for euclidean algorithm from: https://en.wikipedia.org/wiki/Euclidean_algorithm
// function gcd(a, b)
//     while b ≠ 0
//         t := b
//         b := a mod b
//         a := t
//     return a

pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn input_to_scaled(input: &String, initial: &Vec<String>, replacew: &Vec<String>) -> String {

    let chopped: Vec<String> = input
        .split_inclusive(&[' ', '\n'][..])
        .map(|c| c.trim_end_matches(' ').to_string())
        .collect();
    //println!("{:?}", chopped);
    let mut result: Vec<String> = Vec::new();

    let zipped = initial.iter().zip(replacew.iter());
    //println!("Zipped: {:?}", zipped);

    let mut index = 0;
    for (initial, replacement) in zipped {
        //println!("{:?}, {:?}", initial, replacement);
        while index < chopped.len() {
            let cur_elem = &chopped[index];
            if initial == cur_elem {
                result.push(replacement.to_string());
                index += 1;
                break
            } 
            result.push(cur_elem.to_string());
            index += 1;
        }
    }

    while index < chopped.len() {
        let cur_elem = &chopped[index];
        result.push(cur_elem.to_string());
        index += 1;
    }

    //println!("{:?}", line);
    //println!("{:?}", result);

    let mut collapsed: String = result.join(" ");
    collapsed = collapsed.replace("\n ", "\n");

    //println!("{:?}", collapsed);
    collapsed

}
