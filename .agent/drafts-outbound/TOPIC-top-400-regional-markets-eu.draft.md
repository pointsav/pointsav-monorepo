---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC
audience: general
bcsc_class: no-disclosure-implication
version: "2.0"
date: 2026-05-30
title: "Top 400 Regional Markets — Europe"
methodology_version: suburban-regional-v2
routes_to: project-editorial
---

# Top 400 Regional Markets — Europe

This is a ranked list of the four hundred highest-scoring Regional Markets in Europe, where a *Regional Market* is a named suburb or satellite municipality that lies within commuting distance of a major European metropolitan centre and contains one or more retail co-location clusters. Markets are ranked by a composite score that combines the tier composition of their co-location clusters and the presence of civic-anchor infrastructure (medical and academic).

The list identifies commercial markets in the suburban ring of major European cities — the research gap between institutional metro-market analysis and genuinely rural areas. It is produced separately from lists of metro cores (covered extensively by institutional research) and standalone secondary cities.

## Definition and Scope

A European Regional Market qualifies for this list when it meets three criteria. First, the settlement must contain at least one co-location cluster. Second, the settlement must be a named municipality whose name differs from that of its nearest major metropolitan reference centre. Third, the centroid of the settlement's clusters must lie between 15 and 80 kilometres from the nearest major European metropolitan reference point; markets closer than 15 kilometres are classified as metro-core areas and markets beyond 80 kilometres are classified as standalone secondary cities — both excluded from this list.

The geographic coherence constraint requires that all co-location clusters within a single named settlement lie within a 200-kilometre bounding box. Settlements failing this constraint are name-collision aggregations and are excluded.

## Ranking Methodology

The composite score is the product of three factors. *Tier score* counts four points per Tier 1 cluster, two points per Tier 2 cluster, and one point per Tier 3 cluster. *Civic multiplier* is 1.5 when at least one cluster includes a medical or academic anchor and 1.0 otherwise. *Confidence factor* is 1.0 for high-confidence markets and 0.7 for provisional markets.

The full formula is: *Score = tier_score × civic_multiplier × confidence_factor*.

The three tiers describe progressively richer combinations of European retail anchor categories. *Tier 1* (T1) clusters contain a hypermarket anchor (Kaufland, Globus, Carrefour, Auchan, or equivalent) alongside a hardware anchor (Hornbach, Bauhaus, Leroy Merlin, or equivalent) and at minimum one additional anchor category (electronics, warehouse club, or sporting goods). *Tier 2* (T2) clusters contain a hypermarket and a hardware anchor. *Tier 3* (T3) clusters contain a single recognised anchor.

The highest composite score in the current European dataset is 18.0 (Chemnitz, a suburb of Dresden). The minimum score among the ranked 400 is 3.0. One market scores 15 or above, six score 12 or above, and the long tail of the top 400 sits predominantly between 3.0 and 9.0.

## European Retail Profile

European T1 cluster compositions differ meaningfully from North American ones. In German and Central European markets, the T1 triad is typically anchored by Kaufland or Globus (hypermarket), Hornbach or Bauhaus (hardware), and MediaMarkt or Saturn (electronics). In French and Belgian markets, the hypermarket role is filled by Carrefour, Auchan, or Leclerc, with Leroy Merlin or Brico Dépôt as hardware anchor. In the United Kingdom, Tesco Extra or Sainsbury's Superstore anchors the hypermarket position, with Wickes or B&Q as hardware anchor. Spanish and Italian markets are represented at lower density due to thinner OSM retail-anchor coverage.

## Suburban Geography

The 400 markets span 11 European countries. Germany leads with 124 markets, reflecting the polycentric character of the German urban system and dense suburban retail development in the Rhine-Ruhr, Rhine-Neckar, and Saxon-Thuringian corridors. France contributes 102 markets, the United Kingdom 81, Spain 23, Italy 21, Poland 16, and the Netherlands 14.

The metropolitan areas generating the most qualifying suburbs are London (14), Paris (14), Stuttgart (12), and Le Havre (9). The Rhine-Ruhr system — encompassing Düsseldorf, Essen, Cologne, and Dortmund — collectively generates over 30 qualifying suburbs across multiple reference metro centroids.

## Top 25

The twenty-five highest-ranked European Regional Markets are listed below, with the metropolitan centre each market is a suburb of.

| Rank | Market | Country | Suburb of | km | T1 | T2 | T3 | Civic | Score |
|------|--------|---------|-----------|----|----|----|----|-------|-------|
| 1 | Chemnitz, Stadt | DE | Dresden | 64 | 3 | 0 | 0 | Yes | 18.0 |
| 2 | Ingolstadt | DE | Regensburg | 56 | 2 | 0 | 1 | Yes | 13.5 |
| 3 | Dessau-Roßlau, Stadt | DE | Halle | 44 | 2 | 0 | 0 | Yes | 12.0 |
| 4 | Kaiserslautern, Stadt | DE | Mannheim | 54 | 2 | 0 | 0 | Yes | 12.0 |
| 5 | Krefeld, Stadt | DE | Düsseldorf | 19 | 2 | 0 | 0 | Yes | 12.0 |
| 6 | Bielsko-Biała | PL | Katowice | 48 | 2 | 0 | 0 | Yes | 12.0 |
| 7 | Zwickau, Stadt | DE | Leipzig | 69 | 1 | 1 | 1 | Yes | 10.5 |
| 8 | Radom | PL | Kielce | 69 | 1 | 1 | 1 | Yes | 10.5 |
| 9 | Fife | GB | Edinburgh | 19 | 0 | 3 | 1 | Yes | 10.5 |
| 10 | MiltonKeynes | GB | Luton | 27 | 1 | 0 | 2 | Yes | 9.0 |
| 11 | Duisburg, Stadt | DE | Essen | 17 | 1 | 0 | 2 | Yes | 9.0 |
| 12 | Bremerhaven, Stadt | DE | Bremen | 54 | 1 | 0 | 2 | Yes | 9.0 |
| 13 | Murcia | ES | Alicante | 72 | 1 | 0 | 2 | Yes | 9.0 |
| 14 | Hertsmere | GB | London | 22 | 1 | 1 | 0 | Yes | 9.0 |
| 15 | Auch | FR | Toulouse | 69 | 1 | 1 | 0 | Yes | 9.0 |
| 16 | Beauvais | FR | Amiens | 54 | 1 | 1 | 0 | Yes | 9.0 |
| 17 | Jena, Stadt | DE | Halle | 68 | 1 | 1 | 0 | Yes | 9.0 |
| 18 | Heilbronn, Universitätsstadt | DE | Stuttgart | 40 | 1 | 1 | 0 | Yes | 9.0 |
| 19 | Brandenburg an der Havel, Stadt | DE | Berlin | 57 | 1 | 1 | 0 | Yes | 9.0 |
| 20 | Heidelberg, Stadt | DE | Mannheim | 19 | 1 | 1 | 0 | Yes | 9.0 |
| 21 | Worms, Stadt | DE | Mannheim | 19 | 1 | 1 | 0 | Yes | 9.0 |
| 22 | Stade, Hansestadt | DE | Hamburg | 37 | 1 | 1 | 0 | Yes | 9.0 |
| 23 | Villingen-Schwenningen, Stadt | DE | Freiburg | 48 | 1 | 1 | 0 | Yes | 9.0 |
| 24 | Asti | IT | Turin | 45 | 1 | 1 | 0 | Yes | 9.0 |
| 25 | Toruń | PL | Bydgoszcz | 45 | 1 | 1 | 0 | Yes | 9.0 |

