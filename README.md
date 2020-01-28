# Backend
## prerequisites:
- Rust
- cargo
- mongoDB running

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