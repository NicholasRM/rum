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
    use std::collections::HashMap;

    use crate::rumdata::RumData;
    use crate::rumcpu::RumCpu;
    use crate::rummemory::RumMemory;

    #[test]
    #[should_panic]
    fn test_output_panic(){
        let mut cpu:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        let mut rumdata:RumData = RumData{
            cpu: cpu,
            memory: RumMemory::init(vec![0])
        };
        rumdata.output(7);
    }

}