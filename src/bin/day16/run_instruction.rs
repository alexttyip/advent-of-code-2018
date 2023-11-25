pub mod day16 {
    pub type Int = usize;

    #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
    pub enum Opcode {
        Addr,
        Addi,
        Mulr,
        Muli,
        Banr,
        Bani,
        Borr,
        Bori,
        Setr,
        Seti,
        Gtir,
        Gtri,
        Gtrr,
        Eqir,
        Eqri,
        Eqrr,
    }

    pub const OPCODES: [Opcode; 16] = [
        Opcode::Addr,
        Opcode::Addi,
        Opcode::Mulr,
        Opcode::Muli,
        Opcode::Banr,
        Opcode::Bani,
        Opcode::Borr,
        Opcode::Bori,
        Opcode::Setr,
        Opcode::Seti,
        Opcode::Gtir,
        Opcode::Gtri,
        Opcode::Gtrr,
        Opcode::Eqir,
        Opcode::Eqri,
        Opcode::Eqrr,
    ];

    pub fn get_values_for_register_mode(
        register: [Int; 4],
        instruction: [Int; 4],
    ) -> (Int, Int, Int) {
        let ar = instruction[1];
        let br = instruction[2];
        let cr = instruction[3];

        let a = register[ar];
        let b = register[br];

        (a, b, cr)
    }

    pub fn get_values_for_immediate_mode(
        register: [Int; 4],
        instruction: [Int; 4],
    ) -> (Int, Int, Int) {
        let ar = instruction[1];
        let cr = instruction[3];

        let a = register[ar];
        let b = instruction[2];

        (a, b, cr)
    }

    pub fn run_instruction(
        mut register: [Int; 4],
        opcode: &Opcode,
        instruction: [Int; 4],
    ) -> [Int; 4] {
        match opcode {
            Opcode::Addr => {
                let (a, b, cr) = get_values_for_register_mode(register, instruction);

                register[cr] = a + b;
            }
            Opcode::Addi => {
                let (a, b, cr) = get_values_for_immediate_mode(register, instruction);

                register[cr] = a + b;
            }

            Opcode::Mulr => {
                let (a, b, cr) = get_values_for_register_mode(register, instruction);

                register[cr] = a * b;
            }
            Opcode::Muli => {
                let (a, b, cr) = get_values_for_immediate_mode(register, instruction);

                register[cr] = a * b;
            }

            Opcode::Banr => {
                let (a, b, cr) = get_values_for_register_mode(register, instruction);

                register[cr] = a & b;
            }
            Opcode::Bani => {
                let (a, b, cr) = get_values_for_immediate_mode(register, instruction);

                register[cr] = a & b;
            }

            Opcode::Borr => {
                let (a, b, cr) = get_values_for_register_mode(register, instruction);

                register[cr] = a | b;
            }
            Opcode::Bori => {
                let (a, b, cr) = get_values_for_immediate_mode(register, instruction);

                register[cr] = a | b;
            }

            Opcode::Setr => {
                let (a, _, cr) = get_values_for_register_mode(register, instruction);

                register[cr] = a;
            }
            Opcode::Seti => {
                let a = instruction[1];
                let cr = instruction[3];

                register[cr] = a;
            }

            Opcode::Gtir => {
                let a = instruction[1];
                let br = instruction[2];
                let cr = instruction[3];

                let b = register[br];

                register[cr] = Int::from(a > b);
            }
            Opcode::Gtri => {
                let ar = instruction[1];
                let b = instruction[2];
                let cr = instruction[3];

                let a = register[ar];

                register[cr] = Int::from(a > b);
            }
            Opcode::Gtrr => {
                let (a, b, cr) = get_values_for_register_mode(register, instruction);

                register[cr] = Int::from(a > b);
            }

            Opcode::Eqir => {
                let a = instruction[1];
                let br = instruction[2];
                let cr = instruction[3];

                let b = register[br];

                register[cr] = Int::from(a == b);
            }
            Opcode::Eqri => {
                let ar = instruction[1];
                let b = instruction[2];
                let cr = instruction[3];

                let a = register[ar];

                register[cr] = Int::from(a == b);
            }
            Opcode::Eqrr => {
                let (a, b, cr) = get_values_for_register_mode(register, instruction);

                register[cr] = Int::from(a == b);
            }
        }

        register
    }
}
