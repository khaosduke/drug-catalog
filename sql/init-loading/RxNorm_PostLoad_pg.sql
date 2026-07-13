/****
  Script to add supporting structure to loaded RxNorm database.
  Written by: Joseph Nahmias <joe@nahmias.net>
***/

-- Constraints for rxnatomarchive
ALTER TABLE rxnatomarchive ADD CONSTRAINT rxnatomarchive_pk PRIMARY KEY (rxaui,rxcui,merged_to_rxcui);

-- Constraints and Indexes for rxnconso
ALTER TABLE rxnconso ADD CONSTRAINT rxnconso_pk PRIMARY KEY (rxaui);
ALTER TABLE rxnconso ADD CONSTRAINT rxnconso_ck_suppress CHECK (suppress IN ('E','N','O','Y'));
CREATE INDEX rxnconso_ix_cui ON rxnconso (rxcui);
CREATE INDEX rxnconso_ix_str ON rxnconso USING GIN (to_tsvector('english', str));
CREATE INDEX rxnconso_ix_sab_code ON rxnconso (sab,code);
CREATE INDEX rxnconso_ix_tty ON rxnconso (tty);

-- Constraints for rxncui
ALTER TABLE rxncui ADD CONSTRAINT rxncui_pk PRIMARY KEY (cui1,cui2);

-- Indexes for rxndoc
CREATE INDEX rxndoc_ix_kv ON rxndoc (key, value);
CREATE UNIQUE INDEX rxndoc_uq_ef ON rxndoc (key,value) WHERE type = 'expanded_form';

-- Constraints and Indexes for rxnrel
ALTER TABLE rxnrel ADD CONSTRAINT rxnrel_ck_suppress CHECK (suppress IN ('E','N','Y'));
ALTER TABLE rxnrel ADD CONSTRAINT rxnrel_ck_dir CHECK (dir IN ('N','Y'));
ALTER TABLE rxnrel ADD CONSTRAINT rxnrel_ck_stypes_eq CHECK (stype1 = stype2);
CREATE UNIQUE INDEX rxnrel_uq_cui ON rxnrel (rxcui2,rel,rui) WHERE stype2 = 'CUI';
CREATE UNIQUE INDEX rxnrel_uq_aui ON rxnrel (rxaui2,rel,rui) WHERE stype2 = 'AUI';
CREATE INDEX rxnrel_ix_cui1 ON rxnrel (rxcui1,rela,rxcui2) WHERE stype1 = 'CUI';
CREATE INDEX rxnrel_ix_aui1 ON rxnrel (rxaui1,rela,rxaui2) WHERE stype1 = 'AUI';
CREATE INDEX rxnrel_ix_cui2 ON rxnrel (rxcui2,rela,rxcui1) WHERE stype2 = 'CUI';
CREATE INDEX rxnrel_ix_aui2 ON rxnrel (rxaui2,rela,rxaui1) WHERE stype2 = 'AUI';

-- Constraints for rxnsab
ALTER TABLE rxnsab ADD CONSTRAINT rxnsab_pk PRIMARY KEY (rsab);

-- Constraints for rxnsat
ALTER TABLE rxnsat ADD CONSTRAINT rxnsat_pk PRIMARY KEY (sab,atn,atv,rxaui);
ALTER TABLE rxnsat ADD CONSTRAINT rxnsat_ck_suppress CHECK (suppress IN ('N','O','Y'));

-- Constraints and Indexes for rxnsty
ALTER TABLE rxnsty ADD CONSTRAINT rxnsty_pk PRIMARY KEY (rxcui,tui);
CREATE INDEX rxnsty_ix_cui ON rxnsty (rxcui);

-- Derived Tables from rxndoc with PKs (for use in FKs below)
DROP TABLE IF EXISTS rxnatn;
CREATE TABLE rxnatn AS
SELECT d.value AS atn, d.expl AS attrname
FROM rxndoc d
WHERE d.key = 'ATN' AND d.type = 'expanded_form'
WITH DATA
;
ALTER TABLE rxnatn ADD CONSTRAINT rxnatn_pk PRIMARY KEY (atn);

DROP TABLE IF EXISTS rxnidt;
CREATE TABLE rxnidt AS
SELECT d.value AS idt, d.expl AS id_type
FROM rxndoc d
WHERE d.key = 'STYPE' AND d.type = 'expanded_form'
WITH DATA
;
ALTER TABLE rxnidt ADD CONSTRAINT rxnidt_pk PRIMARY KEY (idt);

DROP TABLE IF EXISTS rxnlat;
CREATE TABLE rxnlat AS
SELECT d.value AS lat, d.expl AS language
FROM rxndoc d
WHERE d.key = 'LAT' AND d.type = 'expanded_form'
WITH DATA
;
ALTER TABLE rxnlat ADD CONSTRAINT rxnlat_pk PRIMARY KEY (lat);

DROP TABLE IF EXISTS rxntty;
CREATE TABLE rxntty AS
SELECT d.value AS tty, d.expl AS termtype
FROM rxndoc d
WHERE d.key = 'TTY' AND d.type = 'expanded_form'
WITH DATA
;
ALTER TABLE rxntty ADD CONSTRAINT rxntty_pk PRIMARY KEY (tty);

-- Foreign Keys
ALTER TABLE rxnconso ADD CONSTRAINT rxnconso_fk_tty FOREIGN KEY (tty) REFERENCES rxntty (tty);
ALTER TABLE rxnconso ADD CONSTRAINT rxnconso_fk_lat FOREIGN KEY (lat) REFERENCES rxnlat (lat);
ALTER TABLE rxnconso ADD CONSTRAINT rxnconso_fk_sab FOREIGN KEY (sab) REFERENCES rxnsab (rsab);
ALTER TABLE rxnrel ADD CONSTRAINT rxnrel_fk_sab FOREIGN KEY (sab) REFERENCES rxnsab (rsab);
ALTER TABLE rxnrel ADD CONSTRAINT rxnrel_fk_stype1 FOREIGN KEY (stype1) REFERENCES rxnidt (idt);
ALTER TABLE rxnrel ADD CONSTRAINT rxnrel_fk_stype2 FOREIGN KEY (stype2) REFERENCES rxnidt (idt);
ALTER TABLE rxnsab ADD CONSTRAINT rxnsab_fk_lat FOREIGN KEY (lat) REFERENCES rxnlat (lat);
ALTER TABLE rxnsat ADD CONSTRAINT rxnsat_fk_atn FOREIGN KEY (atn) REFERENCES rxnatn (atn);
ALTER TABLE rxnsat ADD CONSTRAINT rxnsat_fk_stype FOREIGN KEY (stype) REFERENCES rxnidt (idt);
ALTER TABLE rxnatomarchive ADD CONSTRAINT rxnatomarchive_fk_lat FOREIGN KEY (lat) REFERENCES rxnlat (lat);
ALTER TABLE rxnatomarchive ADD CONSTRAINT rxnatomarchive_fk_sab FOREIGN KEY (sab) REFERENCES rxnsab (rsab);
ALTER TABLE rxnatomarchive ADD CONSTRAINT rxnatomarchive_fk_tty FOREIGN KEY (tty) REFERENCES rxntty (tty);
ALTER TABLE rxncuichanges ADD CONSTRAINT rxncuichanges_fk_sab FOREIGN KEY (sab) REFERENCES rxnsab (rsab);

-- Final vacuum and analyze
VACUUM FULL VERBOSE ANALYZE;
