use std::collections::HashMap;

pub struct RumMemory {
    seg0: Vec<u32>,
    active_segs: HashMap<u32, Vec<u32>>,
}

impl RumMemory {
    pub fn init(program: Vec<u32>) -> Self {
        RumMemory { seg0: program, active_segs: HashMap::new() }
    }

    pub fn get_seg_val(&mut self, segnum: u32, offset: usize) -> u32 {
        *self.get_seg(segnum).get(offset).unwrap()
    }

    pub fn store_seg_val(&mut self, segnum: u32, offset: usize, value: u32) {
        let item = self.get_mut_seg(segnum).get_mut(offset).unwrap();

        *item = value;
    }

    pub fn map(&mut self, segnum: u32, size: usize) {
        if segnum == 0 {
            panic!("Attempted to map $m[0], which already exists");
        }

        if !self.active_segs.contains_key(&segnum) {
            self.active_segs.insert(segnum, vec![0; size]);
        } else {
            panic!("Attempted to map $m[{segnum}], which already exists");
        }
    }

    pub fn unmap(&mut self, segnum: u32) {
        if segnum == 0 {
            panic!("Attempted to unmap $m[0], which is illegal");
        }

        if self.active_segs.contains_key(&segnum) {
            self.active_segs.remove(&segnum);
        } else {
            panic!("Attempted to map $m[{segnum}], which already exists");
        }
    }

    pub fn load_program(&mut self, segnum: u32, offset: usize) {
        if segnum != 0 {
            self.seg0 = self.get_seg(segnum).clone();
        }

        if offset >= self.seg0.len() {
            panic!("Illegal access at $m[0][{offset}]");
        }
    }

    fn get_seg(&self, segnum: u32) -> &Vec<u32> {
        if segnum == 0 {
            &self.seg0
        } else {
            self.active_segs.get(&segnum).unwrap()
        }
    }

    fn get_mut_seg(&mut self, segnum: u32) -> &mut Vec<u32> {
        if segnum == 0 {
            &mut self.seg0
        } else {
            self.active_segs.get_mut(&segnum).unwrap()
        }
    }
}