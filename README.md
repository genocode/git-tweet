# git-tweet

This repo was created just for fun. It's a geeky dev game we made up. Basically it's a variation of the [story game](http://www.group-games.com/stationary-games/team-story-game.html) where each memer of the team adds the next line of the story. Here each developer adds a commit to the repository to create a program, while no one really knows where the thing is going.

## Topic

- theme: **Tweet. Anything related to twitter.**
- language: **Rust**

Disclaimer: we are *complete* Rust beginners, and want to learn it in this project.

## Basic rules

- each player can add only one commit at a time, successive commits must be from different players
- commits can contain up to 250 characters
- code can only be added, existing code must stay as it is, *unless* the pushed code doesn't compile, then it can be changed so that it compiles
- the exception is the "refactoring day", when each player can change existing code
- the actual date of the refactoring day is scheduled by the players
- one calendar month can contain up to 1 refactoring day
- the main program lives in the master branch, no one should commit directly to the master branch
- each player must create his own branch, named "player-$GITHUB_USERNAME"
- players commit code only to their branch and open pull requests to the master branch
- if the code doesn't compile, the PR has to be rejected / the player has to amend the commit
- if the code compiles and tests don't succeed, anothe player has to approve the PR
- if the code compiles and tests succeed, the PR can be merged without any approval of other members