use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::io::Write;

const MAX_COLUMNS:usize = 10;

// Create the shape in the relevant place from the input.
// Note we do not bound check as we are informed there will be no illegal input.
pub fn create_shape(shape: char, position: u64) -> Vec<HashSet<u64>> {
    let mut translated_shape: Vec<HashSet<u64>> = Vec::new();
    match shape {
        'Q'=>{
            let mut row1:HashSet<u64> = HashSet::new();
            let mut row2:HashSet<u64> = HashSet::new();
            row1.insert(position);
            row1.insert(position+1);
            row2.insert(position);
            row2.insert(position+1);

            translated_shape.push(row1);
            translated_shape.push(row2);
            translated_shape
        },
        'S'=>{
            let mut row1:HashSet<u64> = HashSet::new();
            let mut row2:HashSet<u64> = HashSet::new();
            row1.insert(position);
            row1.insert(position+1);
            row2.insert(position+1);
            row2.insert(position+2);
        
            translated_shape.push(row1);
            translated_shape.push(row2);
            translated_shape   
        },
        'Z'=>{
            let mut row1:HashSet<u64> = HashSet::new();
            let mut row2:HashSet<u64> = HashSet::new();
            row1.insert(position+1);
            row1.insert(position+2);
            row2.insert(position);
            row2.insert(position+1);
        
            translated_shape.push(row1);
            translated_shape.push(row2);
            translated_shape   
        },
        'T'=>{
            let mut row1:HashSet<u64> = HashSet::new();
            let mut row2:HashSet<u64> = HashSet::new();
            row1.insert(position+1);
            row2.insert(position);
            row2.insert(position+1);
            row2.insert(position+2);
            translated_shape.push(row1);
            translated_shape.push(row2);
            translated_shape   
        },
        'I'=>{
            let mut row1:HashSet<u64> = HashSet::new();
            row1.insert(position);
            row1.insert(position+1);
            row1.insert(position+2);
            row1.insert(position+3);
            translated_shape.push(row1);
            translated_shape   
        },
        'L'=>{
            let mut row1:HashSet<u64> = HashSet::new();
            let mut row2:HashSet<u64> = HashSet::new();
            let mut row3:HashSet<u64> = HashSet::new();
            row1.insert(position);
            row1.insert(position+1);
            row2.insert(position);
            row3.insert(position);
            translated_shape.push(row1);
            translated_shape.push(row2);
            translated_shape.push(row3);
            translated_shape   
        },
        'J'=>{
            let mut row1:HashSet<u64> = HashSet::new();
            let mut row2:HashSet<u64> = HashSet::new();
            let mut row3:HashSet<u64> = HashSet::new();
            row1.insert(position);
            row1.insert(position+1);
            row2.insert(position+1);
            row3.insert(position+1);
            translated_shape.push(row1);
            translated_shape.push(row2);
            translated_shape.push(row3);
            translated_shape   
        }
        _=> {
            translated_shape   
        }
    }
}

fn rationalise_rows(board:&mut Vec<HashSet<u64>>) -> usize {
    board.retain(|x| !x.is_empty());
    board.retain(|x| x.len() != MAX_COLUMNS);
    board.len()
}

fn add_shape_to_board(board:&mut Vec<HashSet<u64>>, shape: Vec<HashSet<u64>>, row: usize) {
    for add_row in 0 .. shape.len() {
        if row + add_row >= board.len() {
            board.push(shape[add_row].clone());
        }
        else {
            board[row+add_row].extend(shape[add_row].clone());
        }
    }
}

fn move_shape_down_board(board:&mut Vec<HashSet<u64>>, shape: Vec<HashSet<u64>>) {
    let mut current_lower_board_row = board.len();
    if current_lower_board_row == 0 {
        // why hang around?
        add_shape_to_board(board, shape, 0);
        return;
    }

    current_lower_board_row -= 1;
    
    let shape_size = shape.len();
    // Put a buffer of empty vectors on top of the top row
    for _i in 0..=shape_size {
        let new_row:HashSet<u64> = HashSet::new();    
        board.push(new_row);
    }
    
    loop {
        let mut current_shape_row = 0;
        // Compare each row of the shape against the rows above the current row.
        // If the current row is 1 and the shape has 3 row we would compare
        // shape row 0 against board row 1
        // shape row 1 against board row 2
        // shape row 2 against board row 3
        // If there is a clash the shape goes on board row 2.
        // If there is no clash decrement the board row and repeat.
        for current_board_row in current_lower_board_row .. current_lower_board_row + shape_size  {
            if board[current_board_row+current_shape_row].is_disjoint(&shape[current_shape_row]) {
                current_shape_row += 1;            
            }
            else {
                // There is a row which clashes - we are done
                add_shape_to_board(board, shape, current_lower_board_row + 1);
                return;
            }           
        };

        // We have hit the lowest possible row
        if current_lower_board_row == 0 {
            add_shape_to_board(board, shape, current_lower_board_row);
            return;
        }
        current_lower_board_row -= 1;
    };
}

