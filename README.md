# Hermod In-Memory DB Host

### Hermod High Performance In-Memory Database System
Hermod is an In-Memory database management system that was designed for YBD. It uses a custom multi-threaded TCP handle server. Stores data in a key-based table that is optimized by an ANN to prioritize data based on access patterns.

## Hermod Installation
To get Hermod working you only need to build the executable for your OS, or get the last executable built in the GitHub release page here, and create a config.json file in the same folder as the executable. This will look like this:

```
{
  "token": "VeryComplexToken"
}
```

#### Notes:  
- If a config.json is not provided Hermod will go to default.
- If provided DEL_TOKEN is the key that will be used to authorized operations that override data, like *del* and *set*.

## Docker Installation
```
services:
  hermod:
    image: grokepeer/hermod:0.2.2
    ports:
      - 2088:2088
    environment:
      - DEL_TOKEN=token
```

#### Notes:  
- If provided DEL_TOKEN is the key that will be used to authorized operations that override data, like *del* and *set*.  

Once the service is started and printed "Waiting on port..." the DB is ready to receive requests.

## API
To access Hermod API the Client needs to establish a TCP connection to Hermod on port 2088. Once the connection is established successfully the host will send:  
```
Hermod - Connection established (v0.0.0, v0.0.0)
```

With the first v0.0.0 being the host version and the second being the API version that the host is using. It is noteworthy to know that for a Client to connect only the API version needs to be matching as that dictates how the server will interact with the client.

#### Security note:  
The connection between host and client **is not encrypted** so all data shared between them is interceptable by any entity that has access to the network of the machines involved.

### Commands formatting:  
#### GET
```
get [data-key] from [tablename]
```

Get, given a data-key and the tablename will return two streams of data, the first, if the reading was successful, will return the data that was saved paired with the key, the second stream will send the query results as described in the *Response formatting* section below.

#### SET
```
set [data-key] in [tablename] to [data]
```

Set, given a data-key, a table and some data will save the data paired to the key in the DB. If key is already in use in the table it will return a 209 code, 200 if the data was successfully written to the DB.

#### DEL
```
del [data-key] from [tablename]
```

Del, given the data-key and the tablename will delete, if the record existed already, a key and it's data.

#### GETLEN
```
sup getlen [tablename]
```

Getlen is a command in the suite of the Sup-er user that retuns the number of records written on a table.

#### Notes:  
- The *Data-Key* is a unique alphanumerical identification key for the each block of data stored in the DB, it cannot contain spaces
- *Tablename* is a unique ID for each table, it cannot contain spaces

### Response formatting:  
```
[response-data]{xxxxxxxxxxxx zzz}
```

#### Notes:  
- \[response-data] is sent in a different stream from the {tail} so the client must expect one or two streams dependently on the request and the response.
- *x* is the query execution time in nano seconds, reported as a 4 to 12 digits number. 
- *z* is an HTTP response code (200, 400, 404, 500...) that indicates the successfulness of the query server-side.

## ANN details

In development

## Memory Leak Protection

In development