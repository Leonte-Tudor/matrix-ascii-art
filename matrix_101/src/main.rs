use std::cmp::min;
use std::io;
use colored::*;

fn print_preamble(ind: usize) {
    println!("\n{:ind$}{}{:ind$}{}{:ind$}{}{:ind$}{}",
             " ", "====================================\n".green(),
             " ", "Congratulations, you have fooled the\n",
             " ", "     borrow checker once again\n",
             " ", "====================================\n".green());
}

fn fill_diags(m: &mut Vec<i32>, c: usize, l: usize, d: usize) {
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
}

fn stroke_diags(m: &mut Vec<i32>, c: usize, d: usize) {
    for i in 0..d {
        m[i * c + i] = 1; // main diagonal
        m[(i + 1) * c - i - 1] = 5; // anti diagonal
    }
}

fn fill_west(m: &mut Vec<i32>, c: usize, c2: usize, l: usize, d: usize, flag_mode: bool) {
    if flag_mode == true {
        for i in 1..c2 {
            for j in i..l - i {
                m[i * c + (j - i) * c + i - 1] = 4;
            }
        }
    } else {
        for i in 1..c2 {
            for j in i..d - i {
                m[i * c + (j - i) * c + i - 1] = 4;
            }
        }
    }
}

fn fill_indicators(m: &mut Vec<i32>, c: usize, c2: usize, l: usize, l2: usize, par: bool) {
    if !par {
        for i in 1..l2 / 2 {
            m[i * c + c2 - 1] = 6; // north indicator
            m[c * (l - i - 1) + c2 - 1] = 7; // south indicator
        }

        for i in 1..c2 / 2 {
            m[(c2 - 1) * c + i] = 8; // west indicator
            m[c2 * c - i - 1] = 9; // east indicator
        }
    } else {  // double the width
        for i in 1..l2 / 2 {
            m[i * c + c2 - 1] = 6;
            m[i * c + c2] = 6;
            m[c * (l - i - 1) + c2 - 1] = 7;
            m[c * (l - i - 1) + c2] = 7;
        }

        for i in 1..c2 / 2 {
            m[(c2 - 1) * c + i] = 8;
            m[c2 * c + i] = 8;
            m[c2 * c - i - 1] = 9;
            m[(c2 + 1) * c - i - 1] = 9;
        }
    }
}

fn draw_matrix(m: &mut Vec<i32>, c: usize, l2: usize, size: usize, ind: usize, par: bool) {
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

fn main() {


    let (mut l, mut c);
    let mut buff = String::new();

    println!("Insert column size (number of rows)");
    io::stdin().read_line(&mut buff).expect("number of rows should be read from stdin");
    l = buff.trim().parse().expect("input not an integer");
    buff = String::from("");

    println!("Insert row size (number of columns)");
    io::stdin().read_line(&mut buff).expect("number of columns should be read from stdin");
    c = buff.trim().parse().expect("input not an integer");

    let size = (l * c) as usize;
    let d = min(l, c);
    let indent: i32 = (c as i32 * 2 - 36) / 2;
    let mut ind: usize = if indent < 0 { 0 } else { indent as usize };
    let flag_mode = false;
    let mut par: bool;

    let mut m = vec![0; size];

    let l2 = if l % 2 == 0 { par = true; l / 2 }
                   else { par = false; l / 2 + 1 };
    let c2 = if c % 2 == 0 { par = true; c / 2 }
                   else { par = false; c / 2 + 1 };

    //let d2 = if d % 2 == 0 { par = false; d/2 } else { par = false; d/2 + 1 };

    print_preamble(ind);

    fill_diags(&mut m, c, l, d);

    fill_west(&mut m, c, c2, l, d, flag_mode);

    fill_indicators(&mut m, c, c2, l, l2, par);

    stroke_diags(&mut m, c, d);

    ind = if ind == 0 { (-indent + 1) as usize } else { 1 };

    draw_matrix(&mut m, c, l2, size, ind, par);
}
