# Puzzle 3-1

## Reflection

This puzzle took more time than I expected. My hurdle the structuring of the data.

After playing with some idea I settled on a simpleish structure. The parsed schematic is stored in a **Hash Map** of Point -> Item where Item is a Number, symbol, or Other. The biggest hang up was representing these items because they have different lengths. I ended up with Item being an enum. Other holds no data, symbol holds the char that it contains, and Number holds a **Rc** of a Number struct. Then in the items Hash Map there is a Rc to the Number in each location of the number.

Parsing also took some time. My initial implmentation used a couple iterators to go through the lines, blocks of chars, and numbers, etc. This was a shitshow so I instead used a **RegEx** to iterate through each block of interesting symbols (e.g. blocks of periods, whole numbers). I am happy with the parsing implementation.

Finally, checking for adjacent symbols (`is_part_number()`) was a little cursed. I think my implementation is *fine*. I wish I would have implemented it generally for any location and size of symbols (like I did in [3-2](../puzzle-3-2/README.md)). I think a better way could be to store the position of every item in its struct and then have a trait/impl for `find_adjacent()` for any item.

As always, the actual computation was quite easy with Rust's iters.

I added a test to verify the correct result. Useful during refactoring.

### New Things I Used

- I enjoyed using the Add/Sub/From traits to make `Point` usage easier
- I became more comfortable with `Rc`.
- This is the first time I've used `HashMap`. Pretty neat.
- I find `matches!()` very useful.
- I struggle to use enum types correctly (usually I use them like subclasses of a parent class) but this time I think its use was very apt.

## Problem Statement

You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

```
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. **What is the sum of all of the part numbers in the engine schematic?**
