---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC
audience: general
bcsc_class: no-disclosure-implication
version: "1.0"
date: 2026-05-30
title: "Top 400 Regional Markets — Europe"
---

# Top 400 Regional Markets — Europe

This article ranks the 400 highest-scoring commercial co-location markets in Europe. The ranking combines retail tier composition, civic infrastructure (medical and academic facilities), and separation from major metropolitan centres. The dataset covers 14 European countries, drawn from a continental inventory of approximately 2,109 Regional Markets in the European subset of a broader 6,493-cluster dataset.

## Overview

A *Regional Market* is a settlement, administrative district, or contiguous urban area that contains one or more retail co-location clusters identified by spatial analysis of national and regional chain locations. Each Regional Market is scored on the strength of the retail tier composition it hosts, the presence of civic infrastructure (hospitals and universities) co-located with retail anchors, and its distance from the nearest of 60 reference European metropolitan centres. The methodology is intended to surface secondary and tertiary commercial centres whose composite profile makes them comparable in importance to the dominant metropolitan core, rather than to rank the metropolitan cores themselves.

Europe presents a polycentric urban structure that differs substantially from the more concentrated metropolitan pattern of North America. Germany alone contributes 105 markets to the top 400 — slightly more than a quarter of the list — reflecting a long-standing pattern of strong regional capitals (Stuttgart, Hannover, Dresden, Leipzig, Nürnberg) and university cities (Würzburg, Heidelberg, Freiburg, Göttingen). France contributes 95, distributed across préfectures and sous-préfectures throughout the national territory. Great Britain contributes 70, with a tier-two-dominated profile centred on metropolitan boroughs and unitary authorities. Spain (40), Poland (31), and Italy (26) round out the principal national contributions.

Two data-coverage caveats apply specifically to European markets. First, the European Union regulatory climate-zone dataset used in the underlying analysis has substantially complete coverage for Germany and France but limited or absent coverage for Spain, Greece, and Great Britain. Second, the SafeGraph mobility dataset that anchors home-to-work catchment estimates is United States–only, so European mobility figures rely on national equivalents (MITMA in Spain, INSEE in France, ONS in Great Britain) where available, and remain provisional elsewhere.

## Ranking Methodology

The composite score combines four factors:

*Tier composition.* Each Regional Market hosts zero or more co-location clusters classified by retail tier. Tier 1 markets contain a hypermarket anchor combined with two or more additional categories (typically hardware, sport, electronics, or lifestyle). Tier 2 markets contain a category anchor (commonly hardware or sport) with one or more adjacent categories. Tier 3 markets contain a single retail category in proximity to civic infrastructure. The tier contribution to the score is weighted Tier-1 > Tier-2 > Tier-3.

*Civic anchoring.* A Regional Market receives a civic multiplier when one or more clusters include a hospital, university, or major medical research facility within the co-location footprint. The multiplier reflects the empirical observation that hospital-and-retail and university-and-retail co-locations are stronger commercial anchors than retail-only compositions of equivalent tier.

*Metropolitan separation.* The score is adjusted by the distance to the nearest of 60 reference European metropolitan centres (London, Paris, Berlin, Madrid, Rome, Vienna, Warsaw, Athens, Lisbon, Stockholm, Helsinki, Copenhagen, and other major capitals and regional centres). Markets within the urban core of a major metropolitan area receive a smaller separation multiplier; markets at a distance of 100 km or more receive a larger multiplier. The intent is to identify regional centres that function as independent commercial anchors rather than as outer suburbs of a larger metropolis.

*Market confidence.* A confidence factor reflects the underlying chain inventory completeness for the country in question. Countries with comprehensive chain coverage (Germany, France, Poland, Spain) receive a confidence factor of 1.0; countries with partial coverage receive a reduced factor.

All four factors combine multiplicatively to produce the composite score reported in the ranking tables.

## Top 25

| Rank | Market | Country | T1 | T2 | T3 | Civic | Score | Nearest Metro | km |
|------|--------|---------|----|----|----|-------|-------|---------------|----|
| 1 | Nürnberg | Germany | 4 | 0 | 1 | Yes | 51.00 | Munich | 150 |
| 2 | Berlin, Stadt | Germany | 12 | 4 | 5 | Yes | 45.75 | Berlin | 2 |
| 3 | Dresden, Stadt | Germany | 2 | 2 | 2 | Yes | 42.00 | Leipzig | 103 |
| 4 | Κοινότητα Πυλαίας | Greece | 5 | 4 | 13 | Yes | 39.68 | Athens | 32 |
| 5 | Łódź | Poland | 2 | 1 | 3 | Yes | 39.00 | Warsaw | 119 |
| 6 | Gdańsk | Poland | 2 | 2 | 1 | Yes | 39.00 | Warsaw | 290 |
| 7 | Magdeburg, Landeshauptstadt | Germany | 2 | 2 | 0 | Yes | 36.00 | Leipzig | 103 |
| 8 | Bydgoszcz | Poland | 2 | 2 | 0 | Yes | 36.00 | Warsaw | 223 |
| 9 | Poznań | Poland | 2 | 2 | 0 | Yes | 36.00 | Wrocław | 143 |
| 10 | Nice | France | 3 | 0 | 0 | Yes | 36.00 | Toulon | 123 |
| 11 | Lublin | Poland | 3 | 0 | 0 | Yes | 36.00 | Warsaw | 153 |
| 12 | Tampere | Finland | 1 | 3 | 0 | Yes | 30.00 | Helsinki | 160 |
| 13 | Palma | Spain | 2 | 0 | 2 | Yes | 30.00 | Barcelona | 206 |
| 14 | Mannheim, Universitätsstadt | Germany | 3 | 0 | 2 | Yes | 29.53 | Frankfurt | 70 |
| 15 | Würzburg | Germany | 2 | 0 | 2 | Yes | 29.08 | Frankfurt | 97 |
| 16 | Hamburg, Freie und Hansestadt | Germany | 8 | 0 | 4 | Yes | 27.00 | Hamburg | 2 |
| 17 | Zaragoza | Spain | 1 | 0 | 5 | Yes | 27.00 | Nice | 188 |
| 18 | Aarhus | Denmark | 1 | 2 | 1 | Yes | 27.00 | Hamburg | 288 |
| 19 | Hannover, Landeshauptstadt | Germany | 2 | 0 | 1 | Yes | 27.00 | Hamburg | 134 |
| 20 | Szczecin | Poland | 2 | 0 | 1 | Yes | 27.00 | Berlin | 126 |
| 21 | København | Denmark | 0 | 3 | 3 | Yes | 27.00 | Hamburg | 285 |
| 22 | Bremen, Stadt | Germany | 1 | 1 | 3 | Yes | 25.85 | Hamburg | 96 |
| 23 | Bielefeld, Stadt | Germany | 1 | 1 | 3 | Yes | 24.96 | Dortmund | 92 |
| 24 | Chemnitz, Stadt | Germany | 3 | 0 | 0 | Yes | 24.00 | Leipzig | 67 |
| 25 | Braunschweig, Stadt | Germany | 1 | 2 | 0 | Yes | 24.00 | Hamburg | 151 |

