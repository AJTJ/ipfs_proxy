Setup:
- clone this repo
- start an ipfs node
- start a postgres docker image
`docker run --name ipfs-node-postgres -e POSTGRES_PASSWORD=mysecretpassword -dp 5432:5432 postgres`
- start/compile the server with `cargo run`

Some things (amongst others) I would change for PROD.
- better session management
- much better error handling
- edge and not-so-edge case handling
  - such as checking for emails/accounts that already exist
- more information and more specific data types stored in the database
- password recovery
- write some tests


Schema
- user
  - id
  - email
  - pw_hash
  - salt
- api_key
  - id
  - user_id
  - key_value
  - is_enabled
- key_requests
  - id
  - api_key_id
  - date_time
  - key_was_enabled

STEPS
- endpoints
  - get user/return data
  - get photo
  - delete api key
  - get all requests
- ensure that photo can be downloaded via the proxy server
- limit node access to only coming from the proxy server
- log requests in proxy server
  - add requests with date/time stamp to database schema