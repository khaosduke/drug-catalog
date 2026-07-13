-- Find all terms containing Lipitor
SELECT
  c.sab || ' - ' || s.son AS source, c.code,
  c.tty || ' - ' || t.termtype AS termtype,
  c.rxaui, c.rxcui, c.str
FROM rxnconso c
INNER JOIN rxntty t ON t.tty = c.tty
INNER JOIN rxnsab s ON s.rsab = c.sab
WHERE to_tsvector('english',c.str) @@ to_tsquery('Lipitor')
ORDER BY c.sab, c.code, c.tty, c.rxaui
;

-- Number of terms for each VA Drug Class
SELECT
  s.atv AS DrugClass,
  COUNT(DISTINCT s.rxaui) AS NumAtoms,
  COUNT(DISTINCT s.rxcui) AS NumConcepts,
  COUNT(DISTINCT c.rxaui) AS NumRelatedAtoms
FROM rxnsat s
LEFT OUTER JOIN rxnconso c ON c.rxcui = s.rxcui
WHERE s.sab = 'VANDF'
  AND s.atn = 'VA_CLASS_NAME'
GROUP BY s.atv
ORDER BY s.atv
;

-- List all the terms in the Anti-Coagulants drug class
SELECT DISTINCT
  dc.atv AS DrugClass,
  c.rxcui, c.rxaui,
  c.sab || ' - ' || s.son AS source, c.code,
  c.tty || ' - ' || t.termtype AS termtype,
  c.str AS DrugName
FROM rxnsat dc
INNER JOIN rxnconso c ON c.rxcui = dc.rxcui
INNER JOIN rxntty t ON t.tty = c.tty
INNER JOIN rxnsab s ON s.rsab = c.sab
WHERE dc.sab = 'VANDF'
  AND dc.atn = 'VA_CLASS_NAME'
  AND dc.atv = '[BL110] ANTICOAGULANTS'
ORDER BY dc.atv, c.rxcui, c.rxaui
;

-- Find the active ingredient in some common OTC meds
SELECT
  fc.rxaui AS AtomID, fc.rxcui AS ConceptID, fc.str AS Drug,
  fc.tty || ' - ' || ft.termtype AS TermType,
  r.rela,
  tc.rxcui, tc.rxaui, tc.str, tc.tty || ' - ' || t.termtype AS TermType,
  r1.rela,
  tc1.rxcui, tc1.rxaui, tc1.str, tc1.tty || ' - ' || t1.termtype AS TermType
FROM rxnconso fc
INNER JOIN rxntty ft ON ft.tty = fc.tty
LEFT OUTER JOIN rxnrel r ON r.stype2 = 'CUI' AND r.rxcui2 = fc.rxcui
  AND r.sab = 'RXNORM' AND fc.tty <> 'IN'
LEFT OUTER JOIN rxnconso tc ON tc.rxcui = r.rxcui1 AND tc.sab = 'RXNORM'
LEFT OUTER JOIN rxntty t ON t.tty = tc.tty
LEFT OUTER JOIN rxnrel r1 ON r1.stype2 = 'CUI' AND r1.rxcui2 = tc.rxcui
  AND r1.sab = 'RXNORM' AND r1.rela = 'has_ingredient'
LEFT OUTER JOIN rxnconso tc1 ON tc1.rxcui = r1.rxcui1 AND tc1.sab = 'RXNORM'
LEFT OUTER JOIN rxntty t1 ON t1.tty = tc1.tty
WHERE (fc.tty = 'IN' OR tc.tty = 'IN' OR tc1.tty = 'IN')
  AND fc.rxaui IN (
    3516232, 5531391, 1478331, 179266,    -- Motrin / Ibuprofen
    3090103, 6364043, 1476962, 1832382,   -- Tylenol / Acetaminophen
    2931863, 6365775, 1477237, 38593      -- Bayer / Aspirin
  )
ORDER BY fc.rxaui DESC
;