The leader, Nürnberg, combines four Tier-1 clusters and one Tier-3 cluster with civic anchoring from the Friedrich-Alexander-Universität Erlangen-Nürnberg system and the Klinikum Nürnberg hospital network, and lies 149 km from Munich — sufficient distance to receive a large separation multiplier while remaining within the southern German economic axis. Berlin (rank 2) and Dresden (rank 3) are the next-highest German entries. Pyláia (rank 4) is an exceptional Greek outlier driven by an unusually dense Tier-3 composition. Łódź, Gdańsk, Bydgoszcz, Poznań, and Lublin together fill five of the top eleven positions, reflecting the strong polycentric profile of Polish regional capitals.

## Full Rankings (26–400)

| Rank | Market | Country | T1 | T2 | T3 | Score |
|------|--------|---------|----|----|----|-------|
| 26 | Göteborg | Sweden | 2 | 0 | 0 | 24.00 |
| 27 | Erfurt, Stadt | Germany | 2 | 0 | 0 | 24.00 |
| 28 | Regensburg | Germany | 2 | 0 | 0 | 24.00 |
| 29 | Leeds | Great Britain | 0 | 1 | 6 | 24.00 |
| 30 | Kaiserslautern, Stadt | Germany | 2 | 0 | 0 | 23.10 |
| 31 | Southampton | Great Britain | 1 | 1 | 1 | 21.00 |
| 32 | Salzburg | Austria | 1 | 1 | 1 | 21.00 |
| 33 | Sheffield | Great Britain | 0 | 1 | 5 | 21.00 |
| 34 | Fife | Great Britain | 0 | 3 | 1 | 21.00 |
| 35 | Paris | France | 6 | 0 | 3 | 20.25 |
| 36 | Radom | Poland | 1 | 1 | 1 | 19.72 |
| 37 | Ingolstadt | Germany | 2 | 0 | 1 | 19.14 |
| 38 | Karlsruhe, Stadt | Germany | 2 | 1 | 0 | 18.85 |
| 39 | Warszawa | Poland | 3 | 5 | 3 | 18.75 |
| 40 | Kassel, documenta-Stadt | Germany | 1 | 0 | 2 | 18.00 |
| 41 | Murcia | Spain | 1 | 0 | 2 | 18.00 |
| 42 | Salford | Great Britain | 1 | 1 | 0 | 18.00 |
| 43 | Charnwood | Great Britain | 1 | 1 | 0 | 18.00 |
| 44 | Perpignan | France | 1 | 1 | 0 | 18.00 |
| 45 | Limoges | France | 1 | 1 | 0 | 18.00 |
| 46 | Cottbus/Chóśebuz, Stadt | Germany | 1 | 1 | 0 | 18.00 |
| 47 | Toruń | Poland | 1 | 1 | 0 | 18.00 |
| 48 | Zielona Góra | Poland | 1 | 1 | 0 | 18.00 |
| 49 | Gorzów Wielkopolski | Poland | 1 | 1 | 0 | 18.00 |
| 50 | Klagenfurt am Wörthersee | Austria | 1 | 1 | 0 | 18.00 |
| 51 | Birmingham | Great Britain | 0 | 1 | 4 | 18.00 |
| 52 | Wiltshire | Great Britain | 0 | 2 | 2 | 18.00 |
| 53 | SouthGloucestershire | Great Britain | 0 | 3 | 0 | 18.00 |
| 54 | CheshireEast | Great Britain | 0 | 3 | 0 | 18.00 |
| 55 | Randers | Denmark | 0 | 3 | 0 | 18.00 |
| 56 | Esbjerg | Denmark | 0 | 3 | 0 | 18.00 |
| 57 | Carcassonne | France | 1 | 1 | 1 | 17.96 |
| 58 | Lahti | Finland | 1 | 1 | 0 | 17.70 |
| 59 | Freiburg im Breisgau, Stadt | Germany | 2 | 0 | 1 | 17.23 |
| 60 | Płock | Poland | 1 | 1 | 0 | 17.04 |
| 61 | Bremerhaven, Stadt | Germany | 1 | 0 | 2 | 16.80 |
| 62 | Derby | Great Britain | 1 | 0 | 1 | 15.00 |
| 63 | Dundee | Great Britain | 1 | 0 | 1 | 15.00 |
| 64 | CheshireWestandChester | Great Britain | 1 | 0 | 1 | 15.00 |
| 65 | Karlstad | Sweden | 1 | 0 | 1 | 15.00 |
| 66 | Odense | Denmark | 1 | 0 | 1 | 15.00 |
| 67 | Cherbourg-en-Cotentin | France | 1 | 0 | 1 | 15.00 |
| 68 | Bamberg | Germany | 1 | 0 | 1 | 15.00 |
| 69 | Neubrandenburg, Stadt | Germany | 1 | 0 | 1 | 15.00 |
| 70 | Göttingen, Stadt | Germany | 1 | 0 | 1 | 15.00 |
| 71 | Erlangen | Germany | 1 | 0 | 1 | 15.00 |
| 72 | Stuhr | Germany | 1 | 0 | 1 | 15.00 |
| 73 | Siero | Spain | 1 | 0 | 1 | 15.00 |
| 74 | Granada | Spain | 1 | 0 | 1 | 15.00 |
| 75 | Gijón | Spain | 1 | 0 | 1 | 15.00 |
| 76 | Córdoba | Spain | 1 | 0 | 1 | 15.00 |
| 77 | Roquetas de Mar | Spain | 1 | 0 | 1 | 15.00 |
| 78 | Bologna | Italy | 1 | 0 | 1 | 15.00 |
| 79 | Genova | Italy | 1 | 0 | 1 | 15.00 |
| 80 | Bari | Italy | 1 | 0 | 1 | 15.00 |
| 81 | Venezia | Italy | 1 | 0 | 1 | 15.00 |
| 82 | Słupsk | Poland | 1 | 0 | 1 | 15.00 |
| 83 | Linz | Austria | 1 | 0 | 1 | 15.00 |
| 84 | Groningen | Netherlands | 1 | 0 | 1 | 15.00 |
| 85 | Bradford | Great Britain | 0 | 1 | 3 | 15.00 |
| 86 | Coventry | Great Britain | 0 | 1 | 3 | 15.00 |
| 87 | Kirklees | Great Britain | 0 | 2 | 1 | 15.00 |
| 88 | Sunderland | Great Britain | 0 | 2 | 1 | 15.00 |
| 89 | Durham | Great Britain | 0 | 2 | 1 | 15.00 |
| 90 | Cardiff | Great Britain | 0 | 2 | 1 | 15.00 |
| 91 | EastRidingofYorkshire | Great Britain | 0 | 2 | 1 | 15.00 |
| 92 | Mulhouse | France | 1 | 0 | 1 | 14.71 |
| 93 | Zwickau, Stadt | Germany | 1 | 1 | 1 | 14.45 |
| 94 | Heidelberg, Stadt | Germany | 1 | 1 | 0 | 13.91 |
| 95 | Villingen-Schwenningen, Stadt | Germany | 1 | 1 | 0 | 13.86 |
| 96 | Calais | France | 1 | 0 | 1 | 13.83 |
| 97 | Paderborn, Stadt | Germany | 1 | 0 | 1 | 13.66 |
| 98 | Wien | Austria | 4 | 3 | 5 | 13.50 |
| 99 | Milano | Italy | 2 | 2 | 6 | 13.50 |
| 100 | München, Landeshauptstadt | Germany | 3 | 0 | 6 | 13.50 |
| 101 | Lübeck, Hansestadt | Germany | 2 | 0 | 0 | 13.39 |
| 102 | Dessau-Roßlau, Stadt | Germany | 2 | 0 | 0 | 13.17 |
| 103 | Jena, Stadt | Germany | 1 | 1 | 0 | 12.91 |
| 104 | Leipzig, Stadt | Germany | 3 | 2 | 1 | 12.75 |
| 105 | MiltonKeynes | Great Britain | 1 | 0 | 2 | 12.74 |
| 106 | Auch | France | 1 | 1 | 0 | 12.43 |
| 107 | Madrid | Spain | 1 | 0 | 12 | 12.00 |
| 108 | Köln, Stadt | Germany | 2 | 2 | 4 | 12.00 |
| 109 | NorthDown | Great Britain | 1 | 0 | 0 | 12.00 |
| 110 | Gloucester | Great Britain | 1 | 0 | 0 | 12.00 |
| 111 | Sandwell | Great Britain | 1 | 0 | 0 | 12.00 |
| 112 | Preston | Great Britain | 1 | 0 | 0 | 12.00 |
| 113 | Aberdeen | Great Britain | 1 | 0 | 0 | 12.00 |
| 114 | Sundsvall | Sweden | 1 | 0 | 0 | 12.00 |
| 115 | Umeå | Sweden | 1 | 0 | 0 | 12.00 |
| 116 | Kalmar | Sweden | 1 | 0 | 0 | 12.00 |
| 117 | Lyngby-Taarbæk | Denmark | 1 | 0 | 0 | 12.00 |
| 118 | Dury | France | 1 | 0 | 0 | 12.00 |
| 119 | Amilly | France | 1 | 0 | 0 | 12.00 |
| 120 | Andelnans | France | 1 | 0 | 0 | 12.00 |
| 121 | Auxerre | France | 1 | 0 | 0 | 12.00 |
| 122 | Aubière | France | 1 | 0 | 0 | 12.00 |
| 123 | Clermont-Ferrand | France | 1 | 0 | 0 | 12.00 |
| 124 | Saint-Doulchard | France | 1 | 0 | 0 | 12.00 |
| 125 | Trélissac | France | 1 | 0 | 0 | 12.00 |
| 126 | Onet-le-Château | France | 1 | 0 | 0 | 12.00 |
| 127 | Ville-la-Grand | France | 1 | 0 | 0 | 12.00 |
| 128 | Saintes | France | 1 | 0 | 0 | 12.00 |
| 129 | Rochefort | France | 1 | 0 | 0 | 12.00 |
| 130 | Saran | France | 1 | 0 | 0 | 12.00 |
| 131 | Châtellerault | France | 1 | 0 | 0 | 12.00 |
| 132 | Niort | France | 1 | 0 | 0 | 12.00 |
| 133 | Coutances | France | 1 | 0 | 0 | 12.00 |
| 134 | Saint-Germain-du-Puy | France | 1 | 0 | 0 | 12.00 |
| 135 | Le Boulou | France | 1 | 0 | 0 | 12.00 |
| 136 | Lorient | France | 1 | 0 | 0 | 12.00 |
| 137 | Aurillac | France | 1 | 0 | 0 | 12.00 |
| 138 | Vierzon | France | 1 | 0 | 0 | 12.00 |
| 139 | Verdun | France | 1 | 0 | 0 | 12.00 |
| 140 | Moulins-lès-Metz | France | 1 | 0 | 0 | 12.00 |
| 141 | Furiani | France | 1 | 0 | 0 | 12.00 |
| 142 | Auray | France | 1 | 0 | 0 | 12.00 |
| 143 | Thionville | France | 1 | 0 | 0 | 12.00 |
| 144 | Antibes | France | 1 | 0 | 0 | 12.00 |
| 145 | Houdemont | France | 1 | 0 | 0 | 12.00 |
| 146 | Sarlat-la-Canéda | France | 1 | 0 | 0 | 12.00 |
| 147 | Remiremont | France | 1 | 0 | 0 | 12.00 |
| 148 | Orval | France | 1 | 0 | 0 | 12.00 |
| 149 | Montluçon | France | 1 | 0 | 0 | 12.00 |
| 150 | Berck | France | 1 | 0 | 0 | 12.00 |
| 151 | Marzy | France | 1 | 0 | 0 | 12.00 |
| 152 | Épinal | France | 1 | 0 | 0 | 12.00 |
| 153 | Saint-André-les-Vergers | France | 1 | 0 | 0 | 12.00 |
| 154 | Vichy | France | 1 | 0 | 0 | 12.00 |
| 155 | Metz | France | 1 | 0 | 0 | 12.00 |
| 156 | Mauriac | France | 1 | 0 | 0 | 12.00 |
| 157 | Brive-la-Gaillarde | France | 1 | 0 | 0 | 12.00 |
| 158 | Ménétrol | France | 1 | 0 | 0 | 12.00 |
| 159 | Chasseneuil-du-Poitou | France | 1 | 0 | 0 | 12.00 |
| 160 | Poitiers | France | 1 | 0 | 0 | 12.00 |
| 161 | Porto-Vecchio | France | 1 | 0 | 0 | 12.00 |
| 162 | Abbeville | France | 1 | 0 | 0 | 12.00 |
| 163 | Puilboreau | France | 1 | 0 | 0 | 12.00 |
| 164 | Garbsen, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 165 | Hameln, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 166 | Konstanz, Universitätsstadt | Germany | 1 | 0 | 0 | 12.00 |
| 167 | Passau | Germany | 1 | 0 | 0 | 12.00 |
| 168 | Bentwisch | Germany | 1 | 0 | 0 | 12.00 |
| 169 | Eisenach, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 170 | Hildesheim, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 171 | Halberstadt, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 172 | Flensburg, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 173 | Trier, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 174 | Schwabach | Germany | 1 | 0 | 0 | 12.00 |
| 175 | Goslar, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 176 | Lambrechtshagen | Germany | 1 | 0 | 0 | 12.00 |
| 177 | Bad Neustadt a.d.Saale, St | Germany | 1 | 0 | 0 | 12.00 |
| 178 | Leer (Ostfriesland), Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 179 | Isernhagen | Germany | 1 | 0 | 0 | 12.00 |
| 180 | Kulmbach, GKSt | Germany | 1 | 0 | 0 | 12.00 |
| 181 | Friedrichshafen, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 182 | Neumarkt i.d.OPf., GKSt | Germany | 1 | 0 | 0 | 12.00 |
| 183 | Hof | Germany | 1 | 0 | 0 | 12.00 |
| 184 | Straubing | Germany | 1 | 0 | 0 | 12.00 |
| 185 | Coburg | Germany | 1 | 0 | 0 | 12.00 |
| 186 | Amberg | Germany | 1 | 0 | 0 | 12.00 |
| 187 | Gifhorn, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 188 | Weiden i.d.OPf. | Germany | 1 | 0 | 0 | 12.00 |
| 189 | Schweinfurt | Germany | 1 | 0 | 0 | 12.00 |
| 190 | Herford, Hansestadt | Germany | 1 | 0 | 0 | 12.00 |
| 191 | Nienburg (Weser), Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 192 | Papenburg, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 193 | Laatzen, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 194 | Memmingen | Germany | 1 | 0 | 0 | 12.00 |
| 195 | Marktredwitz, GKSt | Germany | 1 | 0 | 0 | 12.00 |
| 196 | Greifswald, Hansestadt | Germany | 1 | 0 | 0 | 12.00 |
| 197 | Nordhausen, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 198 | Forchheim, GKSt | Germany | 1 | 0 | 0 | 12.00 |
| 199 | Bad Oeynhausen, Stadt | Germany | 1 | 0 | 0 | 12.00 |
| 200 | Alicante | Spain | 1 | 0 | 0 | 12.00 |
| 201 | Maracena | Spain | 1 | 0 | 0 | 12.00 |
| 202 | Zaratán | Spain | 1 | 0 | 0 | 12.00 |
| 203 | Coruña, A | Spain | 1 | 0 | 0 | 12.00 |
| 204 | Cuenca | Spain | 1 | 0 | 0 | 12.00 |
| 205 | Elche | Spain | 1 | 0 | 0 | 12.00 |
| 206 | Cartagena | Spain | 1 | 0 | 0 | 12.00 |
| 207 | Málaga | Spain | 1 | 0 | 0 | 12.00 |
| 208 | Aranguren | Spain | 1 | 0 | 0 | 12.00 |
| 209 | Ferrol | Spain | 1 | 0 | 0 | 12.00 |
| 210 | Basauri | Spain | 1 | 0 | 0 | 12.00 |
| 211 | Talavera de la Reina | Spain | 1 | 0 | 0 | 12.00 |
| 212 | Ciudad Real | Spain | 1 | 0 | 0 | 12.00 |
| 213 | Badajoz | Spain | 1 | 0 | 0 | 12.00 |
| 214 | Barrios, Los | Spain | 1 | 0 | 0 | 12.00 |
| 215 | Vinaròs | Spain | 1 | 0 | 0 | 12.00 |
| 216 | Orihuela | Spain | 1 | 0 | 0 | 12.00 |
| 217 | Barakaldo | Spain | 1 | 0 | 0 | 12.00 |
| 218 | Lorca | Spain | 1 | 0 | 0 | 12.00 |
| 219 | Jaén | Spain | 1 | 0 | 0 | 12.00 |
| 220 | Finestrat | Spain | 1 | 0 | 0 | 12.00 |
| 221 | Eivissa | Spain | 1 | 0 | 0 | 12.00 |
| 222 | Tortosa | Spain | 1 | 0 | 0 | 12.00 |
| 223 | Lepe | Spain | 1 | 0 | 0 | 12.00 |
| 224 | Casalecchio di Reno | Italy | 1 | 0 | 0 | 12.00 |
| 225 | La Spezia, Liguria | Italy | 1 | 0 | 0 | 12.00 |
| 226 | Bolzano/Bozen | Italy | 1 | 0 | 0 | 12.00 |
| 227 | Mantova | Italy | 1 | 0 | 0 | 12.00 |
| 228 | Sesto Fiorentino | Italy | 1 | 0 | 0 | 12.00 |
| 229 | Ravenna | Italy | 1 | 0 | 0 | 12.00 |
| 230 | Padova | Italy | 1 | 0 | 0 | 12.00 |
| 231 | San Giovanni Teatino | Italy | 1 | 0 | 0 | 12.00 |
| 232 | Milazzo | Italy | 1 | 0 | 0 | 12.00 |
| 233 | Catanzaro | Italy | 1 | 0 | 0 | 12.00 |
| 234 | Molfetta | Italy | 1 | 0 | 0 | 12.00 |
| 235 | Surbo | Italy | 1 | 0 | 0 | 12.00 |
| 236 | Modena, Modena | Italy | 1 | 0 | 0 | 12.00 |
| 237 | Savignano sul Rubicone | Italy | 1 | 0 | 0 | 12.00 |
| 238 | Trento | Italy | 1 | 0 | 0 | 12.00 |
| 239 | Olsztyn | Poland | 1 | 0 | 0 | 12.00 |
| 240 | Kielce | Poland | 1 | 0 | 0 | 12.00 |
| 241 | Piotrków Trybunalski | Poland | 1 | 0 | 0 | 12.00 |
| 242 | Tarnowo Podgórne | Poland | 1 | 0 | 0 | 12.00 |
| 243 | Piła | Poland | 1 | 0 | 0 | 12.00 |
| 244 | Koszalin | Poland | 1 | 0 | 0 | 12.00 |
| 245 | Ostrołęka | Poland | 1 | 0 | 0 | 12.00 |
| 246 | Krosno | Poland | 1 | 0 | 0 | 12.00 |
| 247 | Seiersberg-Pirka | Austria | 1 | 0 | 0 | 12.00 |
| 248 | Zell am See | Austria | 1 | 0 | 0 | 12.00 |
| 249 | Villach | Austria | 1 | 0 | 0 | 12.00 |
| 250 | Steyr | Austria | 1 | 0 | 0 | 12.00 |
| 251 | Bregenz | Austria | 1 | 0 | 0 | 12.00 |
| 252 | União das freguesias de Perafita, Lavra e Santa Cruz do Bispo | Portugal | 1 | 0 | 0 | 12.00 |
| 253 | Glasgow | Great Britain | 0 | 1 | 2 | 12.00 |
| 254 | York | Great Britain | 0 | 1 | 2 | 12.00 |
| 255 | Wakefield | Great Britain | 0 | 2 | 0 | 12.00 |
| 256 | NorthTyneside | Great Britain | 0 | 2 | 0 | 12.00 |
| 257 | Plymouth | Great Britain | 0 | 2 | 0 | 12.00 |
| 258 | Ipswich | Great Britain | 0 | 2 | 0 | 12.00 |
| 259 | NorthLincolnshire | Great Britain | 0 | 2 | 0 | 12.00 |
| 260 | Herefordshire | Great Britain | 0 | 2 | 0 | 12.00 |
| 261 | Carmarthenshire | Great Britain | 0 | 2 | 0 | 12.00 |
| 262 | Peterborough | Great Britain | 0 | 2 | 0 | 12.00 |
| 263 | HighPeak | Great Britain | 0 | 2 | 0 | 12.00 |
| 264 | Darlington | Great Britain | 0 | 2 | 0 | 12.00 |
| 265 | EastStaffordshire | Great Britain | 0 | 2 | 0 | 12.00 |
| 266 | Mendip | Great Britain | 0 | 2 | 0 | 12.00 |
| 267 | EastLindsey | Great Britain | 0 | 2 | 0 | 12.00 |
| 268 | Kolding | Denmark | 0 | 2 | 0 | 12.00 |
| 269 | Vejle | Denmark | 0 | 2 | 0 | 12.00 |
| 270 | Rudersdal | Denmark | 0 | 2 | 0 | 12.00 |
| 271 | Jyväskylä | Finland | 0 | 2 | 0 | 12.00 |
| 272 | Rostock, Hansestadt | Germany | 0 | 2 | 0 | 12.00 |
| 273 | Rzeszów | Poland | 0 | 2 | 0 | 12.00 |
| 274 | Saint-Martin-Boulogne | France | 1 | 0 | 0 | 11.91 |
| 275 | Belm | Germany | 1 | 0 | 0 | 11.83 |
| 276 | Beauvais | France | 1 | 1 | 0 | 11.80 |
| 277 | Saint-Dizier | France | 1 | 0 | 0 | 11.75 |
| 278 | Vannes | France | 1 | 0 | 0 | 11.66 |
| 279 | Innsbruck | Austria | 1 | 0 | 0 | 11.61 |
| 280 | Nördlingen, GKSt | Germany | 1 | 0 | 0 | 11.49 |
| 281 | Pontivy | France | 1 | 0 | 0 | 11.41 |
| 282 | Bielsko-Biała | Poland | 2 | 0 | 0 | 11.40 |
| 283 | Amiens | France | 1 | 0 | 0 | 11.40 |
| 284 | Crailsheim, Stadt | Germany | 1 | 0 | 1 | 11.37 |
| 285 | Anglet | France | 1 | 0 | 0 | 11.35 |
| 286 | Royan | France | 1 | 0 | 0 | 11.26 |
| 287 | Roma | Italy | 2 | 1 | 5 | 11.25 |
| 288 | Katowice | Poland | 3 | 1 | 1 | 11.25 |
| 289 | Częstochowa | Poland | 1 | 1 | 0 | 11.25 |
| 290 | Siegen, Universitätsstadt | Germany | 1 | 0 | 1 | 11.24 |
| 291 | Plauen, Stadt | Germany | 0 | 2 | 0 | 11.21 |
| 292 | Forbach | France | 1 | 0 | 0 | 11.08 |
| 293 | Champagnole | France | 1 | 0 | 0 | 11.05 |
| 294 | Wittenheim | France | 1 | 0 | 0 | 11.04 |
| 295 | Tarnos | France | 1 | 0 | 0 | 11.03 |
| 296 | Boé | France | 1 | 0 | 0 | 10.97 |
| 297 | Aachen, Stadt | Germany | 0 | 2 | 2 | 10.92 |
| 298 | Heide, Stadt | Germany | 1 | 0 | 0 | 10.89 |
| 299 | Ludwigshafen am Rhein, Stadt | Germany | 1 | 0 | 1 | 10.88 |
| 300 | Neustadt an der Weinstraße, Stadt | Germany | 1 | 0 | 0 | 10.84 |
| 301 | Głogów | Poland | 1 | 0 | 0 | 10.75 |
| 302 | Saarbrücken, Landeshauptstadt | Germany | 1 | 0 | 0 | 10.72 |
| 303 | Trégueux | France | 1 | 0 | 0 | 10.67 |
| 304 | Quimperlé | France | 1 | 0 | 0 | 10.65 |
| 305 | Rostrenen | France | 1 | 0 | 0 | 10.64 |
| 306 | Donauwörth, GKSt | Germany | 1 | 0 | 0 | 10.62 |
| 307 | Montceau-les-Mines | France | 1 | 0 | 0 | 10.51 |
| 308 | Epagny Metz-Tessy | France | 1 | 0 | 0 | 10.50 |
| 309 | Castell-Platja d'Aro | Spain | 1 | 0 | 0 | 10.44 |
| 310 | Fayet | France | 1 | 0 | 0 | 10.41 |
| 311 | Aubenas | France | 1 | 0 | 0 | 10.37 |
| 312 | Wokingham | Great Britain | 1 | 1 | 0 | 10.35 |
| 313 | Arbent | France | 1 | 0 | 0 | 10.33 |
| 314 | Rendsburg, Stadt | Germany | 1 | 0 | 0 | 10.33 |
| 315 | L'Aquila | Italy | 1 | 0 | 0 | 10.32 |
| 316 | Huntingdonshire | Great Britain | 0 | 2 | 0 | 10.31 |
| 317 | Tarragona | Spain | 1 | 0 | 0 | 10.30 |
| 318 | Worms, Stadt | Germany | 1 | 1 | 0 | 10.30 |
| 319 | Fulda, Stadt | Germany | 1 | 0 | 0 | 10.30 |
| 320 | Brandenburg an der Havel, Stadt | Germany | 1 | 1 | 0 | 10.25 |
| 321 | Aue-Bad Schlema, Stadt | Germany | 1 | 0 | 0 | 10.14 |
| 322 | Schwedt/Oder, Stadt | Germany | 1 | 0 | 0 | 10.09 |
| 323 | Dieppe | France | 1 | 0 | 0 | 10.08 |
| 324 | Traunstein, GKSt | Germany | 1 | 0 | 0 | 10.06 |
| 325 | Augsburg | Germany | 1 | 1 | 0 | 10.05 |
| 326 | Narbonne | France | 1 | 0 | 0 | 10.04 |
| 327 | Weimar, Stadt | Germany | 1 | 0 | 0 | 10.04 |
| 328 | Albi | France | 1 | 0 | 1 | 9.99 |
| 329 | Montélimar | France | 1 | 0 | 0 | 9.98 |
| 330 | Chambray-lès-Tours | France | 1 | 0 | 0 | 9.96 |
| 331 | Girona | Spain | 1 | 0 | 0 | 9.96 |
| 332 | Kiel, Landeshauptstadt | Germany | 1 | 0 | 0 | 9.95 |
| 333 | Speyer, Stadt | Germany | 1 | 0 | 0 | 9.93 |
| 334 | Ostrów Wielkopolski | Poland | 1 | 0 | 0 | 9.81 |
| 335 | Thiers | France | 1 | 0 | 0 | 9.79 |
| 336 | Marseille | France | 1 | 3 | 3 | 9.75 |
| 337 | Kraków | Poland | 2 | 1 | 3 | 9.75 |
| 338 | Frankfurt am Main, Stadt | Germany | 2 | 2 | 1 | 9.75 |
| 339 | Montmorot | France | 1 | 0 | 0 | 9.73 |
| 340 | Schwentinental, Stadt | Germany | 1 | 0 | 0 | 9.65 |
| 341 | Saint-Pierre-des-Corps | France | 1 | 0 | 0 | 9.47 |
| 342 | Brescia | Italy | 1 | 0 | 0 | 9.47 |
| 343 | Estancarbon | France | 1 | 0 | 0 | 9.26 |
| 344 | Chalezeule | France | 1 | 0 | 0 | 9.25 |
| 345 | Chartres | France | 1 | 0 | 0 | 9.20 |
| 346 | Landshut | Germany | 1 | 0 | 1 | 9.14 |
| 347 | Albert | France | 1 | 0 | 0 | 9.12 |
| 348 | Tarnów | Poland | 1 | 0 | 0 | 9.11 |
| 349 | BrightonandHove | Great Britain | 0 | 1 | 2 | 9.03 |
| 350 | Liverpool | Great Britain | 0 | 0 | 3 | 9.00 |
| 351 | Bolton | Great Britain | 0 | 0 | 3 | 9.00 |
| 352 | Tameside | Great Britain | 0 | 0 | 3 | 9.00 |
| 353 | Gateshead | Great Britain | 0 | 0 | 3 | 9.00 |
| 354 | Cornwall | Great Britain | 0 | 0 | 3 | 9.00 |
| 355 | Sefton | Great Britain | 0 | 1 | 1 | 9.00 |
| 356 | Stockport | Great Britain | 0 | 1 | 1 | 9.00 |
| 357 | Stroud | Great Britain | 0 | 1 | 1 | 9.00 |
| 358 | Eastleigh | Great Britain | 0 | 1 | 1 | 9.00 |
| 359 | Warrington | Great Britain | 0 | 1 | 1 | 9.00 |
| 360 | Newport | Great Britain | 0 | 1 | 1 | 9.00 |
| 361 | Warwick | Great Britain | 0 | 1 | 1 | 9.00 |
| 362 | Torbay | Great Britain | 0 | 1 | 1 | 9.00 |
| 363 | Lincoln | Great Britain | 0 | 1 | 1 | 9.00 |
| 364 | Cheltenham | Great Britain | 0 | 1 | 1 | 9.00 |
| 365 | Stoke-on-Trent | Great Britain | 0 | 1 | 1 | 9.00 |
| 366 | Stafford | Great Britain | 0 | 1 | 1 | 9.00 |
| 367 | Exeter | Great Britain | 0 | 1 | 1 | 9.00 |
| 368 | Swindon | Great Britain | 0 | 1 | 1 | 9.00 |
| 369 | Barnsley | Great Britain | 0 | 1 | 1 | 9.00 |
| 370 | TauntonDeane | Great Britain | 0 | 1 | 1 | 9.00 |
| 371 | PerthshireandKinross | Great Britain | 0 | 1 | 1 | 9.00 |
| 372 | SouthSomerset | Great Britain | 0 | 1 | 1 | 9.00 |
| 373 | Broxtowe | Great Britain | 0 | 1 | 1 | 9.00 |
| 374 | Doncaster | Great Britain | 0 | 1 | 1 | 9.00 |
| 375 | Trondheim | Norway | 0 | 1 | 1 | 9.00 |
| 376 | Romorantin-Lanthenay | France | 0 | 1 | 1 | 9.00 |
| 377 | Celle, Stadt | Germany | 0 | 1 | 1 | 9.00 |
| 378 | Nordhorn, Stadt | Germany | 0 | 1 | 1 | 9.00 |
| 379 | Singen (Hohentwiel), Stadt | Germany | 0 | 1 | 1 | 9.00 |
| 380 | Wilhelmshaven, Stadt | Germany | 0 | 1 | 1 | 9.00 |
| 381 | Logroño | Spain | 0 | 1 | 1 | 9.00 |
| 382 | León | Spain | 0 | 1 | 1 | 9.00 |
| 383 | Lleida | Spain | 0 | 1 | 1 | 9.00 |
| 384 | Torrelavega | Spain | 0 | 1 | 1 | 9.00 |
| 385 | Lucca | Italy | 0 | 1 | 1 | 9.00 |
| 386 | Białystok | Poland | 0 | 1 | 1 | 9.00 |
| 387 | Gdynia | Poland | 0 | 1 | 1 | 9.00 |
| 388 | Wels | Austria | 0 | 1 | 1 | 9.00 |
| 389 | Graz | Austria | 0 | 0 | 3 | 9.00 |
| 390 | Winchester | Great Britain | 0 | 1 | 1 | 9.00 |
| 391 | Dreux | France | 1 | 0 | 0 | 8.95 |
| 392 | Koblenz, Stadt | Germany | 1 | 0 | 0 | 8.95 |
| 393 | Terni | Italy | 1 | 0 | 0 | 8.93 |
| 394 | Neu-Ulm, GKSt | Germany | 1 | 0 | 0 | 8.93 |
| 395 | Tours | France | 1 | 0 | 0 | 8.86 |
| 396 | Tourville-la-Rivière | France | 1 | 0 | 0 | 8.85 |
| 397 | Pirmasens, Stadt | Germany | 1 | 0 | 0 | 8.82 |
| 398 | Cremona | Italy | 1 | 0 | 0 | 8.73 |
| 399 | Louvroil | France | 1 | 0 | 0 | 8.68 |
| 400 | Puget-sur-Argens | France | 1 | 0 | 0 | 8.66 |

