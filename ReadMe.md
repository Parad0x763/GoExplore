# Setup

## Rust

- `cargo new backend # creates the initial backend project`
- [Multithreaded Web Server](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
- std::mcp: Multi-producer, single-consumer FIFO queue communication primitives.
    - This module provides message-based communication over channels, concretely defined among three types:
        - [Sender]
        - [SyncSender]
        - [Receiver]
    - A [Sender] or [SyncSender] is used to send data to a [Receiver].
        - Both senders are clone-able (multi-producer) such that many threads can send simultaneously to one receiver (single-consumer).

## Tailwind CSS

- [Node.js](https://nodejs.org/en/download)
- [Tailwind CLI](https://tailwindcss.com/docs/installation/tailwind-cli)
- $ npx @tailwindcss/cli -i ./src/main.css -o ./src/output.css --watch # cli tool command to look for CSS classes to build

## Typescript

- [Node.js TypeScript](https://nodejs.org/api/typescript.html)
- `npm install --save-dev tsx`
- `npx tsc --init # creats tsconfig.json`
- `tsc <typescript_file>.ts # compile TypeScript to JavaScript`
- Include `"type": "module"` in the `package.json` file to correctly use modules

## GitHub

- [CLI For Adding Local to Repo](https://docs.github.com/en/migrations/importing-source-code/using-the-command-line-to-import-source-code/adding-locally-hosted-code-to-github)

## HTTP

- HTTP\1.1 treats headers as unordered, so the order of fields like Content-Length and Content-Type does not matter, it just needs to be placed before the body separator `\r\n\r\n`
- All headers are separated by `\r\n`
- Use `application/javascript` for JavaScript files

### Headers

- [MDN-Headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers)

## PostgreSQL

- [posgresql.com/download](https://www.postgresql.org/download/)
- [archwiki/postgresql](https://wiki.archlinux.org/title/PostgreSQL)
- Make sure to enable the service
- `sudo systemctl enable postgresql # enable postgresql service`
- `su postgres # login to postgres user`
- `initdb -D /var/lib/postgres/data # init a database`
- `pg_ctl -D /var/lib/postgres/data -l logfile start # start database server`
