# Backend
## prerequisites:
- Rust 
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ rustup install nightly
$ rustup override set nightly
```
- [mongoDB running](https://docs.mongodb.com/manual/mongo/#start-the-mongo-shell-and-connect-to-mongodb)

---
## Running: 
```
$ cargo run
```

---
## Build: 
```
$ cargo build
```

---
## Usage:
### Create a user
```
    POST http://localhost:8000/api/user
     body = {
                "name": String,
                "email": String
            }

```
---
### Create all users
```
    GET http://localhost:8000/api/users

```
---
### Create a tweet
```
    POST http://localhost:8000/api/tweet
     body = {
                "text": String,
                "user_id": String
            }

```
### Get all tweets
```
    GET http://localhost:8000/api/tweets
    

```
### Get all tweets by profile id
```
    GET http://localhost:8000/api/tweets/profile/{user_id}
    

```
### Get a tweet by id
```
    GET http://localhost:8000/api/tweet/{tweet_id}
    

```
### Like a tweet
```
    PUT http://localhost:8000/api/tweet/like
    body = {
                "tweet_id": String,
	            "user_id": String
            }

```
### Retweet a tweet
```
    PUT http://localhost:8000/api/tweet/retweet
    body = {
                "tweet_id": String,
	            "user_id": String
            }

```

#### Support: 
- Weldisson Araujo
- email: weldisson.araujo@gmail.com

---
