# Setup

## Go

- [Get Started](https://go.dev/learn/)
- go mod init website/server # inits the website/server module
- .gitignore setup
    - [github/gitignore/Go.gitignore](https://github.com/github/gitignore/blob/main/Go.gitignore)
- $ go run main.go # runs the server

## Tailwind CSS

- [Node.js](https://nodejs.org/en/download)
- [Tailwind CLI](https://tailwindcss.com/docs/installation/tailwind-cli)
- $ npx @tailwindcss/cli -i ./src/main.css -o ./src/output.css --watch # cli tool command to look for CSS classes to build

## Typescript

- [Node.js TypeScript](https://nodejs.org/api/typescript.html)
- $ npm install --save-dev tsx
- $ npx tsx <typescript_file>.ts # run typescript file

## GitHub

- [CLI For Adding Local to Repo](https://docs.github.com/en/migrations/importing-source-code/using-the-command-line-to-import-source-code/adding-locally-hosted-code-to-github)

## PostgreSQL

- [posgresql.com/download](https://www.postgresql.org/download/)
- [archwiki/postgresql](https://wiki.archlinux.org/title/PostgreSQL)
- Make sure to enable the service
- `sudo systemctl enable postgresql # enable postgresql service`
- `su postgres # login to postgres user`
- `initdb -D /var/lib/postgres/data # init a database`
- `pg_ctl -D /var/lib/postgres/data -l logfile start # start database server`