## By Country

### Germany (DE)

Top-400 entries: **105**. Markets with at least one Tier-1 cluster: **98**. Markets with civic anchoring: **105**. Highest-ranked market: **Nürnberg** at rank **1**.

Germany contributes the largest share of the top 400, reflecting its polycentric urban structure. Major regional centres outside Berlin, Hamburg, and Munich — Nürnberg, Dresden, Mannheim, Würzburg, Hannover, Bremen, Bielefeld — are well represented, alongside numerous mid-sized cities anchored by university hospitals (Universitätskliniken) and dense hardware-and-hypermarket retail composition.

### France (FR)

Top-400 entries: **95**. Markets with at least one Tier-1 cluster: **94**. Markets with civic anchoring: **95**. Highest-ranked market: **Nice** at rank **10**.

France ranks second by count, with strong representation from secondary and tertiary cities. The pattern reflects the prevalence of out-of-town hypermarket-and-hardware co-locations associated with Centre Commercial-style developments. Civic anchoring frequently includes the Centre Hospitalier Universitaire (CHU) network and regional universities.

### Great Britain (GB)

Top-400 entries: **70**. Markets with at least one Tier-1 cluster: **13**. Markets with civic anchoring: **70**. Highest-ranked market: **Leeds** at rank **29**.

Great Britain shows a tier-two-dominated profile: 70 markets with relatively few Tier-1 entries but substantial Tier-2 and Tier-3 representation. Leeds is the highest-ranked British market at 29. Local authority districts and unitary authorities — Sheffield, Birmingham, Wiltshire, Cheshire East — appear frequently in the mid-tier range.

