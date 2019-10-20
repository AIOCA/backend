# AIOC-BACKEND

The backend and database server for All In One Compute App.

# Setup
## Rust
* [Install rust compiler and tools](https://www.rust-lang.org/tools/install)
* Install nightly toolchain
        
        `rustup install nightly`

* Install [diesel_cli](http://diesel.rs/guides/getting-started/)
   
        `cargo install diesel_cli --no-default-features --features mysql`

## MySQL
* Install a mysql database server (mariadb recommended)
* Create database named `aioc`
    
        `CREATE DATABASE aioc;`

## Other Configurations
* **Generate Secret Key** 

        This backend requires a secret key to be generated for generating,encoding and decoding jwt tokens.
        
        `head -c16 /dev/urandom > secret.key`
* **Environment Variables**

        DATABASE_URL must exported for diesel framework.
        
        `echo DATABASE_URL=mysql://username:password@localhost/aioc > .env`

        Replace username and password with your database configurations.
* **Migrations**
        
        Use diesel_clie in order to create tables and relations.
        
        `diesel migration run`
         
# Building

The nightly compiler must be used to compile the backend.Set it as default toolchain.

`rustup override set nightly` 

* Debug builds

        `cargo build`
* Release builds

        `cargo build --release`

