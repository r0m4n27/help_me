# Help-Me - Request help from tutors

Help-Me enables students to request help from their assigned tutors.
To request the help the user creates a request where a message can be left.
This message should contain a link where the student can be reached (e.g. a Zoom link).
The tutors can see all request and process them until all requests are done.

This project was done as part of the Rust course at the
[HSA](https://www.hs-augsburg.de/Informatik.html) and therefore uses Rust
for the front- and backend of the service.

![](./assets/screenshot.png)

## Libraries

The backend uses [sqlx](https://github.com/launchbadge/sqlx) to access the
sqlite database. Sqxl provides macros that turn the queries into rust code at compile time
and also checks their validity. It also handles the migrations of the database.
To serve the data to the frontend a simple REST-API is build using [Rocket](https://rocket.rs/).
It also serves the assets for the frontend.

The frontend is build using [seed](https://seed-rs.org/), which was inspired by [elm](https://elm-lang.org/).
Earlier versions of the frontend used [yew](https://yew.rs/) and [yedux](https://github.com/intendednull/yewdux).

## Setup

1. Install cargo packages

```sh
rustup target add wasm32-unknown-unknown
cargo install sqlx-cli --no-default-features --features sqlite,native-tls
cargo install trunk wasm-bindgen-cli
```

2. Set the `DATABASE_URL`

This can either be done manually or a `.env` file can be created
at the root of the project. [sqlx](https://docs.rs/sqlx/0.5.9/sqlx/sqlite/struct.SqliteConnectOptions.html)
describes a valid connection uri. (Note: This connection uri should be
an absolute path otherwise the migrations can't be run)

3. Perform the migrations

The migrations are run automatically. You only have to
create the sqlite database file that is at the `DATABASE_URL`.
If you want to perform the migrations manually you can run:

```sh
cd backend
sqlx migrate run
```

4. Build the frontend

```sh
cd frontend
trunk build --dist ../dist --release
```

5. Start the backend

The backend serves the api and frontend. The default config can be changed
in `Rocket.toml`. The application expects to be run at the root of the project
otherwise it won't find the `dist` folder. It will save it's logs `help_me.log`
and `user_actions.log` to the current directory.

```sh
cargo build --bin help_me-backend --release
cargo run --bin help_me-backend
(or just) ./target/release/help_me-backend
```

## Usage

### Admin

An admin account can be created from the register page.
This will be automatically done when the invite code field
is empty and the database doesn't hold another admin account.
This means that the first registered user won't be a tutor but an admin.
If another user tries to register an account without an invite code
the resulting request will lead to an error.

The admin has the ability to create create invite codes so that the
tutors can register their account and to delete accounts when they
are not needed anymore. The only information about the accounts
of the tutors the admin will see is their username.

### Tutor

A tutor can register an account with the provided invite code from the admin.
After the registration a list of all open requests is provided.
The tutor can start to process the requests and after he helped the student
the request can be finished and the tutor will be taken to the index page.

### User

A unregistered user can create a request on the index page.
He provides a title and description (which should a link of some kind of meeting
software) that will be shown to the tutor. The request will be shown until
the tutor finishes it. After the request is submitted the user has the ability
to change the content and resolve it by himself if the help from the tutor is not necessary.


## Useful Resources

Other projects/blog posts that helped me in creating this project:

- [RESTful API in Sync & Async Rust](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md)
- [Rocket](https://rocket.rs/)
- [Time Tracker (Seed)](https://github.com/MartinKavik/seed-app-time-tracker)
