/// A representation of the memory of a Universal Machine. Segment 0 (`seg0`) contains
/// the current running program, while `active_segs` contains any other segments
pub struct RumMemory {
    // pub seg0: Vec<u32>,
    // pub active_segs: HashMap<u32, Vec<u32>>,

    pub segs: Vec<Vec<u32>>,
    pub available_segs: Vec<u32>,
}

impl RumMemory {
    /// Function to initialize the memory of the machine. Only `seg0` is initialized with
    /// a value from the parameters.
    /// 
    /// Returns an instance of RumMemory with `seg0` equal to `program`
    /// 
    /// # Arguments:
    /// * `program`: a `Vec<u32>` containing the instructions of the program given by the command line.
    pub fn init(program: Vec<u32>) -> Self {
        //RumMemory { seg0: program, active_segs: HashMap::new() }
        RumMemory { segs: vec![program], available_segs: Vec::new() }
    }

    /// Function to get a value in a particular segment at an offset. This function will
    /// panic iff the segment has not been mapped or if the segment has been mapped
    /// but the offset points to a value that is out of bounds.
    /// 
    /// Returns a `u32` containing the value at $m`[segnum]` `[offset]`
    /// 
    /// # Arguments:
    /// * `segnum`: The segment being looked for, representing a key in `active_segs`.
    /// * `offset`: The index of the value being extracted.
    pub fn get_seg_val(&mut self, segnum: usize, offset: usize) -> u32 {
        //*self.get_seg(segnum).get(offset).unwrap()
        *self.segs.get(segnum).unwrap().get(offset).unwrap()
    }

    /// Function to store a value in a particular segment at an offset. This function
    /// will panic iff the segment has not been mapped or if the segment has been mapped
    /// but the offset points to a value that is out of bounds.
    /// 
    /// In all other cases, $m`[segnum]` `[offset]` = `value`
    /// 
    /// # Arguments:
    /// * `segnum`: The segment being looked for, representing a key in `active_segs`.
    /// * `offset`: The index where the value will be inserted.
    /// * `value`: The value being stored into a segment.
    pub fn store_seg_val(&mut self, segnum: usize, offset: usize, value: u32) {
        // let item = self.get_mut_seg(segnum).get_mut(offset).unwrap();

        let item = self.segs.get_mut(segnum).unwrap().get_mut(offset).unwrap();

        *item = value;
    }

    /// Function to map a segment inside of `active_segs`. This function will panic iff the program calls to map
    /// `seg0`, or if it attempts to map a segment that is already active.
    /// 
    /// In all other cases, a new segment $m`[segnum]` contains a Vec<u32> of length equal to `size`.
    /// 
    /// # Arguments:
    /// * `segnum`: The segment being mapped, representing a key that is not currently in `active_segs`.
    /// * `size`: The size of the segment that is being mapped.
    pub fn map_seg(&mut self, size: usize) -> usize {
        if self.available_segs.is_empty() {
            self.segs.push(vec![0; size]);
            self.segs.len() - 1
        } else {
            let next_seg = self.available_segs.pop().unwrap() as usize;
            self.segs[next_seg] = vec![0; size];
            next_seg
        }
    }


    /// Function to unmap a segment inside of `active_segs`. This function will panic iff the program
    /// calls to unmap `seg0` or if it attempts to unmap a segment that is not currently active.
    /// 
    /// In all other cases, $m`[segnum]` is removed, will all of its data being cleared.
    /// 
    /// # Arguments:
    /// * `segnum`: The segment being unmapped, representing a key in `active_segs`.
    pub fn unmap_seg(&mut self, segnum: usize) {
        if !self.segs[segnum].is_empty(){
            self.segs[segnum].clear();
            self.available_segs.push(segnum as u32);
            return;
        }
        panic!("segment is not empty")
    }

    /// Function to load a program from memory. This function will panic iff the segment being loaded
    /// is not currently active or if the offset points to a value out of bounds.
    /// 
    /// This program does not automatically set the program counter equal to `offset`. This functionality
    /// is handled by the caller once this function finishes executing.
    /// 
    /// In all other cases, $m`[0]` = $m`[segnum]`. If segnum = 0, nothing about `seg0` changes after execution.
    /// 
    /// # Arguments:
    /// * `segnum`: The segment being loaded into `seg0`, representing a key in `active_segs`.
    /// * `offset`: The offset that the program counter will be set to. This is used only to compare the new length of `seg0`
    pub fn load_program(&mut self, segnum: usize, offset: usize) {
        if self.segs[segnum].is_empty() {
            panic!("Attempted to load $m[{segnum}], which is not active");
        }

        if segnum != 0 {
            self.segs[0] = self.segs[segnum].clone();
        }

        if offset >= self.segs[0].len() {
            panic!("Illegal attempt to access memory at $m[{segnum}][{offset}]");
        }
    }
}