# Hermod In-Memory DB Host

### Hermod High Performance In-Memory Database System
Hermod is an In-Memory database management system that was designed for YBD. It uses a custom multi-threaded TCP handle server. Stores data in a key-based table that is optimized by an ANN to prioritize data based on access patterns.

## Docker Installation
```
services:
  hermod:
    image: grokepeer/hermod:0.2.2
    ports:
      - 2088:2088
```

Once the service is started and printed "[Hermod] Up and running..." the DB is ready to receive requests.

## API
To access Hermod API the Client needs to establish a TCP connection to Hermod on port 2088. Once the connection is established successfully the host will send:  
```
Hermod - Connection established (v0.0.0, v0.0.0)
```

With the first v0.0.0 being the host version and the second being the API version that the host is using. It is noteworthy to know that for a Client to connect only the API version needs to be matching as that dictates how the server will interact with the client.

#### Security note:  
The connection between host and client **is not encrypted** so all data shared between them is interceptable by any entity that has access to the network of the machines involved.

### Commands formatting:  
```
get [data-key] from [tablename]

del [data-key] from [tablename]

set [data-key] in [tablename] to [data]

sup create [tablename]

sup delete [tablename]

sup getlen [tablename]
```

#### Notes:  
- The *Data-Key* is a unique alphanumerical identification key for the each block of data stored in the DB, it cannot contain spaces
- *Tablename* is a unique ID for each table, it cannot contain spaces

### Response formatting:  
```
[data]{xxxxxxxxxxxx zzz}
```

#### Notes:  
- *x* is a 4 to 12 digits number that is the query execution time in nano seconds
- *z* is an HTTP response code

## ANN details

In development

## Memory Leak Protection

In development