Trying out some stuff.  This code (and design) is exploratory, thus
the quality is quite low.  Major refactoring or rewriting will happen
prior to this being used anywhere.

That said, constructive criticism is welcome -- I'm learning lots of
new things with this.


NOTE: this project contains built files (in `static/` (haha)) and
files from old experiments such as React.  This may cause problems


# Building

These build instructions are what I remember doing.  They haven't been
verified.  This likely also depends on some libraries... More info to
come later

```
rustup install nightly-2017-08-11
rustup default nightly-2017-08-11
cargo install diesel --no-default-features --features postgres
mkdir -p migrations
diesel database setup
cargo build
<manual elm compilation>
cargo run
```

## Database

You'll have to setup postgres and make a .env file containing
something like:

```
DATABASE_URL=postgres://postgres:password@localhost/diesel
```


### Security

This code has not been audited or reviewed and is not fit for
production use.

Passwords are stored by hashing with Blake2b using a random salt of up
to 16 bytes.

By using this code you are taking responsibility for any loss or
damage that may occur.  If you notice any potential problems when
reviewing the code, please submit an issue or a pull request.

## Frontend

```
npm install -g elm
```
...


Elm files are being compiled using something like:
```
elm-make frontend/Article.elm --output=static/Article.html
```

This will be improved and automated in the near future.
