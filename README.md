# Wordle Solver

Imagine yourself, over two years into a pandemic and people start posting obscure grids on Twitter from a word game. You have flashbacks to sudoku. You refuse to play the game in earnest and resist the temptation to write some kind of solver. Eventually, though, you just give in.

If you must go down this road, you can begin your journey here. https://www.powerlanguage.co.uk/wordle/

## NOTE: This is a WIP and it's very likely broken.

This is a fun little exploratory project where I'm tinkering with various approaches. This code will be broken a lot until I remove this banner.

## How to use this solver.

### Building it.

```
cargo build --release
```

### Using it.

To get started, simply run the solver. Each time you get feedback from the puzzle, you will run the command again adding the feedback you received on the command line.

```
$ ./target/release/wordle-solve
alert
```

Enter "alert" as the first row of the puzzle. The feedback you receive from the puzzle will include green, yellow and gray squares. You will then add that to the command line for the next run. Let's assume your feedback looks as follows:

🟨⬜⬜⬜⬜

The next command to run will be:

```
$ ./target/release/wordle-solve ybbbb
piano
```

Enter "piano" into the 2nd line of the puzzle. Let's now assume you get feedback of:

🟩🟨🟨🟨⬜

The next command to run will be:

```
$ ./target/release/wordle-solve yxxxx gyyyx
panic
```

Enter "panic" into the 3rd line.

As you can see, feedback is entered as 5 character expressions. The characters for each type of feedback are:

 - 🟩 "g"
 - 🟨 "y"
 - ⬜ "b"
