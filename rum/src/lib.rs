pub mod rumcpu;
pub mod rumdata;
pub mod ruminfoextr;
pub mod rumload;
pub mod rummemory;

#[cfg(test)]
mod info_test{
    use crate::ruminfoextr;
    #[test]
    fn test_one_reg(){
        assert_eq!(ruminfoextr::get_one_reg(4), 4);
    }

    #[test]
    fn test_two_reg(){
        assert_eq!(ruminfoextr::get_two_regs(41), (5, 1));
    }

    #[test]
    fn test_three_reg(){
        assert_eq!(ruminfoextr::get_three_regs(489), (7, 5, 1));
    }

    #[test]
    fn test_opcode(){
        assert_eq!(ruminfoextr::get_opcode(7 << 28), 7);
    }

    #[test]
    fn test_load_reg(){
        assert_eq!(ruminfoextr::get_load_reg(8 << 25), 0);
    }

    #[test]
    fn test_get_val(){
        assert_eq!(ruminfoextr::get_value(2), 2);
    }
}

#[cfg(test)]
mod cpu_test{
    use crate::rumcpu::RumCpu;
    #[test]
    fn test_cmov_zero(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 0, 0, 234567]
        };
        cpu.cmov(1, 2, 6);
        assert_eq!(cpu.fetch_reg(1), 87);
    }

    #[test]
    fn test_cmov_non_zero(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 0, 0, 234567]
        };
        cpu.cmov(1, 2, 0);
        assert_eq!(cpu.fetch_reg(1), 4);
    }


    #[test]
    fn test_add(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 0, 0, 234567]
        };
        cpu.add(6, 0, 1);
        assert_eq!(cpu.fetch_reg(6), 340);
    }

    #[test]
    fn test_add_wrap(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 0, 0, 234567]
        };
        cpu.add(6, 2, 3);
        assert_eq!(cpu.fetch_reg(6), 3);
    }

    #[test]
    fn test_mul(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 0, 0, 234567]
        };
        cpu.mul(6, 0, 1);
        assert_eq!(cpu.fetch_reg(6), 22011);
    }

    #[test]
    fn test_mul_wrap(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 0, 0, 234567]
        };
        cpu.mul(6, 2, 3);
        assert_eq!(cpu.fetch_reg(6), 4294967292);
    }

    #[test]
    fn test_div(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        cpu.div(6, 5, 2);
        assert_eq!(cpu.fetch_reg(6), 3);
    }

    #[test]
    fn test_div_round(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        cpu.div(6, 1, 2);
        assert_eq!(cpu.fetch_reg(6), 21);
    }

    #[test]
    #[should_panic]
    fn test_div_zero(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        cpu.div(1, 2, 6);
    }

    #[test]
    fn test_div_of_zero(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        cpu.div(1, 6, 2);
        assert_eq!(cpu.fetch_reg(1), 0);
    }

    #[test]
    fn test_nand_self(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        cpu.nand(6, 1, 1);
        assert_eq!(cpu.fetch_reg(6), !87);
    }

    #[test]
    fn test_nand_max(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        cpu.nand(6, 3, 5);
        assert_eq!(cpu.fetch_reg(6), !12);
    }

    #[test]
    fn test_nand_zero(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        cpu.nand(2, 6, 0);
        assert_eq!(cpu.fetch_reg(2), u32::MAX);
    }
    #[test]
    fn test_nand(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        cpu.nand(6, 1, 0);
        assert_eq!(cpu.fetch_reg(6), (86 * -1) as u32);
    }

    #[test]
    fn test_load_value(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        cpu.load_value(6, 255);
        assert_eq!(cpu.fetch_reg(6), 255);
    }
}

#[cfg(test)]
mod output_test{
    use crate::rumdata::RumData;
    use crate::rumcpu::RumCpu;
    use crate::rummemory::RumMemory;

    #[test]
    #[should_panic]
    fn test_output_panic(){
        let cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        let rumdata:RumData = RumData{
            cpu: cpu,
            memory: RumMemory::init(vec![0])
        };
        rumdata.output(7);
    }

}

#[cfg(test)]
mod segments_test {
    use std::collections::HashMap;

    use crate::rummemory::RumMemory;

    impl RumMemory {
        pub fn custom_init(segments: HashMap<u32, Vec<u32>>, seg0: Vec<u32>) -> Self {
            RumMemory {seg0, active_segs: segments}
        }
    }

    #[test]
    fn test_get_val(){
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );

        assert_eq!(example.get_seg_val(0, 3), 100);
        assert_eq!(example.get_seg_val(1, 0), 69);
        assert_eq!(example.get_seg_val(2, 1), 32);
    }

    #[test]
    #[should_panic]
    fn seg_val_oob() {
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );
        example.get_seg_val(0, 4);
    }

    #[test]
    #[should_panic]
    fn seg_val_missing() {
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );
        example.get_seg_val(4, 0);
    }

    #[test]
    fn test_store_val(){
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );

        example.store_seg_val(0, 0, 1);
        example.store_seg_val(1, 2, 2);
        example.store_seg_val(2, 3, 3);

        assert_eq!(example.get_seg_val(0, 0), 1);
        assert_eq!(example.get_seg_val(1, 2), 2);
        assert_eq!(example.get_seg_val(2, 3), 3);
    }

    #[test]
    #[should_panic]
    fn store_val_oob() {
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );
        example.store_seg_val(0, 4, 69);
    }

    #[test]
    #[should_panic]
    fn store_val_missing() {
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );
        example.store_seg_val(4, 0, 8);
    }

    #[test]
    fn test_map_seg(){
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );

        example.map_seg(3, 4);
        example.map_seg(68, 90000);
        example.map_seg(777777, 0);

        let first = example.active_segs.get(&3).unwrap();
        let second = example.active_segs.get(&68).unwrap();
        let third = example.active_segs.get(&777777).unwrap();

        assert!(example.active_segs.contains_key(&3) && first.len() == 4);
        assert!(example.active_segs.contains_key(&68) && second.len() == 90000);
        assert!(example.active_segs.contains_key(&777777) && third.len() == 0);   
    }

    #[test]
    #[should_panic]
    fn cant_map_seg0() {
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );

        example.map_seg(0, 78);
    }

    #[test]
    #[should_panic]
    fn cant_remap_seg() {
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );

        example.map_seg(1, 5);
    }

    #[test]
    fn test_unmap_seg(){
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );

        example.unmap_seg(1);

        assert!(!example.active_segs.contains_key(&1));
    }

    #[test]
    #[should_panic]
    fn cant_unmap_seg0() {
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );

        example.unmap_seg(0);
    }

    #[test]
    #[should_panic]
    fn cant_unmap_missing_seg() {
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );

        example.unmap_seg(5);
    }

    #[test]
    fn test_jump(){
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );

        example.load_program(0, 3);

        assert_eq!(example.seg0, vec![0,0,0,100]);
    }

    #[test]
    fn test_load_program() {
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );

        example.load_program(1, 0);

        assert_eq!(example.seg0, vec![69, 0, 0, 42]);
    }

    #[test]
    #[should_panic]
    fn cant_load_oob() {
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );

        example.load_program(2, 9);
    }

    #[test]
    #[should_panic]
    fn cant_load_missing_segment() {
        let mut example = RumMemory::custom_init(
            HashMap::from([(1, vec![69, 0, 0, 42]),(2, vec![75, 32, 0, 0])]),
            vec![0,0,0,100]
        );

        example.load_program(9, 0);
    }
}