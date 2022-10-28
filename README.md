# Beginner programs
Goal of this repository is to log down solutions to "small" programs/exercises in a variety of programming languages (useful for learning new languages in general).

Right now, just Python and Rust.
Structure of the repository should be as follows:
```
beginner-programs/
|- 001_exercise-1/
|  |- description.md
|  |- python/
|  |- rust/
|- 002_exercise-2
```
While each exercise should preferably be geared towards a certain concept or two, it should not be *named* as such. The exercise name should be what the program is doing.

There are often multiple ways to achieve the same outcome.
Within each exercise, try to keep one folder per language only (the "best" way / most instructive way).
For other historical attempts, look at git history.
Prefer self-contained examples and fewer dependencies. This is not a repo for you to learn a particular library, e.g., use `argparse` not `click` or `rich`.

And within each language folder, you can structure the folder however required by the language best-practices.

`description.md` should contain the source of the problem (where I got this problem from), text description of the problem, and preferably a difficulty level.

## Managing dependencies
Haven't thought about this yet. But most likely will just use the global environment.

Since in preferring minimal solutions over heavier ones, we should be able to complete these without installing a dedicated venv for every single exercise.

## Resources
### Rust
* [Tour of rust](https://tourofrust.com/)
* [Practice.rs](https://practice.rs)
