pub mod rumcpu;
pub mod rumdata;
pub mod ruminfoextr;
pub mod rumload;
pub mod rummemory;

#[cfg(test)]
mod info_test{
    use crate::ruminfoextr::{self, get_one_reg};
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

mod cpu_test{
    use crate::rumcpu::{self, RumCpu};
    #[test]
    fn test_add(){
        let mut CPU:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 0, 0, 234567]
        };
        CPU.add(6, 0, 1);
        assert_eq!(CPU.fetch_reg(6), 340);
    }

    #[test]
    fn test_add_wrap(){
        let mut CPU:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 0, 0, 234567]
        };
        CPU.add(6, 2, 3);
        assert_eq!(CPU.fetch_reg(6), 3);
    }

    #[test]
    fn test_mul(){
        let mut CPU:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 0, 0, 234567]
        };
        CPU.mul(6, 0, 1);
        assert_eq!(CPU.fetch_reg(6), 22011);
    }

    #[test]
    fn test_mul_wrap(){
        let mut CPU:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 0, 0, 234567]
        };
        CPU.mul(6, 2, 3);
        assert_eq!(CPU.fetch_reg(6), 4294967292);
    }

    #[test]
    fn test_div(){
        let mut CPU:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        CPU.div(6, 5, 2);
        assert_eq!(CPU.fetch_reg(6), 3);
    }

    #[test]
    fn test_div_round(){
        let mut CPU:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        CPU.div(6, 1, 2);
        assert_eq!(CPU.fetch_reg(6), 21);
    }

    #[test]
    #[should_panic]
    fn test_div_zero(){
        let mut CPU:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        CPU.div(1, 2, 6);
    }

    #[test]
    fn test_div_of_zero(){
        let mut CPU:RumCpu = RumCpu{
            pc: 0,
            regs: [253, 87, 4, u32::MAX, 6969696, 12, 0, 234567]
        };
        CPU.div(1, 6, 2);
        assert_eq!(CPU.fetch_reg(1), 0);
    }

    
}