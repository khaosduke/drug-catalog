#!/bin/sh
# Script to load the RxNorm database into PostgreSQL
# Written by Joseph Nahmias <joe@nahmias.net>

DEFDB=RxNorm
RRFDIR="${1:-../../rrf/}"

echo Starting to load into database ${PGDATABASE:=${DEFDB}}.
export PGDATABASE

echo Dropping/Creating RxNorm tables
psql -1 -f RxNorm_DDL_pg.sql

for rrf in ${RRFDIR}/*.RRF
do
  TBL=`basename ${rrf} .RRF`
  echo Loading ${TBL} from ${rrf} ...
  SQL=""
  SQL="${SQL} BEGIN TRANSACTION;"
  SQL="${SQL} TRUNCATE TABLE ${TBL};"
  SQL="${SQL} COPY ${TBL} FROM STDIN WITH (FORMAT text, FREEZE, NULL '', DELIMITER '|');"
  SQL="${SQL} COMMIT;"
  # escape backslashes in the data, strip trailing delimiter
  sed -r -e 's#\\#\\\\#g' -e 's/\|$//' ${rrf} | psql -c "${SQL}"
done

echo Performing post-load tasks: PK, FK, Indexes, etc ...
psql -f RxNorm_PostLoad_pg.sql

echo Finished loading into database ${PGDATABASE}.
