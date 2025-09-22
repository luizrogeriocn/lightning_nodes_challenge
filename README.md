## Build tools & versions used

These were the main tools and libraries used:

- **Rust**: 1.90
- **SQLite**: 3.45
- **SQLx**: 0.6.2
- **Actix-web**: 4

## Steps to run the app

### Install Rust

Follow [the official instructions](https://www.rust-lang.org/tools/install)

### Clone and build the project

```bash
git clone https://github.com/luizrogeriocn/lightning_nodes_challenge.git
cd lightning_nodes_challenge
cargo build
```

### Run the server

```bash
cargo run
```

The server will start on port 8080 at localhost.

On startup the server will try to connect to the database and create it if it does not exist yet. It will also run any pending migrations.

Approximately every minute, the subroutine for importing nodes will run.

### Test the endpoint

```bash
curl http://127.0.0.1:8080/nodes
```

Or visit it in your browser [Get /nodes](http://127.0.0.1:8080/nodes)

When you first run the server, there will be no records to be displayed. After roughly a minute, the importation subroutine will run (it prints to the terminal when it starts and ends) and then we are able to see the results by refreshing the page.

## What was the reason for your focus? What problems were you trying to solve?

At first, I was mostly focused on having a web server running that could connect to a database and
read the data it should serve. So I decided to go with a Sqlite database as I figured it would be
simpler to get it running, and I also manually inserted the data so I could easily check if the code
was working as expected.

Once I got the server working, I tried to extract some of the code into other modules/files because
`src/main.rs` was a bit messy with all the code in it.

Then I implemented the "job" to import data from the [specified url](https://mempool.space/api/v1/lightning/nodes/rankings/connectivity), which is basically a spawned thread from the server process that continually checks if it's time to fetch the data again and, if it is, upserts it.

## How long did you spend on this project?

Roughly 6 hours in total.

## Did you make any trade-offs for this project? What would you have done differently with more time?

I am not sure if it counts as making trade-offs because I actually didn't know any better as
this was my first time using Rust. So I tried to have the simplest possible solution that met
the given specifications.

Here are a few areas that I think could have been done differently:

- Sqlite as the main database
- Cron job that runs inside the web server process

Even though I think Sqlite is a great fit for a project like this, if I was to deploy this to
production I would try and use Postgresql.

For the importing subroutine I would probably use the system's cron jobs and a library to handle background job processing outside of the web server's process.


These are areas that are completely lacking but that I would try to implement if I had more
familiarity with the Rust language and tooling:

- No pagination on GET /nodes
- No error handling
- No unit/integration tests
- No observability
- No ORM/QueryBuilder (raw sql usage)
- No environment-based settings
- No containerization

## What do you think is the weakest part of your project?

The ones mentioned above are the areas I think could have been improved.

I would prioritize the error handling and writing tests, though, as I believe
those would make the project more robust.

## Is there any other information you’d like us to know?

As I didn't (and still don't) know Rust, I tried to follow some resources I found related to what I
needed (or, at least, what I thought I needed) to implement to the best of my abilities:

- https://actix.rs/docs/getting-started
- https://medium.com/@mikecode/actix-web-project-build-a-simple-todo-list-api-with-sqlite-database-sqlx-crud-restful-api-cc5d13d3f50a
- https://blog.logrocket.com/making-http-requests-rust-reqwest/
- https://docs.rs/sqlx/latest/sqlx/
- https://docs.rs/reqwest/latest/reqwest/index.html
- https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.from_timestamp_millis
- https://docs.rs/chrono/latest/chrono/offset/struct.FixedOffset.html#method.east
- https://docs.rs/actix-jobs/latest/actix_jobs/trait.Job.html
- https://dev.to/chaudharypraveen98/scheduling-async-tasks-made-easy-with-actix-cron-1omi
- https://tpbabparn.medium.com/feasibility-of-implementing-cronjob-with-rust-programming-language-186eaed0a7d8
