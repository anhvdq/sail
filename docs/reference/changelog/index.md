---
prev: false
next: false
---

# Changelog

## 0.3.1

_July 7, 2025_

- Added support for the following SQL functions ([#570](https://github.com/lakehq/sail/pull/570), [#571](https://github.com/lakehq/sail/pull/571), [#582](https://github.com/lakehq/sail/pull/582), [#585](https://github.com/lakehq/sail/pull/585), and [#586](https://github.com/lakehq/sail/pull/586)):
  - `dayname`
  - `nullifzero`
  - `zeroifnull`
  - `split` (partial support)
  - `collect_set`
  - `count_if`
- Fixed issues with the `from_utc_timestamp` SQL function ([#596](https://github.com/lakehq/sail/pull/596)).
- Added support for the `DataFrame.sampleBy` method in the Spark DataFrame API ([#547](https://github.com/lakehq/sail/pull/547)).
- Added support for the following SQL statements ([#588](https://github.com/lakehq/sail/pull/588)):
  - `SHOW COLUMNS`
  - `SHOW DATABASES`
  - `SHOW TABLES`
  - `SHOW VIEWS`
- Improved data source listing performance ([#579](https://github.com/lakehq/sail/pull/579)).
- Improved the internal logic of data source options ([#587](https://github.com/lakehq/sail/pull/587) and [#598](https://github.com/lakehq/sail/pull/598)).
- Updated gRPC server TCP and HTTP configuration ([#593](https://github.com/lakehq/sail/pull/593)).

### Contributors

Huge thanks to [@SparkApplicationMaster](https://github.com/SparkApplicationMaster) for the first contributions related to SQL functions!

Huge thanks to [@davidlghellin](https://github.com/davidlghellin) for the continued contributions related to the Spark DataFrame API!

## 0.3.0

_June 28, 2025_

The 0.3.0 release introduces support for Spark 4.0 in Sail, alongside the existing support for Spark 3.5. One of the most notable changes in Spark 4.0 is the new `pyspark-client` package, a lightweight PySpark client. When using Sail in your PySpark applications, you can now choose to install this client package, instead of the full `pyspark` package that includes all the JAR files.

Here is a summary of the new features and improvements in this release.

- Improved remote data access performance by caching object stores ([#515](https://github.com/lakehq/sail/pull/515)).
- Added support for data reader and writer configuration ([#466](https://github.com/lakehq/sail/pull/466) and [#535](https://github.com/lakehq/sail/pull/535)).
- Added support for the following SQL functions ([#527](https://github.com/lakehq/sail/pull/527)):
  - `crc32`
  - `sha`
  - `sha1`
- Fixed issues with casting integers to timestamps ([#533](https://github.com/lakehq/sail/pull/533)).
- Fixed issues with the `random` and `randn` SQL functions ([#530](https://github.com/lakehq/sail/pull/530)).
- Added support for the `DataFrame.sample` method in the Spark DataFrame API ([#496](https://github.com/lakehq/sail/pull/496)).
- Added support for Spark 4.0 ([#467](https://github.com/lakehq/sail/pull/467), [#498](https://github.com/lakehq/sail/pull/498), and [#559](https://github.com/lakehq/sail/pull/559)).
- Updated the default value of a few configuration options ([#565](https://github.com/lakehq/sail/pull/565)).

### Breaking Changes

The `spark` "extra" has been removed from the `pysail` package. As a result, you can no longer use commands like `pip install pysail[spark]` to install Sail along with Spark. Instead, you must install the PySpark package separately in your Python environment.

This change allows you to freely choose the PySpark version when using Sail. Depending on your requirements, you can opt for either the `pyspark` package (Spark 3.5 or later) or the `pyspark-client` package (introduced in Spark 4.0).

### Contributors

We are thrilled by the growing interest from the community.
Huge thanks to [@rafafrdz](https://github.com/rafafrdz), [@davidlghellin](https://github.com/davidlghellin), [@lonless9](https://github.com/lonless9), and [@pimlie](https://github.com/pimlie) for making their first contributions to Sail!

## 0.2.6

_May 14, 2025_

- Improved temporal data type casting and display ([#448](https://github.com/lakehq/sail/pull/448)).
- Corrected the time unit for reading `INT96` timestamp data from Parquet files ([#444](https://github.com/lakehq/sail/pull/444)).
- Fixed issues with column metadata in the Spark DataFrame API ([#447](https://github.com/lakehq/sail/pull/447)).
- Supported referring to aliased aggregation expressions in Spark SQL `GROUP BY` and `HAVING` clauses ([#456](https://github.com/lakehq/sail/pull/456)).
- Supported more data formats and added directory listing endpoints in the MCP server ([#455](https://github.com/lakehq/sail/pull/455) and [#458](https://github.com/lakehq/sail/pull/458)).

## 0.2.5

_April 22, 2025_

- Corrected Spark session default time zone configuration and fixed various issues for timestamp data types ([#438](https://github.com/lakehq/sail/pull/438)).
- Improved object store setup and cluster mode task execution ([#432](https://github.com/lakehq/sail/pull/432)).

## 0.2.4

_April 10, 2025_

- Improved MCP server logging ([#421](https://github.com/lakehq/sail/pull/421)).
- Improved AWS S3 data access ([#426](https://github.com/lakehq/sail/pull/426)).
- Supported AWS credential caching ([#430](https://github.com/lakehq/sail/pull/430)).
- Fixed issues with cluster mode task execution ([#429](https://github.com/lakehq/sail/pull/429)).
- Supported `exceptAll()` and `tail()` in the Spark DataFrame API ([#417](https://github.com/lakehq/sail/pull/417)).

## 0.2.3

_March 21, 2025_

- Implemented MCP (Model Context Protocol) server ([#410](https://github.com/lakehq/sail/pull/410)).
- Supported the `hf://` protocol for reading Hugging Face datasets ([#412](https://github.com/lakehq/sail/pull/412)).
- Supported glob patterns in data source URLs ([#415](https://github.com/lakehq/sail/pull/415)).
- Supported a few data reader and writer options for CSV files ([#414](https://github.com/lakehq/sail/pull/414)).
- Fixed a few issues with SQL temporary views ([#413](https://github.com/lakehq/sail/pull/413)).
- Improved task error reporting in cluster mode ([#409](https://github.com/lakehq/sail/pull/409)).

## 0.2.2

_March 6, 2025_

- Switched to the built-in SQL parser ([#338](https://github.com/lakehq/sail/pull/338), [#358](https://github.com/lakehq/sail/pull/358), [#359](https://github.com/lakehq/sail/pull/359), and [#376](https://github.com/lakehq/sail/pull/376)).
- Supported the majority of Spark SQL syntax ([#378](https://github.com/lakehq/sail/pull/378), [#380](https://github.com/lakehq/sail/pull/380), [#382](https://github.com/lakehq/sail/pull/382), [#385](https://github.com/lakehq/sail/pull/385), [#387](https://github.com/lakehq/sail/pull/387), [#389](https://github.com/lakehq/sail/pull/389), and [#390](https://github.com/lakehq/sail/pull/390)).
- Expanded support for Spark SQL functions ([#364](https://github.com/lakehq/sail/pull/364), [#384](https://github.com/lakehq/sail/pull/384), and [#391](https://github.com/lakehq/sail/pull/391)).
- Fixed issues with `join()` in the Spark DataFrame API ([#392](https://github.com/lakehq/sail/pull/392)).
- Supported `NATURAL JOIN` in Spark SQL ([#396](https://github.com/lakehq/sail/pull/396)).
- Fixed an issue with SQL window expressions ([#386](https://github.com/lakehq/sail/pull/386)).
- Fixed result parity issues with derived TPC-DS queries ([#393](https://github.com/lakehq/sail/pull/393)).

## 0.2.1

_January 15, 2025_

- Supported SQL table functions and lateral views ([#326](https://github.com/lakehq/sail/pull/326) and [#327](https://github.com/lakehq/sail/pull/327)).
- Supported PySpark UDTFs ([#329](https://github.com/lakehq/sail/pull/329)).
- Improved literal and data type support ([#317](https://github.com/lakehq/sail/pull/317), [#328](https://github.com/lakehq/sail/pull/328), [#330](https://github.com/lakehq/sail/pull/330), and [#339](https://github.com/lakehq/sail/pull/339)).
- Supported `ANTI JOIN` and `SEMI JOIN` ([#337](https://github.com/lakehq/sail/pull/337)).
- Fixed a few PySpark UDF issues ([#343](https://github.com/lakehq/sail/pull/343)).
- Supported nested fields in SQL ([#340](https://github.com/lakehq/sail/pull/340)).
- Supported more queries in the derived TPC-DS benchmark ([#346](https://github.com/lakehq/sail/pull/346)).
- Supported more datetime functions ([#349](https://github.com/lakehq/sail/pull/349)).

## 0.2.0

_December 3, 2024_

We are excited to announce the first Sail release with the distributed processing capability. Spark SQL and DataFrame queries can now run on Kubernetes, powered by the Sail distributed compute engine. We also introduced a new Sail CLI and a configuration mechanism that will serve as the entrypoint for all Sail features moving forward.

We continued extending coverage for Spark SQL functions and the Spark DataFrame API. The changes are listed below.

- Supported the following DataFrame and SQL functions ([#278](https://github.com/lakehq/sail/pull/278) and [#305](https://github.com/lakehq/sail/pull/305)).
  - `DataFrame.crosstab`
  - `DataFrame.replace`
  - `DataFrame.to`
  - `reverse`
  - `aes_decrypt`
  - `aes_encrypt`
  - `try_aes_decrypt`
  - `base64`
  - `unbase64`
  - `weekofyear`
- Supported `mapInPandas()` and `mapInArrow()` for Spark DataFrame ([#310](https://github.com/lakehq/sail/pull/310)).
- Supported `applyInPandas()` for grouped and co-grouped Spark DataFrame ([#313](https://github.com/lakehq/sail/pull/313)).

### Breaking Changes

This release comes with the new Sail CLI, and the way to launch the Spark Connect server and PySpark shell is different from the 0.1.x versions. Please refer to the [Getting Started](/introduction/getting-started/) page for the updated instructions.

## 0.1.7

_November 1, 2024_

- Expanded support for Spark DataFrame functions ([#268](https://github.com/lakehq/sail/pull/268) and [#261](https://github.com/lakehq/sail/pull/261)).
  Added full parity and coverage for the following DataFrame and SQL functions.
  - `DataFrame.summary`
  - `DataFrame.describe`
  - `DataFrame.corr`
  - `DataFrame.cov`
  - `DataFrame.stat`
  - `DataFrame.drop`
  - `corr`
  - `regr_avgx`
- Fixed most issues with `ORDER BY` in the derived TPC-DS benchmark, bringing total coverage to 74 out of the 99 queries ([#261](https://github.com/lakehq/sail/pull/261)).

We also made significant changes to the Sail internals to support **distributed processing**. We are targeting the 0.2.0 release in the next few weeks for an MVP (minimum viable product) of this exciting feature. Please stay tuned! If you are interested in the ongoing work, you can follow [#246](https://github.com/lakehq/sail/issues/246) in our GitHub repository to get the latest updates!

## 0.1.6

_October 23, 2024_

- Supported `UNION` by name ([#253](https://github.com/lakehq/sail/pull/253)).
- Fixed a few issues with column references ([#254](https://github.com/lakehq/sail/pull/254) and [#257](https://github.com/lakehq/sail/pull/257)).

## 0.1.5

_October 17, 2024_

- Expanded support for Spark SQL syntax and functions ([#239](https://github.com/lakehq/sail/pull/239) and [#247](https://github.com/lakehq/sail/pull/247)).
  Added full parity and coverage for the following SQL functions.
  - `current_catalog`
  - `current_database`
  - `current_schema`
  - `hash`
  - `hex`
  - `unhex`
  - `xxhash64`
  - `unix_timestamp`
- Fixed a few issues with `JOIN` ([#250](https://github.com/lakehq/sail/pull/250)).

## 0.1.4

_October 03, 2024_

- Enabled Avro in DataFusion ([#234](https://github.com/lakehq/sail/pull/234)).
- Expanded support for Spark SQL syntax and functions ([#213](https://github.com/lakehq/sail/pull/213) and [#207](https://github.com/lakehq/sail/pull/207)).
  Added full parity and coverage for the following SQL functions.
  - `array`
  - `date_format`
  - `get_json_object`
  - `json_array_length`
  - `overlay`
  - `replace`
  - `split_part`
  - `to_date`
  - `any_value`
  - `approx_count_distinct`
  - `current_timezone`
  - `first_value`
  - `greatest`
  - `last`
  - `last_value`
  - `least`
  - `map_contains_key`
  - `map_keys`
  - `map_values`
  - `min_by`
  - `substr`
  - `sum_distinct`
- Supported HDFS ([#196](https://github.com/lakehq/sail/pull/196)).
- Supported parsing value prefixes followed by whitespace ([#218](https://github.com/lakehq/sail/pull/218) and [lakehq/sqlparser-rs#6](https://github.com/lakehq/sqlparser-rs/pull/6)).
- Added basic support for Python UDAF ([#214](https://github.com/lakehq/sail/pull/214)).

### Contributors

Huge thanks to our first community contributor, [@skewballfox](https://github.com/skewballfox) for adding support for HDFS!!

## 0.1.3

_September 18, 2024_

- Supported column positions in `GROUP BY` and `ORDER BY` ([#205](https://github.com/lakehq/sail/pull/205)).
- Expanded support for `INSERT` statements ([#195](https://github.com/lakehq/sail/pull/195)).
- Fixed issues with Spark configuration ([#192](https://github.com/lakehq/sail/pull/192)).
- Expanded support for `CREATE` and `REPLACE` statements ([#183](https://github.com/lakehq/sail/pull/183)).
- Supported `GROUPING SETS` aggregation ([#184](https://github.com/lakehq/sail/pull/184/files)).
- Integrated [fastrace](https://github.com/fast/fastrace) for more performant logging and tracing ([#166](https://github.com/lakehq/sail/pull/166)).
- Enabled gzip and zstd compression in Tonic ([#166](https://github.com/lakehq/sail/pull/166)).

## 0.1.2

_September 10, 2024_

- Fixed issues with aggregation queries.
- Extended support for SQL functions.
- Added support for temporary views and global temporary views.

## 0.1.1

_September 03, 2024_

- Extended support for SQL statements and SQL functions.
- Fixed a performance issue for the PySpark DataFrame `show()` method.

## 0.1.0

_August 29, 2024_

This is the first Sail release.
