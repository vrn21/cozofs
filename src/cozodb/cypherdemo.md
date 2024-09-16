Cypher Query Language Demo

MATCH (m:Movie {title: 'The Matrix'})<-[d:DIRECTED]-(p:Person)
RETURN p.name as director


MATCH (n: Node<{specifc condition}>) <-[d: ADJACENT]- (m:otherNode)
RETURN <>

Types in Cypher
INT8
INT16
INT32
INT64
INT128
UINT8
UINT16
UINT32
UINT64
FLOAT, FLOAT4
DOUBLE
DECIMAL
STRING
NULL
DATE
TIMESTAMP
INTERVAL
STRUCT
  Data Type	DDL definition
  STRUCT	STRUCT(a INT64, b INT64)
  You can extract a value from a STRUCT using the dot notation:
  RETURN {first: 'Adam', last: 'Smith'};

  WITH {first: 'Adam', last: 'Smith'} AS full_name
  RETURN full_name.first AS first_name;

MAP
  A MAP is a dictionary of key-value pairs where all keys have the same type and all values have the same type.
  MAP is similar to STRUCT in that it is an ordered list of mappings. However, MAP does not need to have the same keys present for each row,
  and is thus more suitable when the schema of an entity is unknown beforehand or when the schema varies per row.
  RETURN map([1, 2], ['a', 'b']) AS m;

UNION

BLOB
(Binary Large OBject) allows storage of an arbitrary binary object with up to 4KB in size in Kùzu.
RETURN BLOB('\\xBC\\xBD\\xBA\\xAA') as result;

SERIAL
SERIAL is a logical data type used for creating an auto-incrementing sequence of numbers,
typically used as a unique column identifier, similar to AUTO_INCREMENT feature supported by some other databases.

NODE
CREATE NODE TABLE Person(id SERIAL, name STRING, age INT64, PRIMARY KEY(id));
COPY Person FROM 'person.csv';
MATCH (a:Person) RETURN a;
