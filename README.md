# Bulls and cows bot in Rust

The goal of this project is to [bulls and cows](https://en.wikipedia.org/wiki/Bulls_and_Cows) game. The algorithm is inspired by [this video](https://www.youtube.com/watch?v=v68zYyaEmEA) by 3Blue1Brown.

## Bulls and cows numerical version -- rules

The game starts with choosing a secret (random) 4-digit number with a constraint that the digits must be all different. The objective is to guess that number, given the clues. After each guess, which is also composed of nonrepeating digits, the player is given two numbers - number of bulls and number of cows. Bulls are the digits of the guessed number that are contained in the final answer and are in right positions. Cows are the digits that are containeed in the final answer, but are in wrong positions.

Example game may look like this: 

```
Number to guess: 8536

9435 1b 1c
3865 0b 4c
6538 2b 2c
5638 1b 3c
8536 4b 0c
```
