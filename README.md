# Voskhod

![Voskhod rocket](https://render.fineartamerica.com/images/rendered/default/print/5.500/8.000/break/images-medium-5/voskhod-2-rocket-on-launchpad-science-photo-library.jpg)

This is a basis for a [prelaunchr-like](https://github.com/harrystech/prelaunchr) web app.

This project is accompanied by a condensed walkthrough I wrote [on my blog](https://alexeyzabelin.com/voskhod-rocket/),
which is in turn based on the following series of posts:

[Making a simple blog with Rust - Part I](https://notryanb.github.io/rust-blog-series-1.html)

[Making a simple blog with Rust - Part II](https://notryanb.github.io/rust-blog-series-2.html)


# Running locally

```bash
git clone https://github.com/alexeyzab/voskhod
cd voskhod
cargo build
cargo install diesel_cli --no-default-features --features postgres
```

Create a `.env` file with the following contents:

```
DATABASE_URL=postgres://username:password@localhost/voskhod
```

Then:

```bash
diesel setup
diesel migration run
cargo run --bin voskhod
```