fn main() -> Result<(), std::io::Error> {
    let mut board:Vec<HashSet<u64>> = Vec::with_capacity(103); // 100 rows as per spec, with 3 extra to hold the tallest shapes
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Parameter list needs to be input_file.txt output_file.txt");
        std::process::exit(1);
    }

    let input_file = File::open(args[1].clone())?;
    let mut output_file = File::create(&args[2])?; 
    let data_reader = BufReader::new(input_file);
    
    // Go through the input file line by line
    for input in data_reader.lines() {
        let mut highest_row = 0;
        let line = input?;
        // Read comma separated list of items
        let shapes = line.split(',');

        // Go through the line shape by shape
        for shape in shapes {
            let translated_shape = create_shape(shape.chars().nth(0).unwrap(), shape.chars().nth(1).unwrap() as u64);
            // Now we have the translated shape we drop it onto the board
            move_shape_down_board(&mut board, translated_shape);
            // We remove empty and full rows
            highest_row = rationalise_rows(&mut board);
        }
        // We have finished the line so we output the result
        output_file.write_all(highest_row.to_string().as_bytes())?;
        output_file.write_all(b"\n")?;
        board.clear();
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_q() {
        let q = create_shape('Q', 0);

        let mut q_test: Vec<HashSet<u64>> = Vec::new();
        let mut row1:HashSet<u64> = HashSet::new();
        let mut row2:HashSet<u64> = HashSet::new();
        row1.insert(0);
        row1.insert(1);
        row2.insert(0);
        row2.insert(1);

        q_test.push(row1);
        q_test.push(row2);
        
        assert!(q==q_test);
    }
    
    #[test]
    fn create_s() {
        let s = create_shape('S', 0);

        let mut s_test: Vec<HashSet<u64>> = Vec::new();
        let mut row1:HashSet<u64> = HashSet::new();
        let mut row2:HashSet<u64> = HashSet::new();
        row1.insert(0);
        row1.insert(1);
        row2.insert(1);
        row2.insert(2);

        s_test.push(row1);
        s_test.push(row2);
        
        assert!(s==s_test);
    }
    
    #[test]
    fn create_z() {
        let z = create_shape('Z', 0);

        let mut z_test: Vec<HashSet<u64>> = Vec::new();
        let mut row1:HashSet<u64> = HashSet::new();
        let mut row2:HashSet<u64> = HashSet::new();
        row1.insert(1);
        row1.insert(2);
        row2.insert(0);
        row2.insert(1);

        z_test.push(row1);
        z_test.push(row2);
        
        assert!(z==z_test);
    }

    #[test]
    fn create_t() {
        let t = create_shape('T', 0);

        let mut t_test: Vec<HashSet<u64>> = Vec::new();
        let mut row1:HashSet<u64> = HashSet::new();
        let mut row2:HashSet<u64> = HashSet::new();
        row1.insert(1);
        row2.insert(0);
        row2.insert(1);
        row2.insert(2);

        t_test.push(row1);
        t_test.push(row2);
        
        assert!(t==t_test);
    }
    
    #[test]
    fn create_i() {
        let i = create_shape('I', 0);

        let mut i_test: Vec<HashSet<u64>> = Vec::new();
        let mut row1:HashSet<u64> = HashSet::new();
        row1.insert(0);
        row1.insert(1);
        row1.insert(2);
        row1.insert(3);

        i_test.push(row1);
        
        assert!(i==i_test);
    }
    
    #[test]
    fn create_l() {
        let l = create_shape('L', 0);

        let mut l_test: Vec<HashSet<u64>> = Vec::new();
        let mut row1:HashSet<u64> = HashSet::new();
        let mut row2:HashSet<u64> = HashSet::new();
        let mut row3:HashSet<u64> = HashSet::new();
        row1.insert(0);
        row1.insert(1);
        row2.insert(0);
        row3.insert(0);

        l_test.push(row1);
        l_test.push(row2);
        l_test.push(row3);
        
        assert!(l==l_test);
    }
    
    #[test]
    fn create_j() {
        let j = create_shape('J', 0);

        let mut j_test: Vec<HashSet<u64>> = Vec::new();
        let mut row1:HashSet<u64> = HashSet::new();
        let mut row2:HashSet<u64> = HashSet::new();
        let mut row3:HashSet<u64> = HashSet::new();
        row1.insert(0);
        row1.insert(1);
        row2.insert(1);
        row3.insert(1);

        j_test.push(row1);
        j_test.push(row2);
        j_test.push(row3);
        
        assert!(j==j_test);
    }

    #[test]
    fn put_shape_in_board() {
        let q = create_shape('Q', 0);
        let mut board:Vec<HashSet<u64>> = Vec::with_capacity(103); // 100 rows as per spec, with 3 extra to hold the tallest shapes
        move_shape_down_board(&mut board, q); 
        assert!(board.len() == 2);
    }
    
    #[test]
    fn put_two_shapes_in_board_stacked() {
        let q = create_shape('Q', 0);
        let mut board:Vec<HashSet<u64>> = Vec::with_capacity(103); // 100 rows as per spec, with 3 extra to hold the tallest shapes
        move_shape_down_board(&mut board, q); 
        rationalise_rows(&mut board);
        assert!(board.len() == 2);
        let q2 = create_shape('Q', 0);
        move_shape_down_board(&mut board, q2); 
        rationalise_rows(&mut board);
        assert!(board.len() == 4);        
    }
    
    #[test]
    fn put_shapes_in_board_remove_base() {
        let q = create_shape('Q', 0);
        let mut board:Vec<HashSet<u64>> = Vec::with_capacity(103); // 100 rows as per spec, with 3 extra to hold the tallest shapes
        move_shape_down_board(&mut board, q);
        assert!(board.len() == 2);
        let q2 = create_shape('Q', 2);
        move_shape_down_board(&mut board, q2); 
        rationalise_rows(&mut board);
        assert!(board.len() == 2);        
        let q3 = create_shape('Q', 4);
        move_shape_down_board(&mut board, q3); 
        rationalise_rows(&mut board);
        assert!(board.len() == 2);        
        let i = create_shape('I', 6);
        move_shape_down_board(&mut board, i); 
        let highest_row = rationalise_rows(&mut board);
        assert!(highest_row == 1);     
    }

    #[test]
    fn put_two_shapes_in_board_side_by_side() {
        let q = create_shape('Q', 0);
        let mut board:Vec<HashSet<u64>> = Vec::with_capacity(103); // 100 rows as per spec, with 3 extra to hold the tallest shapes
        move_shape_down_board(&mut board, q); 
        assert!(board.len() == 2);
        let q2 = create_shape('Q', 3);
        move_shape_down_board(&mut board, q2); 
        rationalise_rows(&mut board);
        assert!(board.len() == 2);        
    }

    #[test]
    fn fill_a_row() {
        let mut board:Vec<HashSet<u64>> = Vec::with_capacity(103); // 100 rows as per spec, with 3 extra to hold the tallest shapes

        let i1 = create_shape('I', 0);
        move_shape_down_board(&mut board, i1); 
        assert!(board.len() == 1);
        
        let i2 = create_shape('I', 5);
        move_shape_down_board(&mut board, i2); 
        rationalise_rows(&mut board);
        assert!(board.len() == 1);        

        let q = create_shape('Q', 9);
        move_shape_down_board(&mut board, q);
        rationalise_rows(&mut board);
        assert!(board.len() == 1);        
        
        let highest_row = rationalise_rows(&mut board);
        assert!(highest_row == 1);     
        assert!(board.len() == 1);        
    } 
    
    #[test]
    fn fill_column() {
        let mut board:Vec<HashSet<u64>> = Vec::with_capacity(103); // 100 rows as per spec, with 3 extra to hold the tallest shapes
        for i in 0 .. 50 {
            let q = create_shape('Q', 0);
            move_shape_down_board(&mut board, q);
            rationalise_rows(&mut board);
            assert!(board.len() == (i+1) * 2); 
        }

        assert!(board.len() == 100);
        assert!(board.len() == rationalise_rows(&mut board));
        assert!(100 == rationalise_rows(&mut board));
    }

    #[test]
    fn form_an_orderly_q() {
        let mut board:Vec<HashSet<u64>> = Vec::with_capacity(103); // 100 rows as per spec, with 3 extra to hold the tallest shapes
        let q = create_shape('Q', 0);
        move_shape_down_board(&mut board, q);
        assert!(board.len() == 2);
        let q2 = create_shape('Q', 2);
        move_shape_down_board(&mut board, q2); 
        rationalise_rows(&mut board);
        assert!(board.len() == 2);        
        let q3 = create_shape('Q', 4);
        move_shape_down_board(&mut board, q3); 
        rationalise_rows(&mut board);
        assert!(board.len() == 2);        
        let q4 = create_shape('Q', 6);
        move_shape_down_board(&mut board, q4); 
        rationalise_rows(&mut board);
        assert!(board.len() == 2);        
        let q5 = create_shape('Q', 8);
        move_shape_down_board(&mut board, q5); 
        rationalise_rows(&mut board);
        assert!(board.len() == 0);        
    }
    
    #[test]
    fn make_staircase() {
        let s = create_shape('S', 0);
        let mut board:Vec<HashSet<u64>> = Vec::with_capacity(103); // 100 rows as per spec, with 3 extra to hold the tallest shapes
        move_shape_down_board(&mut board, s);
        assert!(board.len() == 2);
        let s2 = create_shape('S', 2);
        move_shape_down_board(&mut board, s2); 
        rationalise_rows(&mut board);
        assert!(board.len() == 4);        
        let s3 = create_shape('S', 4);
        move_shape_down_board(&mut board, s3); 
        rationalise_rows(&mut board);
        assert!(board.len() == 6);        
        let s4 = create_shape('S', 6);
        move_shape_down_board(&mut board, s4); 
        rationalise_rows(&mut board);
        assert!(board.len() == 8);        
    }
    
    #[test]
    fn make_intricate_pattern() {
        let mut board:Vec<HashSet<u64>> = Vec::with_capacity(103); // 100 rows as per spec, with 3 extra to hold the tallest shapes
        let s = create_shape('S', 0);
        move_shape_down_board(&mut board, s);
        assert!(board.len() == 2);
        let s2 = create_shape('S', 2);
        move_shape_down_board(&mut board, s2); 
        rationalise_rows(&mut board);
        assert!(board.len() == 4);        
        let s3 = create_shape('S', 4);
        move_shape_down_board(&mut board, s3); 
        rationalise_rows(&mut board);
        assert!(board.len() == 6);        
        let s4 = create_shape('S', 5);
        move_shape_down_board(&mut board, s4); 
        rationalise_rows(&mut board);
        assert!(board.len() == 8);        
        let q = create_shape('Q', 8);
        move_shape_down_board(&mut board, q);
        rationalise_rows(&mut board);
        assert!(board.len() == 8);
        let q2 = create_shape('Q', 8);
        move_shape_down_board(&mut board, q2); 
        rationalise_rows(&mut board);
        assert!(board.len() == 8);        
        let q3 = create_shape('Q', 8);
        move_shape_down_board(&mut board, q3); 
        rationalise_rows(&mut board);
        assert!(board.len() == 8);        
        let q4 = create_shape('Q', 8);
        move_shape_down_board(&mut board, q4); 
        rationalise_rows(&mut board);
        assert!(board.len() == 8);   
        let t = create_shape('T', 1);
        move_shape_down_board(&mut board, t); 
        rationalise_rows(&mut board);
        assert!(board.len() == 8);   
        let q5  = create_shape('Q', 1);
        move_shape_down_board(&mut board, q5); 
        rationalise_rows(&mut board);
        assert!(board.len() == 8);   
        let i  = create_shape('I', 0);
        move_shape_down_board(&mut board, i); 
        rationalise_rows(&mut board);
        assert!(board.len() == 8);   
        let q6  = create_shape('Q', 4);
        move_shape_down_board(&mut board, q6); 
        rationalise_rows(&mut board);
        assert!(board.len() == 8);   
    }
    
    #[test]
    fn make_jlq_pattern() {
        let mut board:Vec<HashSet<u64>> = Vec::with_capacity(103); // 100 rows as per spec, with 3 extra to hold the tallest shapes
        let l = create_shape('L', 0);
        let j = create_shape('J', 2);
        let l2 = create_shape('L', 4);
        let j2 = create_shape('J', 6);
        let q = create_shape('Q', 8);
        move_shape_down_board(&mut board, l);
        rationalise_rows(&mut board);
        move_shape_down_board(&mut board, j);
        rationalise_rows(&mut board);
        move_shape_down_board(&mut board, l2);
        rationalise_rows(&mut board);
        move_shape_down_board(&mut board, j2);
        rationalise_rows(&mut board);
        move_shape_down_board(&mut board, q);
        rationalise_rows(&mut board);
        assert!(board.len() == 2);   
    }
}