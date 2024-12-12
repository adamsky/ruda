## <img src="dash/assets/images/ruda-logo.png" width="300">

[![Static Badge](https://img.shields.io/badge/discord-server-blue)](https://discord.gg/gyYhPyTy4s)

Open source, self-hostable, rather unorthodox alternative to popular PaaS
products such as Coolify, Heroku, Netlify, Vercel.

`ruda` enables managing single-binary deployments (what some people call
"fat binaries") directly *on the metal*, as opposed to *using docker*.

If you want to manage multiple single-binary Rust applications spanning
multiple machines, there is a chance you might find this project useful.

The name is a vague reference to the Rust programming language; in many
indo-european languages *ruda* means *ore*, and in some slavic languages
it's a slang word for rust itself.


## Features

- single-binary-app deployment 
- self-hosted app runners 
- [more](https://ruda.app/features)


## Goals

Find the simplest way to deploy Rust applications. 

Include common QOL improvements (automatic push-based deployment, staging
environments, storage backups, etc.).

Integrate with selected 3rd party providers like Github (source access),
Cloudflare (DDOS protection, TLS certificates), Hetzner (server provisioning
and management).

Provide a web dashboard for live management and monitoring as well as a CLI
application.


## Quick start

Install the `ruda` CLI:

```
cargo install ruda-cli
```

Log in with your [`ruda.app`](https://ruda.app) credentials:

```
ruda login
```

Initialize a new hello world app project:

```
ruda new app --template hello_world && cd app
```

Deploy the application with default settings:

```
ruda deploy 
```

