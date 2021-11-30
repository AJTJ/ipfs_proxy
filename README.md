Setup:
- clone this repo
- start an ipfs node with default settings
- start a postgres docker image
`docker run --name ipfs-node-postgres -e POSTGRES_PASSWORD=mysecretpassword -dp 5432:5432 postgres`
- ensure you have postgres installed on your machine (diesel requires)
- install diesel cli `cargo install diesel_cli --no-default-features --features postgres`
- run `diesel migration run` while in the repo to build database
- start/compile the server with `cargo run`
- interact with the server at `http://127.0.0.1:8090/` through postman or curl

Endpoints (with JSON data, if required)
```
- #[post("/register")]
  - {email, password} 
- #[post("/login")]
  - {email, password} 
- #[post("/logout")]
- #[get("/getapikey")]
- #[post("/disablekey")]
  - {api_key}
- #[post("/interactnode")]
  - {api_key}
```

Some things (amongst others) I would change for PROD.
- Separate things into services: simpleapi, auth etc...
- much better error handling
  - Especially with I/O processes
- much better session management
  - probably running redis
- improved data retrieval (turning requests into a CSV per api_key, perhaps)
- edge and not-so-edge case handling
  - such as checking for emails/accounts that already exist
- perhaps more information and more specific data types stored in the database
- password recovery
- email verification
- write some tests
- more checks to ensure that api_keys are valid, user id is valid, etc.
- containerization to ensure endpoint security

STEPS
- ensure that photo can be downloaded via the proxy server
- limit node access to only coming from the proxy server
- log requests in proxy server
  - add requests with date/time stamp to database schema