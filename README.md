# pgsql_tester written in Rust

This is a small connection tester for PostgreSQL.

Execution examples:

```sh
$ ./pgsql_tester

PostgreSQL connection tester, https://github.com/aramcap/rust_pgsql_tester, 2021
Execution example:
./pgsql_tester postgresql://postgres:postgres@localhost:5432/postgres
```

If you want to check an up PostgreSQL DB:

```sh
$ ./pgsql_tester postgresql://postgres:postgres@localhost:5432/postgres
$ echo $?
0
```

But if any parameter is bad or the PostgreSQL DB is down:

```sh
$ ./pgsql_tester postgresql://postgres:postgres@localhost:5432/postgres
Error: Error { kind: Connect, cause: Some(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" }) }
$ echo $?
1
```
