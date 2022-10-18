use std::cmp::min;
use std::io;
use colored::*;

fn main() {


    //println!("Insert column size (number of rows)")
    // let (mut l, mut c): (i32, i32);
    // let mut buff = String::new();
    //
    // io::stdin().read_line(&mut buff).expect("number of lines should be read from stdin");
    // l = buff.trim().parse().expect("input not an integer");
    // buff = String::from("");
    //
    // io::stdin().read_line(&mut buff).expect("number of columns should be read from stdin");
    // c = buff.trim().parse().expect("input not an integer");

    /* let mut x = buff.into_bytes();

    for i in 0.. {
        println!("{}", x[i]);
    }*/

    let (l, c) = (55, 55);
    let indent: i32 = (c as i32 * 2 - 36) / 2;
    let mut ind: usize = if indent < 0 { 0 } else { indent as usize };
    let flag_mode = false;
    let mut par:bool;
    let c2 = if c % 2 == 0 { par = false; c/2 } else { par = true; c/2 + 1 };
    let l2 = if l % 2 == 0 { par = false; l/2 } else { par = false; l/2 + 1 };
    //let d2 = if d % 2 == 0 { par = false; d/2 } else { par = false; d/2 + 1 };

    println!("\n{:ind$}{}{:ind$}{}{:ind$}{}{:ind$}{}",
             " ", "====================================\n".green(),
             " ", "Congratulations, you have fooled the\n",
             " ", "     borrow checker once again\n",
             " ", "====================================\n".green());


    let size = (l * c) as usize;
    let d = min(l, c);

    let mut m = vec![0; size];


    for i in 0..d {

        // for j in ((i * c + c + i)..(i * c + l * (c - i) + i)).step_by(c){
        //     m[j] = 1;
        // }     // first try at filling matrix below main diagonal

        for j in 1..l - i {
            m[i * c + j * c + i] = 2;
        }   // fill below main diagonal

        for j in 1..i + 1 {
            m[(i + 1) * c - j * c - i - 1] = 3;
        }   // fill above anti diagonal

    }




    if flag_mode == true {
        for i in 1..c2 {
            for j in i..l - i {
                m[i * c + (j - i) * c + i - 1] = 4; // fill west region
            }
        }
    }
    else {
        for i in 1..c2 {
            for j in i..d - i {
                m[i * c + (j - i) * c + i - 1] = 4; // fill west region
            }
        }
    }

    for i in 1..l2/2 {
        m[i * c + c2 - 1] = 6; // north indicator
        m[c * (l - i - 1) + c2 - 1] = 7; // south indicator
    }

    for i in 1..c2/2 {
        m[(c2 - 1) * c + i] = 8; // west indicator
        m[c2 * c - i - 1] = 9; // east indicator
    }

    for i in 0..d {
        m[i * c + i] = 1; // main diagonal
        m[(i + 1) * c - i - 1] = 5; // anti diagonal
    }

    ind = if ind == 0 { (-indent + 1) as usize } else { 1 };


    for i in 0..size {

        if i % c as usize == 0 {
            print!("{:ind$}", "");
        }

        match m[i] {
            0 => print!("{} ", "◀".to_string().yellow()),
            1 => print!("{} ", "↘".white().dimmed()),
            2 => print!("{} ", "▲".to_string().red()),
            3 => print!("{} ", "▼".to_string().green()),
            4 => print!("{} ", "▶".to_string().blue()),
            5 => if par == false && i == (c) * (l2 - 1) + l2 - 1
                 { print!("{} ", "⤩".white().dimmed()); }
                 else { print!("{} ", "↙".white().dimmed()); },
            6 => print!("{} ", "▲".white().dimmed()),
            7 => print!("{} ", "▼".white().dimmed()),
            8 => print!("{} ", "◀".white().dimmed()),
            9 => print!("{} ", "▶".white().dimmed()),
            _ => print!("?!?"),
        }

        if (i + 1) % c as usize == 0 {
            println!();
        }
    }
    println!();
}
