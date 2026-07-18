/****
Script to create the RxNorm tables to be loaded.
Written by: Joseph Nahmias <joe@nahmias.net>
***/

-- DROP all loaded tables
DROP TABLE IF EXISTS rxncui;
DROP TABLE IF EXISTS rxncuichanges;
DROP TABLE IF EXISTS rxndoc;
DROP TABLE IF EXISTS rxnsty;
DROP TABLE IF EXISTS rxnsat;
DROP TABLE IF EXISTS rxnrel;
DROP TABLE IF EXISTS rxnconso;
DROP TABLE IF EXISTS rxnatomarchive;
DROP TABLE IF EXISTS rxnsab;
DROP TABLE IF EXISTS rxntty;
DROP TABLE IF EXISTS rxnlat;
DROP TABLE IF EXISTS rxnatn;

CREATE TABLE rxnatomarchive
(
  rxaui               INTEGER NOT NULL,
  aui                 VARCHAR(10) NOT NULL,
  str                 VARCHAR(4000) NOT NULL,
  archive_timestamp   DATE NOT NULL,
  created_timestamp   TIMESTAMPTZ NOT NULL,
  updated_timestamp   TIMESTAMPTZ NOT NULL,
  code                VARCHAR(50) NOT NULL,
  is_brand            VARCHAR(1),
  lat                 VARCHAR(3) NOT NULL,
  last_released       VARCHAR(30),
  saui                VARCHAR(50),
  vsab                VARCHAR(40),
  rxcui               INTEGER NOT NULL,
  sab                 VARCHAR(20) NOT NULL,
  tty                 VARCHAR(20) NOT NULL,
  merged_to_rxcui     INTEGER NOT NULL
);

CREATE TABLE rxnconso
(
  rxcui     INTEGER NOT NULL,
  lat       VARCHAR(3) DEFAULT 'ENG' NOT NULL,
  ts        VARCHAR(1),
  lui       VARCHAR(8),
  stt       VARCHAR(3),
  sui       VARCHAR(8),
  ispref    VARCHAR(1),
  rxaui     INTEGER NOT NULL,
  saui      VARCHAR(50),
  scui      VARCHAR(50),
  sdui      VARCHAR(50),
  sab       VARCHAR(20) NOT NULL,
  tty       VARCHAR(20) NOT NULL,
  code      VARCHAR(50) NOT NULL,
  str       VARCHAR(3000) NOT NULL,
  srl       VARCHAR(10),
  suppress  VARCHAR(1),
  cvf       INTEGER
);

CREATE TABLE rxnrel
(
  rxcui1    INTEGER,
  rxaui1    INTEGER, 
  stype1    VARCHAR(50) NOT NULL,
  rel       VARCHAR(4) NOT NULL,
  rxcui2    INTEGER,
  rxaui2    INTEGER,
  stype2    VARCHAR(50) NOT NULL,
  rela      VARCHAR(100),
  rui       INTEGER,
  srui      VARCHAR(50),
  sab       VARCHAR(20) NOT NULL,
  sl        VARCHAR(1000),
  dir       VARCHAR(1),
  rg        VARCHAR(10),
  suppress  VARCHAR(1),
  cvf       INTEGER
);

CREATE TABLE rxnsab
(
  vcui    VARCHAR(8),
  rcui    VARCHAR(8),
  vsab    VARCHAR(40),
  rsab    VARCHAR(20) NOT NULL,
  son     VARCHAR(3000),
  sf      VARCHAR(20),
  sver    VARCHAR(20),
  vstart  DATE,
  vend    DATE,
  imeta   VARCHAR(10),
  rmeta   VARCHAR(10),
  slc     VARCHAR(1000),
  scc     VARCHAR(1000),
  srl     INTEGER,
  tfr     INTEGER,
  cfr     INTEGER,
  cxty    VARCHAR(50),
  ttyl    VARCHAR(300),
  atnl    VARCHAR(1000),
  lat     VARCHAR(3),
  cenc    VARCHAR(20),
  curver  VARCHAR(1),
  sabin   VARCHAR(1),
  ssn     VARCHAR(3000),
  scit    VARCHAR(4000)
)
;

CREATE TABLE rxnsat
(
  rxcui     INTEGER,
  lui       VARCHAR(8),
  sui       VARCHAR(8),
  rxaui     INTEGER,
  stype     VARCHAR(50),
  code      VARCHAR(50),
  atui      VARCHAR(11),
  satui     VARCHAR(50),
  atn       VARCHAR(1000) NOT NULL,
  sab       VARCHAR(20) NOT NULL,
  atv       VARCHAR(4000),
  suppress  VARCHAR(1),
  cvf       INTEGER
)
;

CREATE TABLE rxnsty
(
  rxcui   INTEGER NOT NULL,
  tui     VARCHAR(4) NOT NULL,
  stn     VARCHAR(100),
  sty     VARCHAR(50),
  atui    VARCHAR(11),
  cvf     INTEGER
)
;

CREATE TABLE rxndoc (
  key     VARCHAR(50) NOT NULL,
  value   VARCHAR(1000),
  type    VARCHAR(50) NOT NULL,
  expl    VARCHAR(1000)
);

CREATE TABLE rxncuichanges
(
  rxaui       INTEGER,
  code        VARCHAR(50),
  sab         VARCHAR(20) NOT NULL,
  tty         VARCHAR(20),
  str         VARCHAR(3000),
  old_rxcui   INTEGER NOT NULL,
  new_rxcui   INTEGER NOT NULL
);

CREATE TABLE rxncui (
  cui1          INTEGER NOT NULL,
  ver_start     VARCHAR(40) NOT NULL,
  ver_end       VARCHAR(40) NOT NULL,
  cardinality   INTEGER NOT NULL,
  cui2          INTEGER NOT NULL
);