## Full Rankings (26–400)

| Rank | Market | Country | Suburb of | T1 | T2 | T3 | Score |
|------|--------|---------|-----------|----|----|----|-------|
| 26 | Częstochowa | PL | Katowice | 1 | 1 | 0 | 9.0 |
| 27 | Wiltshire | GB | Bath | 0 | 2 | 2 | 9.0 |
| 28 | CheshireEast | GB | Manchester | 0 | 3 | 0 | 9.0 |
| 29 | Randers | DK | Aarhus | 0 | 3 | 0 | 9.0 |
| 30 | Hillingdon | GB | London | 1 | 0 | 1 | 7.5 |
| 31 | Derby | GB | Nottingham | 1 | 0 | 1 | 7.5 |
| 32 | CheshireWestandChester | GB | Wigan | 1 | 0 | 1 | 7.5 |
| 33 | Chambéry | FR | Grenoble | 1 | 0 | 1 | 7.5 |
| 34 | Albi | FR | Toulouse | 1 | 0 | 1 | 7.5 |
| 35 | Cambrai | FR | Valenciennes | 1 | 0 | 1 | 7.5 |
| 36 | Osny | FR | Paris | 1 | 0 | 1 | 7.5 |
| 37 | Montauban | FR | Toulouse | 1 | 0 | 1 | 7.5 |
| 38 | Quimper | FR | Brest | 1 | 0 | 1 | 7.5 |
| 39 | Mulhouse | FR | Colmar | 1 | 0 | 1 | 7.5 |
| 40 | Rosenheim | DE | Munich | 1 | 0 | 1 | 7.5 |
| 41 | Bamberg | DE | Nürnberg | 1 | 0 | 1 | 7.5 |
| 42 | Neubrandenburg, Stadt | DE | Cottbus | 1 | 0 | 1 | 7.5 |
| 43 | Potsdam, Stadt | DE | Berlin | 1 | 0 | 1 | 7.5 |
| 44 | Wuppertal, Stadt | DE | Essen | 1 | 0 | 1 | 7.5 |
| 45 | Landshut | DE | Regensburg | 1 | 0 | 1 | 7.5 |
| 46 | Paderborn, Stadt | DE | Bielefeld | 1 | 0 | 1 | 7.5 |
| 47 | Hanau, Brüder-Grimm-Stadt | DE | Frankfurt | 1 | 0 | 1 | 7.5 |
| 48 | Limburg a.d. Lahn, Kreisstadt | DE | Wiesbaden | 1 | 0 | 1 | 7.5 |
| 49 | Erlangen | DE | Nürnberg | 1 | 0 | 1 | 7.5 |
| 50 | Siegen, Universitätsstadt | DE | Cologne | 1 | 0 | 1 | 7.5 |
| 51 | Reutlingen, Stadt | DE | Stuttgart | 1 | 0 | 1 | 7.5 |
| 52 | Crailsheim, Stadt | DE | Stuttgart | 1 | 0 | 1 | 7.5 |
| 53 | Gijón | ES | Oviedo | 1 | 0 | 1 | 7.5 |
| 54 | Durham | GB | Newcastle | 0 | 2 | 1 | 7.5 |
| 55 | EastRidingofYorkshire | GB | Leeds | 0 | 2 | 1 | 7.5 |
| 56 | 's-Gravenhage | NL | Rotterdam | 1 | 1 | 0 | 6.0 |
| 57 | Thurrock | GB | London | 1 | 0 | 0 | 6.0 |
| 58 | Gloucester | GB | Bristol | 1 | 0 | 0 | 6.0 |
| 59 | Rushmoor | GB | Reading | 1 | 0 | 0 | 6.0 |
| 60 | Järfälla | SE | Stockholm | 1 | 0 | 0 | 6.0 |
| 61 | Grande-Synthe | FR | Lille | 1 | 0 | 0 | 6.0 |
| 62 | Roissy-en-France | FR | Paris | 1 | 0 | 0 | 6.0 |
| 63 | Dole | FR | Dijon | 1 | 0 | 0 | 6.0 |
| 64 | Montigny-lès-Cormeilles | FR | Paris | 1 | 0 | 0 | 6.0 |
| 65 | Sartrouville | FR | Paris | 1 | 0 | 0 | 6.0 |
| 66 | Sainte-Geneviève-des-Bois | FR | Paris | 1 | 0 | 0 | 6.0 |
| 67 | Bayeux | FR | Le Havre | 1 | 0 | 0 | 6.0 |
| 68 | Saint-Berthevin | FR | Rennes | 1 | 0 | 0 | 6.0 |
| 69 | Arras | FR | Lille | 1 | 0 | 0 | 6.0 |
| 70 | Andelnans | FR | Colmar | 1 | 0 | 0 | 6.0 |
| 71 | Guérande | FR | Nantes | 1 | 0 | 0 | 6.0 |
| 72 | Béziers | FR | Montpellier | 1 | 0 | 0 | 6.0 |
| 73 | Ifs | FR | Le Havre | 1 | 0 | 0 | 6.0 |
| 74 | Rouen | FR | Le Havre | 1 | 0 | 0 | 6.0 |
| 75 | Angers | FR | Le Mans | 1 | 0 | 0 | 6.0 |
| 76 | Venette | FR | Amiens | 1 | 0 | 0 | 6.0 |
| 77 | Mably | FR | Lyon | 1 | 0 | 0 | 6.0 |
| 78 | Trignac | FR | Nantes | 1 | 0 | 0 | 6.0 |
| 79 | Estancarbon | FR | Toulouse | 1 | 0 | 0 | 6.0 |
| 80 | Saint-Marcel | FR | Versailles | 1 | 0 | 0 | 6.0 |
| 81 | Vendin-le-Vieil | FR | Lille | 1 | 0 | 0 | 6.0 |
| 82 | Bourgoin-Jallieu | FR | Lyon | 1 | 0 | 0 | 6.0 |
| 83 | Yvetot | FR | Le Havre | 1 | 0 | 0 | 6.0 |
| 84 | Douai | FR | Lille | 1 | 0 | 0 | 6.0 |
| 85 | Louvroil | FR | Valenciennes | 1 | 0 | 0 | 6.0 |
| 86 | Castelnaudary | FR | Toulouse | 1 | 0 | 0 | 6.0 |
| 87 | Vitrolles | FR | Marseille | 1 | 0 | 0 | 6.0 |
| 88 | Lécousse | FR | Rennes | 1 | 0 | 0 | 6.0 |
| 89 | Chalon-sur-Saône | FR | Dijon | 1 | 0 | 0 | 6.0 |
| 90 | Redon | FR | Rennes | 1 | 0 | 0 | 6.0 |
| 91 | Villabé | FR | Paris | 1 | 0 | 0 | 6.0 |
| 92 | Ploërmel | FR | Rennes | 1 | 0 | 0 | 6.0 |
| 93 | Wittenheim | FR | Colmar | 1 | 0 | 0 | 6.0 |
| 94 | Châteaubriant | FR | Rennes | 1 | 0 | 0 | 6.0 |
| 95 | Vitré | FR | Rennes | 1 | 0 | 0 | 6.0 |
| 96 | Antibes | FR | Nice | 1 | 0 | 0 | 6.0 |
| 97 | Dreux | FR | Versailles | 1 | 0 | 0 | 6.0 |
| 98 | Chartres | FR | Versailles | 1 | 0 | 0 | 6.0 |
| 99 | Chennevières-sur-Marne | FR | Paris | 1 | 0 | 0 | 6.0 |
| 100 | Avignon | FR | Nîmes | 1 | 0 | 0 | 6.0 |
| 101 | Villefranche-sur-Saône | FR | Lyon | 1 | 0 | 0 | 6.0 |
| 102 | Chalezeule | FR | Dijon | 1 | 0 | 0 | 6.0 |
| 103 | Bruay-la-Buissière | FR | Lille | 1 | 0 | 0 | 6.0 |
| 104 | Le Puy-en-Velay | FR | Saint-Étienne | 1 | 0 | 0 | 6.0 |
| 105 | Givors | FR | Lyon | 1 | 0 | 0 | 6.0 |
| 106 | Lognes | FR | Paris | 1 | 0 | 0 | 6.0 |
| 107 | Berck | FR | Amiens | 1 | 0 | 0 | 6.0 |
| 108 | Saint-Pierre-des-Corps | FR | Le Mans | 1 | 0 | 0 | 6.0 |
| 109 | Villebon-sur-Yvette | FR | Paris | 1 | 0 | 0 | 6.0 |
| 110 | Barentin | FR | Le Havre | 1 | 0 | 0 | 6.0 |
| 111 | Châlons-en-Champagne | FR | Reims | 1 | 0 | 0 | 6.0 |
| 112 | Puget-sur-Argens | FR | Nice | 1 | 0 | 0 | 6.0 |
| 113 | Mondeville | FR | Le Havre | 1 | 0 | 0 | 6.0 |
| 114 | Alès | FR | Nîmes | 1 | 0 | 0 | 6.0 |
| 115 | Soissons | FR | Reims | 1 | 0 | 0 | 6.0 |
| 116 | Saint-Martin-des-Champs | FR | Brest | 1 | 0 | 0 | 6.0 |
| 117 | Tourville-la-Rivière | FR | Le Havre | 1 | 0 | 0 | 6.0 |
| 118 | Fayet | FR | Valenciennes | 1 | 0 | 0 | 6.0 |
| 119 | Arques | FR | Lille | 1 | 0 | 0 | 6.0 |
| 120 | Haguenau | FR | Strasbourg | 1 | 0 | 0 | 6.0 |
| 121 | Saint-Maximin | FR | Paris | 1 | 0 | 0 | 6.0 |
| 122 | Rots | FR | Le Havre | 1 | 0 | 0 | 6.0 |
| 123 | Tours | FR | Le Mans | 1 | 0 | 0 | 6.0 |
| 124 | Cesson | FR | Paris | 1 | 0 | 0 | 6.0 |
| 125 | Chauconin-Neufmontiers | FR | Paris | 1 | 0 | 0 | 6.0 |
| 126 | Buchelay | FR | Versailles | 1 | 0 | 0 | 6.0 |
| 127 | Biganos | FR | Bordeaux | 1 | 0 | 0 | 6.0 |
| 128 | Béthune | FR | Lille | 1 | 0 | 0 | 6.0 |
| 129 | Noyelles-Godault | FR | Lille | 1 | 0 | 0 | 6.0 |
| 130 | Schweighouse-sur-Moder | FR | Strasbourg | 1 | 0 | 0 | 6.0 |
| 131 | Arles | FR | Nîmes | 1 | 0 | 0 | 6.0 |
| 132 | Sin-le-Noble | FR | Valenciennes | 1 | 0 | 0 | 6.0 |
| 133 | Saint-André-de-Cubzac | FR | Bordeaux | 1 | 0 | 0 | 6.0 |
| 134 | Aubagne | FR | Marseille | 1 | 0 | 0 | 6.0 |
| 135 | Abbeville | FR | Amiens | 1 | 0 | 0 | 6.0 |
| 136 | Davézieux | FR | Saint-Étienne | 1 | 0 | 0 | 6.0 |
| 137 | Crépy-en-Valois | FR | Paris | 1 | 0 | 0 | 6.0 |
| 138 | Pont-Audemer | FR | Le Havre | 1 | 0 | 0 | 6.0 |
| 139 | Albert | FR | Amiens | 1 | 0 | 0 | 6.0 |
| 140 | Besançon | FR | Dijon | 1 | 0 | 0 | 6.0 |
| 141 | Chasse-sur-Rhône | FR | Lyon | 1 | 0 | 0 | 6.0 |
| 142 | Saint-Dié-des-Vosges | FR | Strasbourg | 1 | 0 | 0 | 6.0 |
| 143 | Arçonnay | FR | Le Mans | 1 | 0 | 0 | 6.0 |
| 144 | Wetzlar, Stadt | DE | Frankfurt | 1 | 0 | 0 | 6.0 |
| 145 | Albstadt, Stadt | DE | Ulm | 1 | 0 | 0 | 6.0 |
| 146 | Nördlingen, GKSt | DE | Augsburg | 1 | 0 | 0 | 6.0 |
| 147 | Pforzheim, Stadt | DE | Karlsruhe | 1 | 0 | 0 | 6.0 |
| 148 | Hameln, Stadt | DE | Hannover | 1 | 0 | 0 | 6.0 |
| 149 | Neckarsulm, Stadt | DE | Stuttgart | 1 | 0 | 0 | 6.0 |
| 150 | Kamen, Stadt | DE | Dortmund | 1 | 0 | 0 | 6.0 |
| 151 | Baden-Baden, Stadt | DE | Karlsruhe | 1 | 0 | 0 | 6.0 |
| 152 | Neuwied, Stadt | DE | Cologne | 1 | 0 | 0 | 6.0 |
| 153 | Konstanz, Universitätsstadt | DE | Ulm | 1 | 0 | 0 | 6.0 |
| 154 | Passau | DE | Linz | 1 | 0 | 0 | 6.0 |
| 155 | Riesa, Stadt | DE | Dresden | 1 | 0 | 0 | 6.0 |
| 156 | Nagold, Stadt | DE | Stuttgart | 1 | 0 | 0 | 6.0 |
| 157 | Landsberg am Lech, GKSt | DE | Augsburg | 1 | 0 | 0 | 6.0 |
| 158 | Hildesheim, Stadt | DE | Hannover | 1 | 0 | 0 | 6.0 |
| 159 | Halberstadt, Stadt | DE | Magdeburg | 1 | 0 | 0 | 6.0 |
| 160 | Speyer, Stadt | DE | Mannheim | 1 | 0 | 0 | 6.0 |
| 161 | Fredersdorf-Vogelsdorf | DE | Berlin | 1 | 0 | 0 | 6.0 |
| 162 | Erding, GKSt | DE | Munich | 1 | 0 | 0 | 6.0 |
| 163 | Gießen, Universitätsstadt | DE | Frankfurt | 1 | 0 | 0 | 6.0 |
| 164 | Remscheid, Stadt | DE | Düsseldorf | 1 | 0 | 0 | 6.0 |
| 165 | Sindelfingen, Stadt | DE | Stuttgart | 1 | 0 | 0 | 6.0 |
| 166 | Königs Wusterhausen, Stadt | DE | Berlin | 1 | 0 | 0 | 6.0 |
| 167 | Freising, GKSt | DE | Munich | 1 | 0 | 0 | 6.0 |
| 168 | Sankt Augustin, Stadt | DE | Cologne | 1 | 0 | 0 | 6.0 |
| 169 | Goslar, Stadt | DE | Braunschweig | 1 | 0 | 0 | 6.0 |
| 170 | Nürtingen, Stadt | DE | Stuttgart | 1 | 0 | 0 | 6.0 |
| 171 | Bornheim, Stadt | DE | Cologne | 1 | 0 | 0 | 6.0 |
| 172 | Essingen | DE | Stuttgart | 1 | 0 | 0 | 6.0 |
| 173 | Leer (Ostfriesland), Stadt | DE | Groningen | 1 | 0 | 0 | 6.0 |
| 174 | Schwedt/Oder, Stadt | DE | Cottbus | 1 | 0 | 0 | 6.0 |
| 175 | Offenburg, Stadt | DE | Strasbourg | 1 | 0 | 0 | 6.0 |
| 176 | Bad Nauheim, Stadt | DE | Frankfurt | 1 | 0 | 0 | 6.0 |
| 177 | Traunstein, GKSt | DE | Salzburg | 1 | 0 | 0 | 6.0 |
| 178 | Alzey, Stadt | DE | Wiesbaden | 1 | 0 | 0 | 6.0 |
| 179 | Donauwörth, GKSt | DE | Augsburg | 1 | 0 | 0 | 6.0 |
| 180 | Kulmbach, GKSt | DE | Nürnberg | 1 | 0 | 0 | 6.0 |
| 181 | Schwentinental, Stadt | DE | Lübeck | 1 | 0 | 0 | 6.0 |
| 182 | Koblenz, Stadt | DE | Wiesbaden | 1 | 0 | 0 | 6.0 |
| 183 | Friedrichshafen, Stadt | DE | Ulm | 1 | 0 | 0 | 6.0 |
| 184 | Pirmasens, Stadt | DE | Karlsruhe | 1 | 0 | 0 | 6.0 |
| 185 | Neumarkt i.d.OPf., GKSt | DE | Nürnberg | 1 | 0 | 0 | 6.0 |
| 186 | Düren, Stadt | DE | Aachen | 1 | 0 | 0 | 6.0 |
| 187 | Lüneburg, Hansestadt | DE | Hamburg | 1 | 0 | 0 | 6.0 |
| 188 | Mönchengladbach, Stadt | DE | Düsseldorf | 1 | 0 | 0 | 6.0 |
| 189 | Straubing | DE | Regensburg | 1 | 0 | 0 | 6.0 |
| 190 | Lippstadt, Stadt | DE | Bielefeld | 1 | 0 | 0 | 6.0 |
| 191 | Lahr/Schwarzwald, Stadt | DE | Strasbourg | 1 | 0 | 0 | 6.0 |
| 192 | Kleve, Stadt | DE | Nijmegen | 1 | 0 | 0 | 6.0 |
| 193 | Amberg | DE | Regensburg | 1 | 0 | 0 | 6.0 |
| 194 | Neu-Ulm, GKSt | DE | Ulm | 1 | 0 | 0 | 6.0 |
| 195 | Schorndorf, Stadt | DE | Stuttgart | 1 | 0 | 0 | 6.0 |
| 196 | Schwäbisch Gmünd, Stadt | DE | Stuttgart | 1 | 0 | 0 | 6.0 |
| 197 | Weilheim i.OB, St | DE | Munich | 1 | 0 | 0 | 6.0 |
| 198 | Weimar, Stadt | DE | Halle | 1 | 0 | 0 | 6.0 |
| 199 | Hamm, Stadt | DE | Dortmund | 1 | 0 | 0 | 6.0 |
| 200 | Bad Kreuznach, Stadt | DE | Wiesbaden | 1 | 0 | 0 | 6.0 |
| 201 | Gifhorn, Stadt | DE | Braunschweig | 1 | 0 | 0 | 6.0 |
| 202 | Weiden i.d.OPf. | DE | Regensburg | 1 | 0 | 0 | 6.0 |
| 203 | Bocholt, Stadt | DE | Essen | 1 | 0 | 0 | 6.0 |
| 204 | Nienburg (Weser), Stadt | DE | Hannover | 1 | 0 | 0 | 6.0 |
| 205 | Schwäbisch Hall, Stadt | DE | Stuttgart | 1 | 0 | 0 | 6.0 |
| 206 | Papenburg, Stadt | DE | Groningen | 1 | 0 | 0 | 6.0 |
| 207 | Memmingen | DE | Ulm | 1 | 0 | 0 | 6.0 |
| 208 | Sinsheim, Stadt | DE | Mannheim | 1 | 0 | 0 | 6.0 |
| 209 | Weiterstadt, Stadt | DE | Darmstadt | 1 | 0 | 0 | 6.0 |
| 210 | Buxtehude, Hansestadt | DE | Hamburg | 1 | 0 | 0 | 6.0 |
| 211 | Belm | DE | Bielefeld | 1 | 0 | 0 | 6.0 |
| 212 | Weinheim, Stadt | DE | Mannheim | 1 | 0 | 0 | 6.0 |
| 213 | Forchheim, GKSt | DE | Nürnberg | 1 | 0 | 0 | 6.0 |
| 214 | Kiel, Landeshauptstadt | DE | Lübeck | 1 | 0 | 0 | 6.0 |
| 215 | Neustadt an der Weinstraße, Stadt | DE | Mannheim | 1 | 0 | 0 | 6.0 |
| 216 | Bad Oeynhausen, Stadt | DE | Bielefeld | 1 | 0 | 0 | 6.0 |
| 217 | San Sebastián de los Reyes | ES | Madrid | 1 | 0 | 0 | 6.0 |
| 218 | Castelló de la Plana | ES | Valencia | 1 | 0 | 0 | 6.0 |
| 219 | Rozas de Madrid, Las | ES | Madrid | 1 | 0 | 0 | 6.0 |
| 220 | Gavà | ES | Barcelona | 1 | 0 | 0 | 6.0 |
| 221 | Badia del Vallès | ES | Barcelona | 1 | 0 | 0 | 6.0 |
| 222 | Elche | ES | Alicante | 1 | 0 | 0 | 6.0 |
| 223 | Rivas-Vaciamadrid | ES | Madrid | 1 | 0 | 0 | 6.0 |
| 224 | Parets del Vallès | ES | Barcelona | 1 | 0 | 0 | 6.0 |
| 225 | Alcalá de Henares | ES | Madrid | 1 | 0 | 0 | 6.0 |
| 226 | Getafe | ES | Madrid | 1 | 0 | 0 | 6.0 |
| 227 | Ferrol | ES | A Coruña | 1 | 0 | 0 | 6.0 |
| 228 | Talavera de la Reina | ES | Toledo | 1 | 0 | 0 | 6.0 |
| 229 | Orihuela | ES | Alicante | 1 | 0 | 0 | 6.0 |
| 230 | Jaén | ES | Granada | 1 | 0 | 0 | 6.0 |
| 231 | Finestrat | ES | Alicante | 1 | 0 | 0 | 6.0 |
| 232 | Xàtiva | ES | Valencia | 1 | 0 | 0 | 6.0 |
| 233 | La Spezia, Liguria | IT | Genoa | 1 | 0 | 0 | 6.0 |
| 234 | Castelletto sopra Ticino | IT | Milan | 1 | 0 | 0 | 6.0 |
| 235 | Lecco | IT | Milan | 1 | 0 | 0 | 6.0 |
| 236 | Brescia | IT | Verona | 1 | 0 | 0 | 6.0 |
| 237 | Cremona | IT | Milan | 1 | 0 | 0 | 6.0 |
| 238 | Curno | IT | Milan | 1 | 0 | 0 | 6.0 |
| 239 | Mantova | IT | Verona | 1 | 0 | 0 | 6.0 |
| 240 | Ravenna | IT | Bologna | 1 | 0 | 0 | 6.0 |
| 241 | Padova | IT | Venice | 1 | 0 | 0 | 6.0 |
| 242 | Milazzo | IT | Messina | 1 | 0 | 0 | 6.0 |
| 243 | Viterbo | IT | Rome | 1 | 0 | 0 | 6.0 |
| 244 | Molfetta | IT | Bari | 1 | 0 | 0 | 6.0 |
| 245 | Terni | IT | Rome | 1 | 0 | 0 | 6.0 |
| 246 | Modena, Modena | IT | Bologna | 1 | 0 | 0 | 6.0 |
| 247 | Trento | IT | Verona | 1 | 0 | 0 | 6.0 |
| 248 | Benevento | IT | Naples | 1 | 0 | 0 | 6.0 |
| 249 | Tarnów | PL | Kraków | 1 | 0 | 0 | 6.0 |
| 250 | Piotrków Trybunalski | PL | Łódź | 1 | 0 | 0 | 6.0 |
| 251 | Żory | PL | Gliwice | 1 | 0 | 0 | 6.0 |
| 252 | Piaseczno | PL | Warsaw | 1 | 0 | 0 | 6.0 |
| 253 | Tychy | PL | Katowice | 1 | 0 | 0 | 6.0 |
| 254 | Lubin | PL | Wrocław | 1 | 0 | 0 | 6.0 |
| 255 | Ostrołęka | PL | Łomża | 1 | 0 | 0 | 6.0 |
| 256 | Nowy Targ | PL | Kraków | 1 | 0 | 0 | 6.0 |
| 257 | Zell am See | AT | Salzburg | 1 | 0 | 0 | 6.0 |
| 258 | Steyr | AT | Linz | 1 | 0 | 0 | 6.0 |
| 259 | Bregenz | AT | Ulm | 1 | 0 | 0 | 6.0 |
| 260 | Maastricht | NL | Liège | 1 | 0 | 0 | 6.0 |
| 261 | Roermond | NL | Eindhoven | 1 | 0 | 0 | 6.0 |
| 262 | Santa Maria, São Pedro e Matacães | PT | Lisbon | 1 | 0 | 0 | 6.0 |
| 263 | União das freguesias de Sintra (Santa Maria e São Miguel, São Martinho e São Pedro de Penaferrim) | PT | Lisbon | 1 | 0 | 0 | 6.0 |
| 264 | União das freguesias de Palhais e Coina | PT | Lisbon | 1 | 0 | 0 | 6.0 |
| 265 | Stevenage | GB | Luton | 0 | 1 | 2 | 6.0 |
| 266 | York | GB | Leeds | 0 | 1 | 2 | 6.0 |
| 267 | Redbridge | GB | London | 0 | 2 | 0 | 6.0 |
| 268 | Wakefield | GB | Leeds | 0 | 2 | 0 | 6.0 |
| 269 | Ipswich | GB | Cambridge | 0 | 2 | 0 | 6.0 |
| 270 | EastHertfordshire | GB | Luton | 0 | 2 | 0 | 6.0 |
| 271 | NorthLincolnshire | GB | Sheffield | 0 | 2 | 0 | 6.0 |
| 272 | Herefordshire | GB | Birmingham | 0 | 2 | 0 | 6.0 |
| 273 | Carmarthenshire | GB | Swansea | 0 | 2 | 0 | 6.0 |
| 274 | Peterborough | GB | Cambridge | 0 | 2 | 0 | 6.0 |
| 275 | HighPeak | GB | Manchester | 0 | 2 | 0 | 6.0 |
| 276 | Huntingdonshire | GB | Cambridge | 0 | 2 | 0 | 6.0 |
| 277 | Darlington | GB | Newcastle | 0 | 2 | 0 | 6.0 |
| 278 | EastStaffordshire | GB | Birmingham | 0 | 2 | 0 | 6.0 |
| 279 | MidSussex | GB | Brighton | 0 | 2 | 0 | 6.0 |
| 280 | Mendip | GB | Bath | 0 | 2 | 0 | 6.0 |
| 281 | Kolding | DK | Odense | 0 | 2 | 0 | 6.0 |
| 282 | Vejle | DK | Odense | 0 | 2 | 0 | 6.0 |
| 283 | Rudersdal | DK | Copenhagen | 0 | 2 | 0 | 6.0 |
| 284 | Viersen, Stadt | DE | Düsseldorf | 0 | 2 | 0 | 6.0 |
| 285 | Auch | ES | Toulouse | 0 | 2 | 0 | 6.0 |
| 286 | Breda | NL | Rotterdam | 1 | 0 | 1 | 5.0 |
| 287 | Liverpool | GB | Wigan | 0 | 0 | 3 | 4.5 |
| 288 | Osnabrück, Stadt | DE | Bielefeld | 0 | 0 | 3 | 4.5 |
| 289 | Havering | GB | London | 0 | 1 | 1 | 4.5 |
| 290 | NA | GB | London | 0 | 1 | 1 | 4.5 |
| 291 | Sefton | GB | Wigan | 0 | 1 | 1 | 4.5 |
| 292 | Havant | GB | Southampton | 0 | 1 | 1 | 4.5 |
| 293 | Stroud | GB | Bristol | 0 | 1 | 1 | 4.5 |
| 294 | Warrington | GB | Wigan | 0 | 1 | 1 | 4.5 |
| 295 | Newport | GB | Cardiff | 0 | 1 | 1 | 4.5 |
| 296 | SaintEdmundsbury | GB | Cambridge | 0 | 1 | 1 | 4.5 |
| 297 | Colchester | GB | Cambridge | 0 | 1 | 1 | 4.5 |
| 298 | Warwick | GB | Birmingham | 0 | 1 | 1 | 4.5 |
| 299 | Lincoln | GB | Nottingham | 0 | 1 | 1 | 4.5 |
| 300 | Cheltenham | GB | Bristol | 0 | 1 | 1 | 4.5 |
| 301 | Stoke-on-Trent | GB | Manchester | 0 | 1 | 1 | 4.5 |
| 302 | Stafford | GB | Birmingham | 0 | 1 | 1 | 4.5 |
| 303 | Chelmsford | GB | London | 0 | 1 | 1 | 4.5 |
| 304 | MoleValley | GB | London | 0 | 1 | 1 | 4.5 |
| 305 | Swindon | GB | Oxford | 0 | 1 | 1 | 4.5 |
| 306 | Barnsley | GB | Sheffield | 0 | 1 | 1 | 4.5 |
| 307 | TauntonDeane | GB | Cardiff | 0 | 1 | 1 | 4.5 |
| 308 | PerthshireandKinross | GB | Dundee | 0 | 1 | 1 | 4.5 |
| 309 | SouthSomerset | GB | Bath | 0 | 1 | 1 | 4.5 |
| 310 | Doncaster | GB | Sheffield | 0 | 1 | 1 | 4.5 |
| 311 | BasingstokeandDeane | GB | Reading | 0 | 1 | 1 | 4.5 |
| 312 | Saint-Nazaire | FR | Nantes | 0 | 1 | 1 | 4.5 |
| 313 | Valence | FR | Saint-Étienne | 0 | 1 | 1 | 4.5 |
| 314 | Cholet | FR | Nantes | 0 | 1 | 1 | 4.5 |
| 315 | Saint-Malo | FR | Rennes | 0 | 1 | 1 | 4.5 |
| 316 | Martigues | FR | Marseille | 0 | 1 | 1 | 4.5 |
| 317 | Mandelieu-la-Napoule | FR | Nice | 0 | 1 | 1 | 4.5 |
| 318 | Celle, Stadt | DE | Hannover | 0 | 1 | 1 | 4.5 |
| 319 | Nordhorn, Stadt | DE | Münster | 0 | 1 | 1 | 4.5 |
| 320 | Gütersloh, Stadt | DE | Bielefeld | 0 | 1 | 1 | 4.5 |
| 321 | Heidenheim an der Brenz, Stadt | DE | Augsburg | 0 | 1 | 1 | 4.5 |
| 322 | Aschaffenburg | DE | Darmstadt | 0 | 1 | 1 | 4.5 |
| 323 | Moers, Stadt | DE | Essen | 0 | 1 | 1 | 4.5 |
| 324 | Singen (Hohentwiel), Stadt | DE | Ulm | 0 | 1 | 1 | 4.5 |
| 325 | Wilhelmshaven, Stadt | DE | Bremen | 0 | 1 | 1 | 4.5 |
| 326 | Lucca | IT | Florence | 0 | 1 | 1 | 4.5 |
| 327 | Como | IT | Milan | 0 | 1 | 1 | 4.5 |
| 328 | Gdynia | PL | Gdańsk | 0 | 1 | 1 | 4.5 |
| 329 | Nowy Sącz | PL | Kraków | 0 | 1 | 1 | 4.5 |
| 330 | Wels | AT | Linz | 0 | 1 | 1 | 4.5 |
| 331 | Zwolle | NL | Nijmegen | 0 | 1 | 1 | 4.5 |
| 332 | Täby | SE | Stockholm | 1 | 0 | 0 | 4.0 |
| 333 | Helsingborg | SE | Copenhagen | 1 | 0 | 0 | 4.0 |
| 334 | Uddevalla | SE | Gothenburg | 1 | 0 | 0 | 4.0 |
| 335 | Høje-Taastrup | DK | Copenhagen | 1 | 0 | 0 | 4.0 |
| 336 | Le Pontet | FR | Nîmes | 1 | 0 | 0 | 4.0 |
| 337 | Claye-Souilly | FR | Paris | 1 | 0 | 0 | 4.0 |
| 338 | Crêches-sur-Saône | FR | Lyon | 1 | 0 | 0 | 4.0 |
| 339 | Stadthagen, Stadt | DE | Hannover | 1 | 0 | 0 | 4.0 |
| 340 | Meerane, Stadt | DE | Leipzig | 1 | 0 | 0 | 4.0 |
| 341 | Pfungstadt, Stadt | DE | Wiesbaden | 1 | 0 | 0 | 4.0 |
| 342 | Kerpen, Kolpingstadt | DE | Cologne | 1 | 0 | 0 | 4.0 |
| 343 | Hückelhoven, Stadt | DE | Aachen | 1 | 0 | 0 | 4.0 |
| 344 | Henstedt-Ulzburg | DE | Hamburg | 1 | 0 | 0 | 4.0 |
| 345 | Wolfsburg, Stadt | DE | Braunschweig | 1 | 0 | 0 | 4.0 |
| 346 | Traunreut, St | DE | Salzburg | 1 | 0 | 0 | 4.0 |
| 347 | Fuengirola | ES | Málaga | 1 | 0 | 0 | 4.0 |
| 348 | Nigrán | ES | Braga | 1 | 0 | 0 | 4.0 |
| 349 | Villesse | IT | Trieste | 1 | 0 | 0 | 4.0 |
| 350 | Casamassima | IT | Bari | 1 | 0 | 0 | 4.0 |
| 351 | Poczesna | PL | Katowice | 1 | 0 | 0 | 4.0 |
| 352 | Rumia | PL | Gdańsk | 1 | 0 | 0 | 4.0 |
| 353 | Wiener Neustadt | AT | Vienna | 1 | 0 | 0 | 4.0 |
| 354 | St. Pölten | AT | Vienna | 1 | 0 | 0 | 4.0 |
| 355 | Horn | AT | Vienna | 1 | 0 | 0 | 4.0 |
| 356 | Tilburg | NL | Eindhoven | 1 | 0 | 0 | 4.0 |
| 357 | Leeuwarden | NL | Groningen | 1 | 0 | 0 | 4.0 |
| 358 | Zoetermeer | NL | Rotterdam | 1 | 0 | 0 | 4.0 |
| 359 | Ede | NL | Nijmegen | 1 | 0 | 0 | 4.0 |
| 360 | Haarlemmermeer | NL | Amsterdam | 1 | 0 | 0 | 4.0 |
| 361 | Alkmaar | NL | The Hague | 1 | 0 | 0 | 4.0 |
| 362 | Heerhugowaard | NL | The Hague | 1 | 0 | 0 | 4.0 |
| 363 | Dordrecht | NL | Rotterdam | 1 | 0 | 0 | 4.0 |
| 364 | Fürstenwalde/Spree, Stadt | DE | Berlin | 0 | 2 | 0 | 4.0 |
| 365 | Spelthorne | GB | London | 0 | 0 | 2 | 3.0 |
| 366 | Bexley | GB | London | 0 | 0 | 2 | 3.0 |
| 367 | Wirral | GB | Wigan | 0 | 0 | 2 | 3.0 |
| 368 | Cherwell | GB | Oxford | 0 | 0 | 2 | 3.0 |
| 369 | KingstonuponHull | GB | Leeds | 0 | 0 | 2 | 3.0 |
| 370 | Medway | GB | London | 0 | 0 | 2 | 3.0 |
| 371 | Arun | GB | Brighton | 0 | 0 | 2 | 3.0 |
| 372 | Northampton | GB | Coventry | 0 | 0 | 2 | 3.0 |
| 373 | Eastbourne | GB | Brighton | 0 | 0 | 2 | 3.0 |
| 374 | Bedford | GB | Luton | 0 | 0 | 2 | 3.0 |
| 375 | La Roche-sur-Yon | FR | Nantes | 0 | 0 | 2 | 3.0 |
| 376 | Iserlohn, Stadt | DE | Dortmund | 0 | 0 | 2 | 3.0 |
| 377 | Backnang, Stadt | DE | Stuttgart | 0 | 1 | 1 | 3.0 |
| 378 | Ravensburg, Stadt | DE | Ulm | 0 | 0 | 2 | 3.0 |
| 379 | Hagen, Stadt der FernUniversität | DE | Dortmund | 0 | 0 | 2 | 3.0 |
| 380 | Jerez de la Frontera | ES | Cádiz | 0 | 0 | 2 | 3.0 |
| 381 | Torrejón de Ardoz | ES | Madrid | 0 | 0 | 2 | 3.0 |
| 382 | Arnhem | NL | Nijmegen | 0 | 1 | 1 | 3.0 |
| 383 | Dartford | GB | London | 0 | 1 | 0 | 3.0 |
| 384 | Pembrokeshire | GB | Swansea | 0 | 1 | 0 | 3.0 |
| 385 | Wycombe | GB | Reading | 0 | 1 | 0 | 3.0 |
| 386 | WestOxfordshire | GB | Oxford | 0 | 1 | 0 | 3.0 |
| 387 | WestBerkshire | GB | Reading | 0 | 1 | 0 | 3.0 |
| 388 | Walsall | GB | Birmingham | 0 | 1 | 0 | 3.0 |
| 389 | Mansfield | GB | Nottingham | 0 | 1 | 0 | 3.0 |
| 390 | Ashfield | GB | Nottingham | 0 | 1 | 0 | 3.0 |
| 391 | TunbridgeWells | GB | Brighton | 0 | 1 | 0 | 3.0 |
| 392 | ForestHeath | GB | Cambridge | 0 | 1 | 0 | 3.0 |
| 393 | EppingForest | GB | London | 0 | 1 | 0 | 3.0 |
| 394 | Braintree | GB | Cambridge | 0 | 1 | 0 | 3.0 |
| 395 | Stratford-on-Avon | GB | Birmingham | 0 | 1 | 0 | 3.0 |
| 396 | WestLindsey | GB | Sheffield | 0 | 1 | 0 | 3.0 |
| 397 | Cotswold | GB | Bath | 0 | 1 | 0 | 3.0 |
| 398 | Sevenoaks | GB | London | 0 | 1 | 0 | 3.0 |
| 399 | EastAyrshire | GB | Glasgow | 0 | 1 | 0 | 3.0 |
| 400 | CannockChase | GB | Birmingham | 0 | 1 | 0 | 3.0 |

