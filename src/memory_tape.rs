const INITIAL_POS: usize = 0;
const FINAL_POS: usize = 30_000 - 1;
const TAPE_SIZE: usize = 30_000 - 1;
const CELLS_IN_SNIPLET: usize = 11;

pub struct MemoryTape {
    block: [u8; TAPE_SIZE],
    pub head_position: usize,
}

impl MemoryTape {
    pub fn new() -> MemoryTape {
        MemoryTape {
            block: [0; TAPE_SIZE],
            head_position: 0,
        }
    }

    pub fn increment(&mut self) {
        let current_value = self.block[self.head_position];
        let should_overflow = current_value == 255;

        self.block[self.head_position] = if(should_overflow){
            0
        }else{
            current_value + 1
        };
    }

    pub fn decrement(&mut self) {
        let current_value = self.block[self.head_position];
        let should_overflow = current_value == 0;

        self.block[self.head_position] = if(should_overflow){
            255
        }else{
            current_value - 1
        };
    }

    pub fn move_left(&mut self) {
        let current_position = self.head_position;
        let can_move = current_position != INITIAL_POS;

        self.head_position = if(can_move) {
            current_position-1
        } else {
            current_position
        };
    }

    pub fn move_right(&mut self) {
        let current_position = self.head_position;
        let can_move = current_position != FINAL_POS;

        self.head_position = if(can_move) {
            current_position+1
        } else {
            current_position
        };
    }

    pub fn move_to(&mut self, pos: usize) {
        let is_valid_position = (pos <= FINAL_POS) && (pos >= INITIAL_POS);

        self.head_position = if(is_valid_position) {
            pos
        } else {
            self.head_position
        }
    }

    pub fn get_current_value(&self) -> u8 {
        self.block[self.head_position]
    }

    pub fn set_cell_value(&mut self, value: u8) {
        self.block[self.head_position] = value;
    }

    fn get_cells_num_left(head_position: usize, cells_num: usize) -> usize {
        if(head_position < cells_num) {
            head_position
        }else {
            cells_num
        }
    }

    fn get_cells_num_right(head_position: usize, cells_num: usize) -> usize {
        if(head_position + cells_num > FINAL_POS) {
            FINAL_POS - head_position
        }else {
            cells_num
        }
    }

    fn fill_segment(&self, segment: &mut [u8], mut startPos: usize) {
        let cells_in_segment = CELLS_IN_SNIPLET;

        for i in 0..cells_in_segment {
            segment[i] = self.block[startPos];
            startPos += 1;
        } 

    }

    fn count_digits(&self, num: usize) -> usize {
        if num == 0 {
            return 1;
        }

        let mut num_mut = num;
        let mut cnt = 0;

        while num_mut > 0 {
            num_mut /= 10;

            cnt += 1;
        }

        return cnt;
    }

    pub fn print_tape_sniplet(&self){
        let mut cells_used = CELLS_IN_SNIPLET - 1; // count the current cell

        let mut segment = [0; CELLS_IN_SNIPLET];

        let mut right_cells = MemoryTape::get_cells_num_right(self.head_position, CELLS_IN_SNIPLET/2);
        let mut left_cells = MemoryTape::get_cells_num_left(self.head_position, CELLS_IN_SNIPLET/2);

        // when there are less cells on the right or left
        if(right_cells < CELLS_IN_SNIPLET/2) {
            left_cells = CELLS_IN_SNIPLET - 1 - right_cells;
        } else if(left_cells < CELLS_IN_SNIPLET/2) {
            right_cells = CELLS_IN_SNIPLET - 1 - left_cells;
        }

        let mut current_index = self.head_position - left_cells;
        self.fill_segment(&mut segment, current_index);
        
        // build strings
        let mut cells_str = String::new();
        let mut indexes_str = String::new();
        let mut head_str = String::new();

        for cell in segment {
            // calculate spaces
            let cell_digits = self.count_digits(cell as usize);
            let index_digits = self.count_digits(current_index as usize);

            let cell_len = if cell_digits > index_digits {
                cell_digits
            }else {
                index_digits
            };

            let fill_blanks = |spaces: usize, string: &mut String| {
                for _i in 0..spaces {
                    *string += " ";
                }
            };

            //build cell string
            {
                let mut cell_str = String::from("[");
                cell_str += &cell.to_string();

                let spaces_required = cell_len - cell_digits;
                fill_blanks(spaces_required, &mut cell_str);
                
                cell_str += "]"; 

                cells_str += &cell_str;
            }

            //build index string
            {
                let mut index_str = String::from("[");
                index_str += &current_index.to_string();

                let spaces_required = cell_len - index_digits;
                fill_blanks(spaces_required, &mut index_str);
                
                index_str += "]";
                
                indexes_str += &index_str;
            }

            //build head string
            {
                if current_index == self.head_position {
                    // place head
                    let mid_pos = (cell_len+2)/2;
                    
                    for pos in 0..cell_len+2 {
                        head_str += if pos != mid_pos {
                            "_"
                        } else {
                            "^"
                        };
                    }
                } else {
                    for _pos in 0..cell_len+2 {
                        head_str += "_";
                    }
                }
            }

            current_index += 1;
        }

        println!("{}", cells_str);
        println!("{}", indexes_str);
        println!("{}", head_str);
    }
}

#[cfg(test)]
mod basic_operations_tests {
    use crate::MemoryTape;

    #[test]
    fn basic_increment() {
        let mut tape = MemoryTape::new();
        tape.set_cell_value(0);
        tape.increment();

        assert_eq!(tape.get_current_value(), 1);
    }

    #[test]
    fn overflow_increment() {
        let mut tape = MemoryTape::new();
        tape.set_cell_value(255);
        tape.increment();

        assert_eq!(tape.get_current_value(), 0);
    }

    #[test]
    fn basic_decrement() {
        let mut tape = MemoryTape::new();
        tape.set_cell_value(5);
        tape.decrement();

        assert_eq!(tape.get_current_value(), 4);
    }

    #[test]
    fn overflow_decrement() {
        let mut tape = MemoryTape::new();
        tape.set_cell_value(0);
        tape.decrement();

        assert_eq!(tape.get_current_value(), 255);
    }

    #[test]
    fn basic_move_left_and_right() {
        let mut tape = MemoryTape::new();

        tape.move_right();
        assert_eq!(tape.head_position, 1);

        tape.move_left();
        assert_eq!(tape.head_position, 0);
    }

    #[test]
    fn out_of_range_move_left_and_right() {
        let mut tape = MemoryTape::new();

        tape.move_left();
        assert_eq!(tape.head_position, 0);

        tape.move_to(29_999);
        tape.move_right();
        assert_eq!(tape.head_position, 29_999);
    }
}