# Robot AI Movement :electron: 🤖

This project simulates a simple robot that decides how to move based on sensor input using a basic artificial intelligence model.

## How It Works

1. **Initialization**:
   - The robot starts with a random set of weights. These weights help the robot decide its actions based on sensor data.

2. **Sensor Input**:
   - The robot receives input from its environment, such as the distance to an obstacle. In this example, we use a fixed value of `0.3`.

3. **Forward Propagation**:
   - The sensor input and a bias value (always `1.0`) are combined into a matrix. The robot uses the random weights to perform a calculation called "forward propagation" to determine a score.

4. **Decision Making**:
   - The score is passed through a function called sigmoid, which transforms it into a value between `0` and `1`.
   - If this value is greater than `0.5`, the robot decides to **move forward**. Otherwise, it decides to **turn left**.

5. **Robot Movement**:
   - Based on the decision, the robot either moves forward a set distance or turns left by a specified angle.

6. **Output**:
   - Finally, the program prints the robot's action and its new position after moving.

## Example

When you run the program:
- It initializes the weights.
- The robot checks its sensor input.
- It decides whether to move forward or turn left.
- It updates its position accordingly.

This simple AI model demonstrates how a robot can make decisions based on its environment!

----
