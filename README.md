# Wordle Solver

Imagine yourself, over two years into a pandemic and people start posting obscure grids on Twitter from a word game. You have flashbacks to sudoku. You refuse to play the game in earnest and resist the temptation to write some kind of solver. Eventually, though, you just give in.

If you must go down this road, you can begin your journey here. https://www.powerlanguage.co.uk/wordle/

## How to use this solver.

### Building it.

```
cargo build --release
```

### Using it.

To get started, simply run the solver. It will offer you a starting word and wait for you to enter the corresponding feedback.

```
$ ./target/release/wordle-solve
PALET >
```

Enter "PALET" as the first row of the puzzle. The feedback you receive from the puzzle will include green, yellow and black (or gray) squares. You can then enter the feedback into the prompt to get the next word you should guess.

Let's assume your feedback looks as follows:

⬜⬜⬜⬜🟨

Enter that feedback and the best next guess will be output along with a prompt for the corresponding feedback.

```
PALET > bbbby
NORTH >
```

As you can see, feedback is entered as 5 character expressions. The characters for each type of feedback are:

 - 🟩 "g"
 - 🟨 "y"
 - ⬜ "b"

### Just plain ole cheating

The method that wordle uses to select a word each day is pretty simple and can be replicated with just a few lines of code. If want to take away all the fun of the game or if you want to terrorize your friends by blurting out tomorrow's solution ahead of time, run:

```
./target/release/wordle-list-solutions
```

### How does this work?

The file `decision-tree.json` stores a decision tree with the best guess for any given series of feedback strings. The file can be built using the command `wordle-build-decision-tree`. However, that command only finds any decision tree that is capable of solving 100% of wordle puzzles. That command does not look for the optimal decision tree. Fortunately, [Alex Selby](http://sonorouschocolate.com/notes/index.php?title=The_best_strategies_for_Wordle) used a similar approach and did complete the taxing search for an optimal decision tree. Decision trees can be built from the output of his programs using `wordle-import-decision-tree`. Running that command with no arguments will import his [optimal solution](http://sonorouschocolate.com/notes/images/0/0e/Optimaltree.hardmode5.txt) for hard mode that is guaranteed to find every solution in 5 guesses.
