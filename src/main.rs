use expander_compiler::frontend::*;
use std::{env, fs, io};
use extra::Serde;

mod zkfloat;

struct NumberOfParmaters {
    pub num_x: usize,
    pub num_y: usize,
    pub num_w: usize,
    pub num_b: usize,
}

const NUM_OF_PARAMS: NumberOfParmaters =  NumberOfParmaters {
    num_x: 4,
    num_y: 3,
    // (4*2) + (2*3) + (3*3) + (3*3) = 8 + 6 + 9 + 9 = 32
    num_w: 32,
    // 2 + 3 + 3 + 3 = 11
    num_b: 11,
};

#[allow(dead_code)]
fn read_file_to_vec(filename: &str) -> io::Result<Vec<i32>> {
    let content = fs::read_to_string(filename)?;
    let values: Vec<i32> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(values)
}

declare_circuit!(Circuit {
    x: [PublicVariable; NUM_OF_PARAMS.num_x],
    y: [PublicVariable; NUM_OF_PARAMS.num_y],
    w: [Variable; NUM_OF_PARAMS.num_w],
    b: [Variable; NUM_OF_PARAMS.num_b],
});

impl Define<M31Config> for Circuit<Variable> {
    fn define(&self, builder: &mut API<M31Config>) {
        let layer_sizes = [4, 2, 3, 3, 3];
        let mut activations = self.x.to_vec();
        let mut weight_index = 0;
        let mut bias_index = 0;

        for layer_idx in 0..layer_sizes.len() - 1 {
            let current_size = layer_sizes[layer_idx];
            let next_size = layer_sizes[layer_idx + 1];
            let mut next_activations: Vec<Variable> = vec![builder.constant(0); next_size];

            for i in 0..next_size {
                let mut sum: Vec<Variable> = vec![builder.constant(0)];
                for j in 0..current_size {

                    if weight_index >= self.w.len() {
                        eprintln!("Error: Not enough weights provided");
                        std::process::exit(1);
                    }
                    // Placeholder: Assumes Variable supports multiplication and addition
                    let prod = builder.mul(activations[j], self.w[weight_index]);
                    sum.push(builder.add(sum[j], prod));
                    weight_index += 1;
                }
                if bias_index >= self.b.len() {
                    eprintln!("Error: Not enough biases provided");
                    std::process::exit(1);
                }
                sum.push(builder.add(sum[sum.len() - 1], self.b[bias_index]));
                bias_index += 1;
                next_activations[i] = sum[sum.len() - 1];
            }
            activations = next_activations;
            println!("Layer {} Output: {:?}", layer_idx + 1, activations);
        }

        println!("Final Output: {:?}", activations);

        builder.assert_is_equal(activations[0], self.y[0]);
        builder.assert_is_equal(activations[1], self.y[1]);
        builder.assert_is_equal(activations[2], self.y[2]);
    }
}

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there are enough arguments (at least one hidden layer)
    if args.len() < 2 {
        eprintln!("Usage: {} <hidden_layer_sizes...>", args[0]);
        eprintln!("Example: {} 4 8 4", args[0]);
        std::process::exit(1);
    }

    // Parse hidden layer sizes from arguments (skip the program name at args[0])
    let mut hidden_layers: Vec<usize> = Vec::new();
    for (i, arg) in args.iter().enumerate().skip(1) {
        match arg.parse::<usize>() {
            Ok(size) => {
                if size == 0 {
                    eprintln!("Error: Layer size at position {} must be greater than 0", i);
                    std::process::exit(1);
                }
                hidden_layers.push(size);
            }
            Err(_) => {
                eprintln!("Error: '{}' at position {} is not a valid integer", arg, i);
                std::process::exit(1);
            }
        }
    }

    // Print the parsed hidden layer configuration
    println!("Neural Network Hidden Layers: {:?}", hidden_layers);

    let compile_result = compile(&Circuit::default()).unwrap();
    let assignment = Circuit::<M31> {
        x: [M31::from(1), M31::from(2), M31::from(3), M31::from(4)],
        y: [M31::from(2405), M31::from(2405), M31::from(2407)],
        w: [M31::from(1), M31::from(2), M31::from(1), M31::from(3), M31::from(2), M31::from(1), M31::from(3), M31::from(2),
            M31::from(1), M31::from(2), M31::from(3), M31::from(1), M31::from(2), M31::from(1),
            M31::from(2), M31::from(1), M31::from(3), M31::from(2), M31::from(1), M31::from(3), M31::from(2), M31::from(1), M31::from(3),
            M31::from(1), M31::from(2), M31::from(3), M31::from(2), M31::from(1), M31::from(3), M31::from(1), M31::from(2), M31::from(3)],
        b: [M31::from(0), M31::from(1),
            M31::from(1), M31::from(2), M31::from(0),
            M31::from(1), M31::from(2), M31::from(0),
            M31::from(0), M31::from(1), M31::from(2)],    
    };
    let assignments = vec![assignment.clone(); 16];
    let witness = compile_result
        .witness_solver
        .solve_witnesses(&assignments)
        .unwrap();
    let output = compile_result.layered_circuit.run(&witness);
    assert_eq!(output, vec![true; 16]);

    let file = std::fs::File::create("circuit.txt").unwrap();
    let writer = std::io::BufWriter::new(file);
    compile_result
        .layered_circuit
        .serialize_into(writer)
        .unwrap();

    let file = std::fs::File::create("witness.txt").unwrap();
    let writer = std::io::BufWriter::new(file);
        witness.serialize_into(writer).unwrap();

    let file = std::fs::File::create("witness_solver.txt").unwrap();
    let writer = std::io::BufWriter::new(file);
    compile_result
        .witness_solver
        .serialize_into(writer)
        .unwrap();
}
