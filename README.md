# Hermod In-Memory DB

### Hermod High Performance In-Memory Database System

Hermod is an In-Memory database management system that was designed for YBD. It uses a custom multi-threaded http server. Stores data in a key-based table that is optimized by an ANN to prioritize data based on access patterns.

## Installation
```
services:
  hermod:
    image: grokepeer/hermod:0.1.2
    ports:
      - 2088:2088
    environment:
      - HTTP_Threads=n
      - Del_Token=token
```

Once the service is started and printed "[Hermod] Up and running..." the DB is ready to receive requests.
In case env variables are not provided or they are unacceptable, Hermod will take the defaults (1 thread, "token" token)

## API

Hermod API supports get, set, and del operations which are described, in details, in the following paragraphs

### GET formatting:  
```
GET /get HTTP/1.1
Data-Key: key
```

- *Key* (required) is the unique ID of the data present in the DB

#### GET responses:  
- "200 OK"      data
- "404 ERROR"   "No Key found"
- "500 ERROR"   error

### SET formatting:  
```
GET /set HTTP/1.1
Content-Length: length
Data-Key: key
Del-Token: token

{
    data
}
```

- *Length* (required) must be an integer that describes the length, in bytes, of *data*
- *Key* (required) is the unique ID that is going to be associated with the *data*
- *Token* (optional) is an alphanumerical string that is set in the server settings.json file that allows overriding of existent data
- *Data* (required) will be saved in the DB paired with the key. Can be a string, a word, JSON, XML, anything.

#### SET responses:  
- "200 OK"      "Record created successfully"
- "200 OK"      "Record updated successfully"
- "403 Forbidden"   "Unauthorized request"
- "500 ERROR"   error

### DEL formatting:  
```
GET /del HTTP/1.1
Data-Key: key
Del-Token: token
```

- *Key* (required) is the unique ID that is going to be associated with the *data*
- *Token* (required) is an alphanumerical string that is set in the server settings.json file that allows overriding of existent data

#### DEL responses:  
- "200 OK"      "Record deleted"
- "403 Forbidden"   "Unauthorized request"
- "404 ERROR"   "No record to be deleted"
- "500 ERROR"   error

#### Notes:  
- The *Data-Key* is a unique alphanumerical identification for the each block of data stored in the DB
- A generic /set request will take the key and body of the HTTP request and save them paired to each other on the DB as Strings, no manipulation to the data is performed
- A /set request won't override an already existent record with the same key unless a destruction token is given along with it (as *Del-Token* Header)
- As for the overriding /set, the /del request requires the destruction token
- In case a destruction token (*Del-Token*) is not provided or is invalid, the server will return a "403 Forbidden" error

#### Invalid HTTP request behaviours:  
- If *Content-Length* is not provided within the first 500 bytes or if an invalid value is given (a character) the server will only read the first 500 bytes of the request
- In case no body is provided *Data-Key* must be in the first 500 bytes of the request along with all required headers

## ANN details

In development

## Memory Leak Protection

In development