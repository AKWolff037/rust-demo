# rust-demo
A rust application used for learning - simple web service with CRUD commands to a postgres backend

A test project for writing an application in Rust that connects to a Postgres DB and services http requests.  Increments and returns a sequence number per unique key given.
Should have user-level authority so users can only see the sequences that they own.

Eventually will support being able to pass in a format string, so a returned sequence can be formatted appropriately.  
I.e. `{ "format": "%6d" }` would return `"000002"`

