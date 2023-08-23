# capabilities-rs
An example of how to structure your code in such a way that you can write what used to be complex and costly integration tests into simple and fast unit tests.

## The Problem
Developing is always better with lots of trusted unit tests, but maybe your code integrates with lots of external systems so you feel like everything has to be an integration test that calls the outside world. Maybe you read a database connection string from an environment variable so all your tests need an actual database to run. These tests are more complex to write, and take significantly longer to run.

## The Solution
Abstract over effects. If your application takes a lot of effectful actions, let the entry point of your program accept those effects as an input. Your main function will pass the true production effects, and your tests will pass pre-defined, deterministic dummy values so you can test everything but the integration itself quickly.

## Inspiration
This code was inspired by mtl-style Haskell, but this example does not attempt to reacreate higher kinded types nor monads in the Rust type system. Although both are possible to emulate, the resulting code is clunky and difficult to work with, which in my opinion defeats the purpose. This example is solely focused on solving the problem of turning as many tests as possible into unit tests.