### Spain (ES)

Top-400 entries: **40**. Markets with at least one Tier-1 cluster: **36**. Markets with civic anchoring: **40**. Highest-ranked market: **Palma** at rank **13**.

Spain contributes 40 markets, anchored by Palma (13), Zaragoza (17), and a series of provincial capitals. Spanish entries cluster around hypermarket-and-hardware compositions typical of polígono industrial developments on the periphery of regional centres. Civic anchoring includes Hospitales Universitarios and provincial university campuses.

### Poland (PL)

Top-400 entries: **31**. Markets with at least one Tier-1 cluster: **28**. Markets with civic anchoring: **31**. Highest-ranked market: **Łódź** at rank **5**.

Poland delivers strong Tier-1 performance: 31 markets in the top 400 with Łódź (5), Gdańsk (6), Bydgoszcz (8), Poznań (9), and Lublin (11) all in the top 12. Polish regional centres show concentrated retail tier composition, frequently combining international hypermarket brands (Kaufland, Auchan, Carrefour, Tesco) with regional anchors.

### Italy (IT)

Top-400 entries: **26**. Markets with at least one Tier-1 cluster: **25**. Markets with civic anchoring: **26**. Highest-ranked market: **Bologna** at rank **78**.

Italy contributes 26 markets, headed by Bologna (78), Genova (79), and Bari (80). Italian entries reflect both northern industrial cities and southern coastal centres. Civic anchoring is typically by the policlinico universitario network.

