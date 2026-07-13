# Description

This repository contains scripts for loading RxNorm into a PostgreSQL database.

# Using the scripts

## Pre-requisites

- PostgreSQL database server, latest supported version preferred. To obtain,
  visit <https://www.postgresql.org/download/>.
- RxNorm full monthly release, latest preferred. To obtain, visit
  <https://www.nlm.nih.gov/research/umls/rxnorm/docs/rxnormfiles.html>.
- These scripts! Latest version is at:
  <https://gitlab.com/jello/RxNorm_pg/-/archive/master/RxNorm_pg-master.zip>.

## Load the data

1. Unzip the RxNorm full monthly file; for example: `unzip
   RxNorm_full_11042019.zip`.
2. If desired, create a new database; or, you can use an existing one.
3. Set your `PGDATABASE` environment variable to the database you'd like
   to populate. For example, if you want to use the database called `rxn`,
   run `export PGDATABASE=rxn`.
   Note: `RxNorm` is used by default if `PGDATABASE` is unset.
4. Run the load script <code>load_postgresql.sh *RRFDIR*</code>, where
   <code>*RRFDIR*</code> is the folder containing the RRF files from the
   unziped full monthly RxNorm release. For example, if you unzipped the
   RxNorm full montly release file into `~/data/RxNorm-20191104-full/`,
   then you would run `load_postgresql.sh ~/data/RxNorm-20191104-full/rrf/`.
   Note: `../../rrf/` is used by default if <code>*RRFDIR*</code> is omitted.
5. Ensure there are no errors in the output produced; warnings and notices
   are fine.
6. Login to the database and enjoy.

## Explore the data

Start running queries against the data! See the [sample queries](
RxNorm_SampleQueries_pg.sql) in this repository for inspiration.

# Additional information

## About PostgreSQL

PostgreSQL is a powerful, open source object-relational database system that
uses and extends the SQL language combined with many features that safely store
and scale the most complicated data workloads. PostgreSQL has earned a strong
reputation for its proven architecture, reliability, data integrity, robust
feature set, extensibility, and the dedication of the open source community
behind the software to consistently deliver performant and innovative solutions.
PostgreSQL runs on all major operating systems, is ACID-compliant, and has
powerful add-ons such as the popular PostGIS geospatial database extender.

For more, see: <https://www.postgresql.org/>.

## About RxNorm

RxNorm provides normalized names for clinical drugs and links its names
to many of the drug vocabularies commonly used in pharmacy management and drug
interaction software, including those of First Databank, Micromedex,
Gold Standard Drug Database, Multum and United States Pharmacopeia (USP)
Compendial Nomenclature. By providing links between these vocabularies, RxNorm
can mediate messages between systems not using the same software and vocabulary.

For more, see: <https://www.nlm.nih.gov/research/umls/rxnorm/>.
