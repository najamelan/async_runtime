# Async Runtime

## Design goals

## API

Three different execution models:

- global spawn function
- Spawn and SpawnLocal traits so functions can take them in as generic params
- nursery

## Implementation

### Spawn and SpawnLocal traits

We can probably just use the ones from the futures library unless something is very wrong with them. This way we don't have to double the traits. We will create wrapper types for the different executors, so we can implement the traits for them.

### Nursery

Separate library which takes an executor as a generic param based on the traits, and inplements the traits on itself, much like Nemo suggested.
