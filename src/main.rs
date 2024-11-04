use ndarray::{Array, Array2};
use rand::Rng;

fn initialize_weights() -> Array2<f64> {
    let mut rng = rand::thread_rng();
    Array::from_shape_fn((2, 1), |_| rng.gen::<f64>())
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn forward_propagation(inputs: &Array2<f64>, weights: &Array2<f64>) -> f64 {
    let dot_product = inputs.dot(weights);
    sigmoid(dot_product[[0, 0]])
}

fn decide_movement(sensor_input: f64, weights: &Array2<f64>) -> String {
    let inputs = Array::from_shape_vec((1, 2), vec![sensor_input, 1.0]).unwrap(); // Bias input
    let output = forward_propagation(&inputs, weights);

    if output > 0.5 {
        "Move Forward".to_string()
    } else {
        "Turn Left".to_string()
    }
}

fn main() {
    let weights = initialize_weights();
    
    // Simulate sensor input from the robot's environment (e.g., distance to an obstacle)
    let sensor_input = 0.3;
    
    let action = decide_movement(sensor_input, &weights);
    
    println!("Robot action: {}", action);
}

// A simplified robot structure simulating movement control
struct Robot {
    position: (f64, f64), // x, y position
    direction: f64,       // angle in radians
}

impl Robot {
    fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            direction: 0.0,
        }
    }

    fn move_forward(&mut self, distance: f64) {
        self.position.0 += distance * self.direction.cos();
        self.position.1 += distance * self.direction.sin();
    }

    fn turn_left(&mut self, angle: f64) {
        self.direction += angle;
    }
}

fn main() {
    let mut robot = Robot::new();
    let sensor_input = 0.3; // Example sensor input
    let weights = initialize_weights();
    
    // Decide movement based on AI
    let action = decide_movement(sensor_input, &weights);
    
    if action == "Move Forward" {
        robot.move_forward(1.0);
    } else if action == "Turn Left" {
        robot.turn_left(0.785); // Turn 45 degrees
    }
    
    println!("Robot position: {:?}", robot.position);
}
