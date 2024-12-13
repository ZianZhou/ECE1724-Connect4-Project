# Rusty Connect Four

**Source Code:**

Github: https://github.com/ZianZhou/ECE1724-Connect4-Project

## Team Information

- Mingtao Wang  
  Student Number: 1011777579  
  Preferred Email: mingtao.wang@mail.utoronto.ca

- Ziang Jia  
  Student Number: 1005704962  
  Preferred Email: ziang.jia@mail.utoronto.ca

- Zian Zhou  
  Student Number: 1005910454  
  Preferred Email: zn.zhou@mail.utoronto.ca

## Video Demo

[https://youtu.be/znnD6BMbQsk](https://youtu.be/znnD6BMbQsk)

## Motivation

Connect Four is a classic strategy game beloved across generations, known for its simple yet fun gameplay. The team aims to move Connect Four from the real world onto computer screens where it can thrive virtually as a 2D game. The game will be a 2D version of Connect Four called Rusty Connect Four with an exciting twist that can make it even more engaging. In the original version of Connect Four, ties can often occur in a competitive game between two strong players. In this version, a tie will dynamically expand the board dimensions, allowing the game to continue until a winner is determined. Furthermore, there will be power-up items on the board that provide players with special abilities, giving them an edge to win the game. For example, these abilities can be skipping your opponent’s turn or putting obstacles in a specific spot. These items appear randomly on the board and are consumed by the player when the spot in which they occupy is taken. These power-ups will add strategic depth, making each new game different and unpredictable. By creating a familiar yet enhanced version of Connect Four, we hope to make a game that is both entertaining and educational.

The motivation for building the Connect Four game in Rust stems from our goal of bridging a gap in the current Rust ecosystem where game development is complex and difficult for new game developers. Through this game, the team strives to integrate simple and beginner-friendly game development techniques that can still be performant at the same time. The game will provide insight to newcomers and educate them to effectively learn essential concepts like entity-component-systems patterns, scene management, and 2D rendering. This approach of using simple tools to create enhanced features and interesting game mechanics will hopefully inspire more ways to take advantage of Rust’s performant nature.

## Objectives

This project aims to create an accessible and well-documented template for beginner game developers who are new to Rust. This way, they can build more 2D over-the-board games like Gomoku, Go, and Chess easily and efficiently. By implementing a simple game like Connect Four with creative twists, the team can demonstrate a focused example of creative game development in Rust. This includes using Bevy as the foundation for the game engine and 2D rendering. The goal is for this project to act as a reusable example with both foundational and advanced Rust game development concepts, to enable the Rust community to make more creative games in the long run.

Rusty Connect Four aims to create an engaging, educational version of the classic Connect Four game, designed to address challenges within the Rust ecosystem's game development landscape. This project aims to fill a gap by providing an instructive, beginner-friendly template for 2D games, showcasing Rust’s capabilities and performance through enhanced features and coding techniques and crates. Our game will serve as an educational example for beginner game developers and assist them in expanding this approach to other board games.

## Features

**Key Features:**

**Dynamic Board Expansion:**  
After a long time of playing, it is possible to end up with a draw, which can impact the game experience. To mitigate the common issue of ties in Connect Four, our game introduces an innovative mechanism: tie prevention. When a tie occurs, the board dynamically expands, enabling the game to continue until a winner is determined. This feature adds an unpredictable twist that enhances the game’s playability. Players also have the flexibility of choosing whether to enable this feature or not.

**Power-Ups:**  
Randomly appearing power-up items on the board enhance the playability of the game, such as skipping the opponent’s turn, placing an obstacle, or exploding the opponent’s piece. Players can acquire these items by placing their pieces on the designated spot. This additional feature makes each game session unique and adds depth to the gameplay.

**Beginner-Friendly Game Development Concepts:**  
Rusty Connect Four is an accessible entry point for beginner Rust developers. Using Bevy for the game engine, we will highlight concepts such as entity-component-system patterns, scene management, and 2D rendering. These essential techniques in Rust game development will be demonstrated within the codebase, providing educational value to new developers.

**Template for Future Games:**  
By developing Connect Four with enhanced mechanics and add-on features, we aim to create a template that can be extended to other games like Reversi, Go, and Chess. This template will demonstrate best practices in Rust game development, making it easier for developers to create performant and creative 2D board games. The scalability of this project could facilitate new board game development and bridge the gap of lacking 2D board game templates in the Rust ecosystem.

## User’s Guide

**Starting the Game:**  
Compile the project by running `cargo build --release`.  
Launch the game by running `cargo run --release`.

**Main Menu:**  
After launching, you will see the main menu. From here, you can:

- Start a New Game by clicking the "Start" button.
- Toggle Power-Ups before the game begins, players have the option to enable power-ups by toggling the “Power-ups” button.

**Initial Board:**  
By default, you’ll start on a 6x7 Connect Four board.

**Gameplay Controls**  
**Dropping Pieces:**  
Press the number keys (1 through 7) on your keyboard to drop a piece into the corresponding column on the board. For example, pressing "1" drops your piece into the first column.

**Turn-Based Play:**  
Players alternate turns after each piece is dropped. The current player’s turn indicator is displayed at the top.

**Board Expansion:**  
If the board becomes filled and no winner is detected, the board automatically expands to a larger size (up to 10x10). This feature prevents ties, ensuring a definitive outcome. After expansion, players continue playing on the new, larger board using the same controls with keys 8 to 0 enabled as well.

**Power-Ups (If Enabled):**  
If power-ups are enabled on the main menu, 6 power-ups will be generated at random locations on the initial board. During board expansion, some power-ups are randomly generated as well:

- **Purple B (Bomb):**  
  Dropping a piece onto a cell with a 'B' power-up triggers a bomb effect. A bomb explodes the piece directly below the bomb power-up and you can continue to choose a spot to place your piece after bomb detonation. This is like blowing a hole in that column. This can create an opportunity to win or disrupt the opponent’s strategy.

  ![Bomb Power-up](assets/icons/Bomb.png "Bomb")

- **Green S (Skip):**  
  Landing on an 'S' power-up forces the opponent to skip their next turn entirely. By doing this, you gain a significant advantage, as your opponent loses a chance to counter your moves or place pieces on their turn.

  ![Skip Turn Power-up](assets/icons/Skip.png "Skip Turn")

- **Grey H (Obstacle):**  
  Placing a piece on an 'H' power-up creates obstacles to the right and left of it. If there’s nothing underneath the obstacle, obstacles drop to the bottom. Obstacles block future pieces from taking up certain cells, altering the structure of the board. Furthermore, obstacles can override existing pieces or power-ups if there’s already something there. Players must adapt their strategies around these immovable barriers.

  ![Obstacle Power-up](assets/icons/Obstacles.png "Obstacle")

**Win Condition**

- **Winning the Game:**  
  To win, you must arrange four of your pieces consecutively—horizontally, vertically, or diagonally. The game immediately ends when a four-in-a-row is detected.

- **Continued Play after Expansion:**  
  In traditional Connect Four games, there are ties frequently. In Rusty Connect Four, if no winner emerges and the board is filled, it expands. Play continues until there are four in a row.

**Ending the Game**

- **Win Message:**  
  Once a player achieves four in a row, the game goes to the “Game Over” screen and displays a victory message for the winning player. It also shows the final state of the board.

- **Exiting:**  
  You can press “continue” to go back to the starting menu. From the main menu, you can start a new game or re-toggle the power-ups setting as desired.

## Reproducibility Guide

**Prerequisites:**  
Below are the instructions suitable for a Ubuntu Linux server and a macOS Sonoma laptop computer. The following are required:

- Rust stable toolchain (Install from [https://rustup.rs/](https://rustup.rs/))
- Cargo (comes with Rust installation)
- For macOS: Ensure that you have the latest Command Line Tools installed.

**Steps to Build and Run:**

```bash
git clone git@github.com:ZianZhou/ECE1724-Connect4-Project.git
cd ECE1724-Connect4-Project
cargo build --release
cargo run --release
```

**Interacting With the Game**

- Follow the on-screen instructions to enable or disable power-ups.
- Use the number keys to drop pieces and play.
- No additional configuration or environment variables should be required.

**Note:**

- If you encounter any issues, ensure you are running a recent version of Rust.
- Verify that all dependencies are properly installed.
- The build time may take a few minutes, that is completely normal.

## Feature Testing

To ensure the stability, functionality, and user experience of Rusty Connect Four, the following test cases and procedures are designed for key features:

### Gameplay Mechanics

**Piece Placement**

- **Scenario:** Players drop pieces into columns.
- **Steps:**  
  Press number keys (1-7) to place pieces into corresponding columns.
- **Expected Outcome:**  
  Pieces stack correctly in the selected columns.  
  Invalid moves (such as dropping into a full column) are prevented with proper feedback.

**Turn-Based Play**

- **Scenario:** Players alternate turns.
- **Steps:**  
  Ensure the current player’s turn indicator updates accurately.  
  Verify that each player can only take their turn when it’s their turn.
- **Expected Outcome:**  
  Turns alternate properly between players.

**Win Condition**

- **Scenario:** A player achieves four consecutive pieces (horizontally, vertically, or diagonally).
- **Steps:**  
  Play until a four-in-a-row is formed.
- **Expected Outcome:**  
  The game detects the winning condition and displays the victory message immediately.

**Board Reset After Game Over**

- **Scenario:** The game ends and a new game is started.
- **Steps:**  
  End the game and navigate to the main menu.  
  Start a new game.
- **Expected Outcome:**  
  The board resets to its initial size (6x7).

### Dynamic Board Expansion

- **Scenario:** The board is filled without a winner.
- **Steps:**  
  Play a game where no player achieves a four-in-a-row on the initial board.  
  Confirm that the board expands dynamically to the next size.
- **Expected Outcome:**  
  The board expands while maintaining the current state of pieces and allowing continued gameplay.  
  **Edge Case:** Ensure the board does not exceed the maximum size (10x10).  
  **Failure Handling:** Verify that any errors (e.g., invalid board state) are handled without crashing.

### Power-Up Functionality

**Purple Piece B (Bomb)**

- **Scenario:** A player drops a piece on a bomb power-up.
- **Steps:**  
  Enable power-ups and start the game.  
  Drop a piece onto a cell with the 'B' power-up.
- **Expected Outcome:**  
  The bomb explodes, clearing the piece directly below it.  
  The player can continue their turn by placing a piece in a new spot.

**Green Piece S (Skip)**

- **Scenario:** A player activates the skip power-up.
- **Steps:**  
  Drop a piece onto a cell with the 'S' power-up.
- **Expected Outcome:**  
  The opponent’s next turn is skipped.  
  The game properly alternates turns afterward.

**Grey Piece H (Obstacle)**

- **Scenario:** A player triggers the obstacle power-up.
- **Steps:**  
  Drop a piece onto a cell with the 'H' power-up.
- **Expected Outcome:**  
  Obstacles are generated to the left and right of the power-up.  
  If there’s no piece underneath, obstacles drop to the bottom of the board.

### Edge Cases

**Invalid Inputs**

- **Scenario:** Players press invalid keys or input outside the game's range.
- **Expected Outcome:**  
  The game ignores invalid inputs without any crashes or unintended behavior.

**Maximum Board Size**

- **Scenario:** The board reaches the maximum size (10x10).
- **Expected Outcome:**  
  No further expansions occur, and the game continues until a winner is determined.

**Full Column Input**

- **Scenario:** A player tries to drop a piece into a column that is already full.
- **Expected Outcome:**  
  The game prevents the move and provides feedback without crashing.

**Bomb at Bottom Row**

- **Scenario:** A player activates a Bomb power-up located at the bottom row of the board.
- **Expected Outcome:**  
  The bomb effect correctly clears the piece without causing an invalid board state.

**Tie on Expanded Board**

- **Scenario:** A tie occurs after the board has already expanded to its maximum size.
- **Expected Outcome:**  
  The game correctly identifies the tie and ends without errors.

**Consecutive Bombs**

- **Scenario:** Multiple Bomb power-ups are triggered consecutively.
- **Expected Outcome:**  
  Each bomb effect executes properly without overlapping or skipping actions.

**Obstacle Placement Over Pieces**

- **Scenario:** An Obstacle power-up is triggered, and the obstacle lands on an existing piece.
- **Expected Outcome:**  
  The obstacle replaces the piece and updates the board state correctly.

**Skip Turn on Game-Ending Move**

- **Scenario:** A Skip power-up is activated on a turn that could end the game for the opponent.
- **Expected Outcome:**  
  The skip effect takes precedence, and the game continues appropriately.

**Rapid Key Presses**

- **Scenario:** Players press keys rapidly, attempting to drop multiple pieces in quick succession.
- **Expected Outcome:**  
  The game processes inputs in order and ignores any overlapping or invalid inputs.

**No Power-Ups Scenario**

- **Scenario:** Power-ups are disabled in the game settings.
- **Expected Outcome:**  
  The game runs without generating power-ups and maintains normal functionality.

## Contributions by Each Team Member

**Mingtao Wang:**

- Focused on managing game states to ensure smooth transitions and a consistent gameplay experience.
- Implemented backend game power-up logic including Bomb (clears a piece below), Skip (skips opponent's turn), and Obstacle (blocks board cells).
- Developed the foundational backend logic for power-up functionalities and created a game effect function interface for seamless frontend integration.
- Designed and refined power-up interactions to align with gameplay mechanics.
- Conducted testing to ensure the stability and scalability of the backend logic, especially after integrating other team members’ code.

**Ziang Jia:**

- Implemented the core backend logic supporting basic game functionalities (without add-on features).
- Built the dynamic board expansion feature triggered by ties.
- Refined power-up interactions.
- Conducted program testing to ensure backend stability and scalability after adding new features.
- Performed code cleanup and resolved compiler warnings.

**Zian Zhou:**

- Suggested the original Rusty Connect Four concept and guided the initial project direction.
- Contributed to foundational backend logic and established a seamless development setup.
- Built the frontend UI using Bevy, including rendering of the menu, game board, pieces, and power-ups.
- Ensured synchronization between backend and frontend for smooth runtime rendering.
- Enhanced UI/UX through critical bug fixes (e.g., board expansion issues, power-up consumption, drop-piece animations, obstacle removal).
- Implemented toggles for power-ups, introduced new visual themes, and refined color schemes.

## Lessons Learned and Concluding Remarks

Throughout this project, we learned:

1. **Entity-Component-System (ECS) Architecture**

   - **Core Idea:** ECS separates data (components) and behavior (systems), making code modular and reusable.
   - **Implementation:** Using Bevy’s ECS, we handled game logic for pieces, power-ups, and board expansions as independent components.
   - **Benefits:** Improved scalability, easier debugging, and maintainable game logic.

2. **Exploring Bevy Framework**

   - **Ease of Use:** Bevy’s beginner-friendly tools simplified 2D rendering, board expansions, and UI.
   - **Challenges:** Initial ECS design struggles and syncing real-time animations.
   - **Insights:** Scene management simplified menus and states, and Bevy’s rendering tools made animations smooth.

3. **Backend and Frontend Integration**

   - **Challenges:** Synchronizing backend logic (e.g., power-ups) with frontend visuals was complex.
   - **Success:** Achieved a seamless interaction, providing a smooth user experience with consistent performance.

4. **Refactoring for Clarity**

   - **Approach:** Iterative refactoring improved code readability.
   - **Outcome:** Easier maintenance, quicker onboarding for new developers, and simpler debugging.

5. **Incremental Development**

   - **Method:** Building and testing features step-by-step minimized bugs.
   - **Outcome:** Reduced errors and enabled focused, efficient progress.

6. **Educational Template**
   - **Purpose:** Serves as a reusable starting point for 2D board games in Rust.
   - **Highlights:** Demonstrated adding features like board expansions and power-ups without compromising clarity.
   - **Impact:** Showcased Rust’s potential for beginner-friendly, creative game development.

**Conclusion:**  
Rusty Connect Four evolved from a classic game into a feature-rich, dynamic, and educational example for new Rust game developers. We hope the lessons learned and the provided codebase inspire future projects and more innovations in the Rust gaming ecosystem.
