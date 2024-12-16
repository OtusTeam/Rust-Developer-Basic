// Ссылки и заимствования

struct Registers {
    reg1 u8
    reg2 u8
    reg3 u8
}

struct irq_block {}

struct mode_block {}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        let registers = Registers{
            reg1: 0,
            reg2: 0,
            reg3: 0,
        };

        let irq_registers = registers.get_irq_block();
        irq_registers.irq1 = 1;
        irq_registers.irq2 = 2;

        let mod_block = registers.get_mode_block();
        mod_block.mode = 3;

        assert_eq!(registers, Registers{
            reg1: 1,
            reg2: 3,
            reg3: 2,
        });
    }
}
