# Restful Example

`Restful` example using `RUST` language

## Usage

```bash
$ mv .env.sample .env
## Update .env file
$ cargo build
```

## Endpoints
- `GET`
  - `/`  :  Print `Hello World`
  - `/users` : Print all of users
  - `/user/:id` : Print `:id` user
- `POST`
  -  `/users` : Create new user ( With `{"name": "user_name", "height": INTEGER}` body params )
- `PATCH`
   - `/user/:id` : Update user ( With `{"name": "user_name", "height": INTEGER}` body params)
- `DELETE`
  - `/user/:id` : Delete user

## Change Log

Current Version 0.1.0

## Donate

Welcome to Donation :)

- Ƀ BTC : 16MdVNJgvGYbVuaC6KrjGNy2RCrNsaPaZz
- ETC : 0xc45b10108920b5f20e574bbf021e73d93af5dbc8
