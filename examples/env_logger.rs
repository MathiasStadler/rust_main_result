/*!
Using `env_logger`.

Before running this example, try setting the `MY_LOG_LEVEL` environment variable to `info`:

```no_run,shell
$ export MY_LOG_LEVEL='info'

$ MY_LOG_LEVEL=info cargo run --example env_logger
$ MY_LOG_LEVEL=warn cargo run --example env_logger
$ MY_LOG_LEVEL=error cargo run --example env_logger
$ MY_LOG_LEVEL=wrong_level cargo run --example env_lo
gger # NO OUTPUT
MY_LOG_LEVEL=trace cargo run --example env_logger
MY_LOG_STYLE=never MY_LOG_LEVEL=trace cargo run --example env_logger
MY_LOG_STYLE=colors; MY_LOG_LEVEL=trace cargo run --example env_logger
# 

```

Also try setting the `MY_LOG_STYLE` environment variable to `never` to disable colors
or `auto` to enable them:

```no_run,shell
$ export MY_LOG_STYLE=never
```
*/

use log::{debug, error, info, trace, warn};

use env_logger::Env;

fn main() {
    // The `Env` lets us tweak what the environment
    // variables to read are and what the default
    // value is if they're missing
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");
}
