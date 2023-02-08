# Hermod In-Memory DB

### Hermod High Performance In-Memory Database System

Hermod is an In-Memory database management system that was designed for YBD. It uses a custom multi-threaded http server. Stores data in a key-based table that is optimized by an ANN to prioritize data based on access patterns.

## HTTP Request Standard

HTTP request formatting to Hermod is, in general, as follows:  
```
GET / HTTP/1.1
Content-Length: length
Data-Key: key
Del-Token: token

{
    //Informations to store
}
```
### The request path will have to be changed based on the requested operation:  
- /get to get all informations related to the key given
- /set to set a new record given the key
- /del to delete a record given the key

### Storage operation:  
- The *Data-Key* is a unique alphanumerical identification for the each block of data stored in the DB
- A generic /set request will take the key and body of the HTTP request and save them paired to each other on the DB as Strings, no manipulation to the data is performed
- A /set request won't override an already existent record with the same key unless a destruction token is given along with it (as Del-Token Header)
- As for the overriding /set, the /del request requires the destruction token

### Invalid HTTP request behaviours:  
- If *Content-Length* is not provided within the first 500 bytes or if an invalid value is given (a character) the server will only read the first 500 bytes of the request
- In case no body is provided *Data-Key* must be in the first 500 bytes of the request along with all required headers

## ANN details

In development

## Memory Leak Protection

In development