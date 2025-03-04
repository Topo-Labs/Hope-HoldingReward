# 1.Get Users before signup
GET http://localhost:8000/api/v1/users
[Options]
output: users.txt
HTTP 200

# 2.Signup a new user
POST http://localhost:8000/api/v1/users/signup
Content-Type: application/json
{
    "name": "steve",
    "email": "jiege2@sina.com",
    "password": "123456"
}
HTTP 200

GET http://localhost:8000/api/v1/users
HTTP 200


curl -X POST -H "Content-Type: application/json" -d '{"name":"jiege", "email":"jiege@sina.com", "password": "123456"}' http://localhost:8000/api/v1/users/signup