### Austria (AT)

Top-400 entries: **12**. Markets with at least one Tier-1 cluster: **10**. Markets with civic anchoring: **11**. Highest-ranked market: **Salzburg** at rank **32**.

Austria's 12 markets are led by Salzburg (32), Klagenfurt (50), and Vienna (98). The Austrian profile is balanced across tiers, with civic anchoring through Landeskliniken and federal universities.

### Denmark (DK)

Top-400 entries: **9**. Markets with at least one Tier-1 cluster: **3**. Markets with civic anchoring: **9**. Highest-ranked market: **Aarhus** at rank **18**.

Denmark contributes 9 markets, with Aarhus the highest-ranked at 18. Danish entries are notable for tier-two-dominated profiles and strong civic anchoring through the Region-based hospital network.

### Sweden (SE)

Top-400 entries: **5**. Markets with at least one Tier-1 cluster: **5**. Markets with civic anchoring: **5**. Highest-ranked market: **Göteborg** at rank **26**.

Sweden's 5 markets are led by Göteborg (26), reflecting the geographic dispersion of Swedish secondary cities outside the Stockholm core.

### Finland (FI)

Top-400 entries: **3**. Markets with at least one Tier-1 cluster: **2**. Markets with civic anchoring: **3**. Highest-ranked market: **Tampere** at rank **12**.

