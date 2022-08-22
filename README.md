# Simple Auth Workflow (Axum vs Actix)

Here I built a simple login form and underlying authentication logic using Axum and Actix to compare both against each
other.


## Axum

To start run:
- `cd axum`
- `cargo run`
- open [localhost:8080](http://localhost:8080/)


## Actix

To start run:
- `cd actix`
- `cargo run`
- open [localhost:8080](http://localhost:8080/)



## The Setup

- When initially opening `/` one will be redirected to `/login`.
- At `/login` one can login with the user name `jnp` and any password.
- When logged in, one will be redirected to `/`.
- Trying to login again will just redirect the user back to `/`.
- One can logout at `/logout` and will be redirected to `/login`.
