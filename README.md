# Hermod In-Memory DB Host

### Hermod High Performance In-Memory Database System
Hermod is an In-Memory database management system that was designed for YBD. It uses a custom multi-threaded http server. Stores data in a key-based table that is optimized by an ANN to prioritize data based on access patterns.

## Docker Installation
```
services:
  hermod:
    image: grokepeer/hermod:0.1.2
    ports:
      - 2088:2088
    environment:
      - Del_Token=token
```

Once the service is started and printed "[Hermod] Up and running..." the DB is ready to receive requests.
In case env variables are not provided or they are unacceptable, Hermod will take the defaults ("token" token)

## API
To access Hermod API the Client needs to establish a TCP connection to Hermod on port 2088. Once the connection is established successfully the host will send:  
```
Hermod - Connection established (v0.0.0, v0.0.0)
```

With the first v0.0.0 being the host version and the second being the API version that the host is using. It is noteworthy to know that for a Client to connect only the API version needs to be matching as that dictates how the server will interact with the client.

#### Security note:  
The connection between host and client **is not encrypted** so all data shared between them is interceptable by any entity that has access to the network of the machines involved.

### GET formatting:  
```
get [data-key] from [tablename]
```

#### Notes:  
- The *Data-Key* is a unique alphanumerical identification key for the each block of data stored in the DB

## ANN details

In development

## Memory Leak Protection

In development