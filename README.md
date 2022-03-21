Rust Philly's Newsletter subscription service

This repo is the result of Rust Philly members reviewing each chapter of https://zero2prod.com.

If you're looking for something to quickly deploy for yourself, take a look at Luca's full source code here: https://github.com/LukeMathWalker/zero-to-production

### Trade-off
Rust Philly is working through each chapter to Zero2Prod book. We figure it would be neat to deploy our own production-grade newsletter service, but we also are in this meetup to level-up in Rust, so that we can be more effective in our day jobs. There is a trade-off here to acknowledge.

#### Fork and deploy, learn implementation details later
PROS:
- You will have a deployable production grade MVP in the short-term.

CONS:
- You may not be equipped to effectively maintain said API.
- You may not be able to propose engineering improvements to other API codebases because you skipped understanding the techniques applied during the construction of this codebase.

#### Read every chapter of Luca's Zero2Prod book and walk through an iterative TDD practice
PROS:
- You will be equipped to effectively maintain this repository.
- You will be able to propose engineering improvements to other API codebases based off of learnings from this book.
- You will still have a deployable production-grade API MVP.

CONS:
- You will have to spend more time upfront (to read the book).