Finland is represented by Tampere (12), Lahti (58), and Jyväskylä (271). Tampere's rank reflects its position as Finland's second urban centre and its concentration of major retail and civic anchors.

### Greece (GR)

Top-400 entries: **1**. Markets with at least one Tier-1 cluster: **1**. Markets with civic anchoring: **1**. Highest-ranked market: **Κοινότητα Πυλαίας** at rank **4**.

Greece is represented by a single entry: Κοινότητα Πυλαίας (Pylaía) at rank 4, an exceptional outlier driven by an unusually dense Tier-3 composition combined with civic infrastructure adjacent to Thessaloniki.

### Netherlands (NL)

Top-400 entries: **1**. Markets with at least one Tier-1 cluster: **1**. Markets with civic anchoring: **1**. Highest-ranked market: **Groningen** at rank **84**.

The Netherlands is represented by Groningen at rank 84. Other Dutch markets fall outside the top 400 in the current dataset, in part because Dutch retail concentration tends to be more evenly distributed across the national territory.

### Norway (NO)

Top-400 entries: **1**. Markets with at least one Tier-1 cluster: **0**. Markets with civic anchoring: **1**. Highest-ranked market: **Trondheim** at rank **375**.

Norway is represented by Trondheim at rank 375.

### Portugal (PT)