## Country Breakdown

| Country | Count | Notes |
|---------|-------|-------|
| Germany (DE) | 124 | Polycentric urban system; Rhine-Ruhr, Rhine-Neckar, and Saxony corridors dominant |
| France (FR) | 102 | Île-de-France and provincial city rings; Le Havre metro generates notable cluster density |
| United Kingdom (GB) | 81 | London commuter belt, Midlands, and Scottish Central Belt |
| Spain (ES) | 23 | Madrid and Barcelona suburban rings; OSM coverage thinner outside these cores |
| Italy (IT) | 21 | Po Valley suburban municipalities; Turin and Milan rings |
| Poland (PL) | 16 | Upper Silesian Industrial Region suburbs; Katowice and Łódź rings |
| Netherlands (NL) | 14 | Randstad outer suburban ring; Rotterdam and Amsterdam commuter municipalities |
| Austria (AT) | 7 | Vienna suburban ring |
| Denmark (DK) | 5 | Copenhagen and Aarhus suburban municipalities |
| Sweden (SE) | 4 | Stockholm suburban ring |
| Portugal (PT) | 3 | Lisbon suburban ring |

## Score Distribution Note

The lower maximum score in the European dataset (18.0 vs 25.5 in North America) reflects structural differences in European retail geography rather than weaker markets. European LAU-2 municipalities are significantly smaller geographic units than US incorporated places, so a European suburb typically contains fewer co-location clusters — and therefore lower tier scores — even when its underlying retail density is comparable per square kilometre. The suburban-regional definition was designed to surface genuine commuter-belt markets at the appropriate geographic resolution for each continent.

## Data Sources and Methodology

Co-location data is drawn from OpenStreetMap (ODbL licence) filtered by Wikidata chain identifiers, supplemented by civic-anchor records from the Overture Maps Foundation Places dataset (CDLA Permissive 2.0). Municipal boundaries are from Eurostat GISCO LAU 2021. Cluster boundaries are computed by a two-pass DBSCAN algorithm. Metropolitan reference centroids are drawn from approximately 160 major European metropolitan areas.

The suburban-regional classification (15–80 km from the nearest major metro centroid) is a filter applied before scoring, not a score component. Every market in this list is already in the suburban band; no distance bonus or penalty is applied within the list.