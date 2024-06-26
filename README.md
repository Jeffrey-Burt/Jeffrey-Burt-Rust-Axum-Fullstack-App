# Jeffrey Burt's Full Stack Rust Axum web application
Web application using the RUST Axum web framework. 

Useful things to download alongside this project:
* Git https://git-scm.com/download/win
* Rust https://www.rust-lang.org/learn/get-started
* Postman https://www.postman.com/downloads/
* VSCode https://code.visualstudio.com/download
* MySQL Shell https://dev.mysql.com/downloads/workbench/
* MySQL Server https://dev.mysql.com/downloads/file/?id=526084

To start out:

```git clone https://github.com/Jeffrey-Burt/Jeffrey-Burt-Rust-Axum-Fullstack-App.git```

```cd Jeffrey-Burt-Rust-Axum-Fullstack-App```

```cargo build```

Long as everything compiles properly, start the local service with the following:

```cargo run```

---

If you want continuous modification and execution, you can use cargo-watch

```cargo install cargo-watch```

Once installed, run cargo-watch with the following command:

```cargo-watch -qcx run```

---

Notes:

*DO NOT* put ; after HtmlTemplate();

```the trait Template is not implemented for AboutTemplate, which is required by HtmlTemplate<AboutTemplate>: axum::response::IntoResponse```
^ This error is caused by the path being incorrect on one of the #[template(path = "foo/bar")]


If the error is that a "result was found", then you have to .unwrap() the function

---

VSCode Extensions Installed:

* Even Better TOML
* Error Lens
* CodeLLDB
* Postman
* rust-analyzer