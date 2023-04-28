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
- If a config.json is not provided Hermod will go to default settings.
- If provided DEL_TOKEN is the key that will be used to authorized operations that override data, like *del* and *set*.

## Docker Installation
Example of docker-compose.yaml
```
services:
  hermod:
    image: grokepeer/hermod:0.2.3
    ports:
      - 2088:2088
    environment:
      - DEL_TOKEN=token
```

#### Notes:  
- If provided DEL_TOKEN is the key that will be used to authorized operations that override data, like *del* and *set*.  

Once the service is started and printed "Waiting on port..." the DB is ready to receive requests.

## API v0.3.0
To access Hermod API the Client needs to establish a TCP connection to Hermod on port 2088 and send "auth: "token"" with the right token to be authenticated or none if no authentication is needed or available. Once the connection is established successfully the host will send:  
```
Hermod - Connection established (v0.0.0, v0.0.0, Auth)
```

With the first v0.0.0 being the host version and the second being the API version that the host is using and then Auth or noAuth depending on wether the guest authenticated successfully as super-user. 
It is noteworthy to know that for a Client to connect only the API version needs to be matching as that dictates how the server will interact with the client.

#### Superuser note:  
A user who successfully authenticates as superuser can create and delete tables, use sup commands (such as gettab and getlen), delete records and override their content when calling a set query on an existing record.

#### Security note:  
The connection between host and client **is not encrypted** so all data shared between them is interceptable by any entity that has access to the network of the machines involved.

### Commands formatting:  
#### GET
```
get [data-key] from [tablename]
```

Get, given a data-key and the tablename will return the query results as described in the *Response formatting* section below.

##### Response codes:
- 200 the record was found and was returned to the user
- 400 bad request
- 404 table or recordkey given not found
- 500 generic host error

#### SET
```
set [data-key] in [tablename] to [data]
```

Set, given a data-key, a table and some data will save the data paired to the key in the DB. If key is already in use in the table it will return a 209 code, 200 if the data was successfully written to the DB.

##### Response codes:
- 201 the new record was successfully created in the table
- 200 the record was reset to a new content
- 400 bad request
- 404 table given not found
- 405 set request on an existing record with an unauthenticated user (non-superuser)
- 500 generic host error

#### DEL
```
del [data-key] from [tablename]
```

Del, given the data-key and the tablename will delete, if the record existed already, a key and it's data.

##### Response codes:
- 200 the record was deletec
- 400 bad request
- 404 table or recordkey given not found
- 405 delete request with an unauthenticated user (non-superuser)
- 500 generic host error

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
[response-data]{xxxxxxxxxxxx zzz}\u4
```

#### Notes:  
- \[response-data] is possibly sent in a different stream from the {tail} so the client must expect multiple streams dependently on the request and the response.
- *x* is the query execution time in nano seconds, reported as a 3 to 12 digits number. 
- *z* is an HTTP response code (200, 400, 404, 500...) that indicates the successfulness of the query server-side.
- Every request always terminates with a UTF8 character number 4 ("\u4").

## ANN details

In development

## Memory Leak Protection

In development