Top-400 entries: **1**. Markets with at least one Tier-1 cluster: **1**. Markets with civic anchoring: **1**. Highest-ranked market: **União das freguesias de Perafita, Lavra e Santa Cruz do Bispo** at rank **252**.

Portugal is represented by the União das freguesias de Perafita, Lavra e Santa Cruz do Bispo at rank 252, a parish-level designation in the Porto metropolitan area reflecting the granularity of Portuguese civic boundary data.

## Distribution by Tier

Of the 400 markets, **317** (79%) contain at least one Tier-1 cluster, **127** (32%) contain at least one Tier-2 cluster, and **129** (32%) contain at least one Tier-3 cluster. A market may contribute to more than one tier count if it hosts clusters of multiple tiers.

Single-tier composition is the most common pattern. **219** markets contain only Tier-1 clusters, **25** contain only Tier-2, and **6** contain only Tier-3. **150** markets contain a mixed composition spanning two or more tiers — a profile that typically signals a larger regional centre with both anchor-class developments and smaller secondary clusters.

Civic anchoring is near-universal in the top 400: **399** of 400 markets (99.75%) include at least one hospital or university within the co-location footprint of at least one cluster. The single exception in the list is the central Wien (Vienna) market, where the civic facilities are located in administrative districts adjacent to but not within the retail clusters identified by the analysis.

