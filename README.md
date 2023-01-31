# Hermod In-Memory DB

### Hermod High Performance In-Memory Database System

Hermod is an In-Memory database system that was designed for YBD. It uses a custom multi-threaded http service. Stores data in a key-based table that is optimized by an ANN to prioritize data based on access patterns.

### HTTP Request Standard

HTTP request formatting to Hermod is at minimum as follows:  
```
GET / HTTP/1.1
Content-Length: length

{
    "sid": "******",
    "token": "******"
}
```

### ANN details

...

### Memory Leak Protection

...