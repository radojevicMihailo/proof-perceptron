use expander_compiler::frontend::{extra::UnconstrainedAPI, *};

mod zkfloat;

struct NumberOfParmaters {
    pub num_x: usize,
    pub num_y: usize,
    pub num_w: usize,
}

const NUM_OF_PARAMS: NumberOfParmaters =  NumberOfParmaters {
    num_x: 3,
    num_y: 4,
    num_w: 16,
};

declare_circuit!(Circuit {
    x: [PublicVariable; NUM_OF_PARAMS.num_x],
    y: [PublicVariable; NUM_OF_PARAMS.num_y],
    w: [Variable; NUM_OF_PARAMS.num_w]
});

impl Define<M31Config> for Circuit<Variable> {
    fn define(&self, builder: &mut API<M31Config>) {
        for i in 0..100 {
            
        }
    }
}

fn main() {
    println!("Hello, world!");
}