## Civic Infrastructure

European civic anchoring follows national patterns. In Germany, the dominant civic anchor is the Universitätsklinikum — the federally chartered university hospital that combines teaching, research, and tertiary care under a single institutional umbrella. Markets such as Heidelberg, Würzburg, Erlangen, Mannheim, and Freiburg derive a substantial share of their composite score from the depth of the local Universitätsklinikum complex and the associated Hochschulen.

In France, the Centre Hospitalier Universitaire (CHU) network plays the equivalent role. The CHUs are typically the largest single employer in their host city and are commonly co-located with regional universities. The Polish landscape combines large public university systems (Uniwersytet Łódzki, Uniwersytet im. Adama Mickiewicza w Poznaniu) with regional hospital complexes. Italian civic anchoring is led by the policlinico universitario system (Policlinico Sant'Orsola in Bologna, Policlinico San Martino in Genova).

Great Britain shows a distinctive profile: the National Health Service trust system provides hospital anchoring at most ranked markets, while academic anchoring is more variable and concentrated in the traditional university cities. Spanish civic infrastructure is anchored by Hospitales Universitarios and the public university network. Northern European markets (Denmark, Sweden, Finland, Norway) combine Region- or county-based hospital systems with national university networks.

## AEC Data Coverage

European Regional Markets are characterised by several Architecture–Engineering–Construction (AEC) data layers in the underlying dataset:

*EU regulatory climate zones.* The European Union's harmonised climate-zone classification is intended to support building energy regulation. Coverage is substantially complete for Germany and France (approaching 100% of European Regional Markets in those countries), but remains limited for Spain (approximately 3.7% in the current dataset) and is absent or incomplete for Great Britain, Greece, and several smaller member states. Markets in countries with limited coverage are flagged accordingly in the data tables.

*Köppen–Geiger climate classification.* The Köppen–Geiger system provides 100% global coverage and is available for every Regional Market in the dataset. Continental and northern European markets are predominantly classified Dfb (warm-summer humid continental) or Cfb (temperate oceanic). Southern European markets are predominantly Csa (Mediterranean hot-summer) or Csb (Mediterranean warm-summer).

*WWF Terrestrial Ecoregions.* The World Wildlife Fund's Terrestrial Ecoregions classification provides ecoregion and biome coverage for over 99% of Regional Markets. The dominant European biome is Temperate Broadleaf & Mixed Forests, with Mediterranean Forests, Woodlands & Scrub in the southern peninsular markets.

*ASHRAE climate zones.* The ASHRAE 169 climate-zone system is United States–only and is not applicable to European markets.

## Data Sources

- Co-location cluster dataset: 6,493 clusters across 18 countries (2026 vintage), of which the European subset comprises approximately 2,109 Regional Markets.
- Retail point-of-interest data: OpenStreetMap contributors (ODbL 1.0) and Overture Maps Foundation (CDLA Permissive 2.0).
- Civic anchor data: Overture Maps Foundation healthcare and education category extracts, supplemented by national authority registries where available.
- Population data: Kontur Population 2023 (CC BY 4.0), 400-metre H3 hexagonal grid.
- Climate-zone data: European Union harmonised climate zones; Köppen–Geiger present-day classification (Beck et al., 2018); WWF Terrestrial Ecoregions (Olson et al., 2001).
- Metropolitan reference set: 60 largest European urban areas by population, derived from Eurostat metropolitan region statistics.

## See Also

- *Top 400 Regional Markets — North America* (planned companion article).
- *Co-location Intelligence Overview* (methodology background).
- *O-D Catchment Methodology* (treatment of home-to-work mobility within Regional Markets).
- *Trade Area Data Sources* (population, spend, and chain inventory data).
