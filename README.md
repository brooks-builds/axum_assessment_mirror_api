# Mirror API Assessment

The Mirror API Assessment will test your ability to take in data and send it back to the caller.

## Skills 

The following skills are assessed:

- Able to read data passed in the path of a request
- Able to read data passed in query parameters of a request
- Able to read JSON data passed in the body of a request
- Able to respond to a request with JSON

## Instruction

To run the assessment, clone and checkout this repo to your local drive.

Run the server with the command `cargo watch -x run`. This will cause the server to restart every time you save a change.

All the code you need to update is in the [mirror](src/router/mirror.rs) route handler file. Currently the route isn't extracting any data from the request and isn't returning anything. Update the handler so that it

- extracts the path parameter defined in the router
- extracts the query variable defined in the router
- extracts the JSON passed in the body of the request
- returns the path parameter, query variable, and JSON in an object that has the following shape

```json
{
	"json": {
		"username": string,
		"password": string,
		"favorite_number": i32
	},
	"query": {
		"id": i32
	},
	"path": string
}
```

With the server running, the tests can be run with the command `cargo test`. The tests ensure that the above requirements are passing. However they are some other requirements that are necessary to pass this assessment.

- Code is formatted
- Cargo linting doesn't find any problems
- Clippy is happy

You can format your code with the command `cargo fmt`. It is possible to set auto-formatting on your code editor of choice so that this happens automatically when you save.

You can lint your code with the command `cargo check`. Cargo check can be configured to run in your editor of choice to tell you if there are linting problems.

Clippy is an optional linter that goes above and beyond the normal linting that Cargo check provides. It's a helpful tool as it can provide alternate (usually better) methods of writing the same code. You can run clippy with the command `cargo clippy`. It is possible to set clippy to run instead of the normal Cargo check linter so that you get it's lintings instead.

To run all of these checks run the shell script `./check.sh`. If there is output it will put it in a file named check.out. It will validate that the tests are passing, the code is formatted, and both the linters don't have any errors or warnings.

