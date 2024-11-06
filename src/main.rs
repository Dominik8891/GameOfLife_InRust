use rand::Rng;
use std::{thread, time::{self}};
use crossterm::cursor::MoveTo;

fn main() {
    // Define the size of the game field.
    let row_numbers = 40;
    let col_numbers = row_numbers * 2; 

    let mut living_cells: i32 = 0;
    let mut count: i32 = 0;

    let dead   = ' ';
    let alive  = '*';
    let border = '■';

    let time_for_thread_sleep = time::Duration::from_millis(100);
    
    //#################################################################
    //################### generate field with stars ###################
    let mut field = generate_field(row_numbers , col_numbers, border, alive);

    show_field(&field, row_numbers, col_numbers, &living_cells);

    living_cells = count_living_cells(&field, &alive);
    //#################################################################

    //#################################################################
    //################### game loop ###################################
    loop {
        let new_field = progress_field(&field, row_numbers, col_numbers, &dead, &alive);
        show_field(&field, row_numbers, col_numbers, &living_cells);
        field = new_field;


        if count == 50 {
            break;
        }
        else if living_cells == count_living_cells(&field, &alive) {
            count += 1;
        }
        else {
            living_cells = count_living_cells(&field, &alive);
            count = 0;
        }

        thread::sleep(time_for_thread_sleep);
    }
    //#################################################################


}

fn generate_field(row_numbers: usize, col_numbers: usize, border: char, alive: char) -> Vec<Vec<char>>{
    let mut field = vec![vec![' ';col_numbers]; row_numbers];
    for x in 0..row_numbers {
        for y in 0..col_numbers {
            if x == 0 {
                field[x][y] = border;
            }
            else if x == row_numbers - 1 {
                field[x][y] = border;
            }
            else if y == 0 {
                field[x][y] = border;
            }
            else if y == col_numbers - 1 {
                field[x][y] = border;
            }
        }
        }
    field = fill_field_with_rng_stars(row_numbers, col_numbers, field, border, alive);
    return field;
}

fn fill_field_with_rng_stars(row_numbers: usize, col_numbers: usize, in_field: Vec<Vec<char>>, border: char, alive: char) -> Vec<Vec<char>>{
    let mut rng = rand::thread_rng();
    let mut field = in_field;
    for x in 0..row_numbers {
        for y in 0..col_numbers {
            if field[x][y] != border && rng.gen_range(0..100) < 50 {
               field[x][y] = alive;
            }
        }
    }
    return field;
}

fn show_field(field: &Vec<Vec<char>>, row_numbers: usize, col_numbers: usize, in_cells: &i32) {
    print!("{}",crossterm::cursor::MoveTo(0, 0));
    for x in 0..row_numbers {
        for y in 0..col_numbers {
            print!("{}", field[x][y]);
        }
        println!();
    }
    println!("\n living cells: {}", in_cells);
}

fn progress_field(in_field: &Vec<Vec<char>>, row_numbers: usize, col_numbers: usize, dead: &char, alive: &char) -> Vec<Vec<char>>  {
    let mut new_field = in_field.clone();
    for x in 1..row_numbers - 1 {
        for y in 1..col_numbers - 1 {
            let neighbors = find_neighbors(x, y, &in_field, &alive);
            new_field[x][y] = live_or_die(neighbors, in_field[x][y], &dead, &alive);
        }
    }
    return new_field;
}

fn find_neighbors(in_x_pos: usize, in_y_pos: usize, in_field: &Vec<Vec<char>>, alive: &char) -> i8 {
    let mut neighbors = 0;
    for x in in_x_pos - 1..in_x_pos + 2 {
        for y in in_y_pos - 1..in_y_pos + 2 {
            if x == in_x_pos && y == in_y_pos {
                continue;
            }
            else if in_field[x][y] == alive.clone() {
                neighbors = neighbors + 1;
            }
        }
    }
    return neighbors;
}

fn live_or_die(in_neighbors: i8, in_condition: char, dead: &char, alive: &char) -> char {
    if in_neighbors == 2 && in_condition == alive.clone() {
        return alive.clone();
    }
    else if in_neighbors == 3 {
        return alive.clone();
    }
    else {
        return dead.clone();
    }
}

fn count_living_cells(in_field: &Vec<Vec<char>>, alive: &char) -> i32 {
    let mut living_cells: i32 = 0;
    for x in 0..in_field.len() {
        for y in 0.. in_field[x].len() {
            if in_field[x][y] == *alive {
                living_cells += 1;
            }
        }
    }
    return living_cells;
}