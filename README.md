<div align="center">
  <img src="https://github.com/viktormarinho/enable/blob/main/static/enable.png" alt="Enable" style="height: 100px;" />
</div>

# Enable

Enable is a portable [feature flag](https://launchdarkly.com/blog/what-are-feature-flags/) (aka feature toggles) service, contained in a single executable file.

Taking inspiration from the [Pocketbase](https://pocketbase.io/) project, enable can be easily
downloaded as a single binary and run anywhere as long as [rust can compile to it](https://doc.rust-lang.org/nightly/rustc/platform-support.html).

Enable is built using [Axum](https://github.com/tokio-rs/axum), [Svelte](https://svelte.dev/) and utilizes [SQLite3](https://www.sqlite.org/index.html), 
one of if not the best alternative for having a portable database.

## How to run Enable
* Clone the repository (or create a codespace)
  * <code> git clone https://github.com/viktormarinho/enable </code>
* Install the sqlx cli
  * <code> Cargo install sqlx-cli </code>
* Reset database and do migrations
  * <code> sqlx db reset </code>
* Run the project
  * <code>Cargo run</code>
  
Be happy :)
