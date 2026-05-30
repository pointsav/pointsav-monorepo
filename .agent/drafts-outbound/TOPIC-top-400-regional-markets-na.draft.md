---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC
audience: general
bcsc_class: no-disclosure-implication
version: "1.0"
date: 2026-05-30
title: "Top 400 Regional Markets — North America"
---

# Top 400 Regional Markets — North America

This is a ranked list of the four hundred highest-scoring Regional Markets in North America, where a *Regional Market* is a named settlement (city, town, or province- or state-level area) that contains one or more retail co-location clusters of major hypermarket, hardware, warehouse-club, healthcare, and higher-education facilities. Markets are ranked by a composite score that combines the tier composition of their co-locations, the presence of civic-anchor infrastructure (medical and academic), and the geographic distance from the nearest major metropolitan centre. The underlying dataset spans 6,493 co-location clusters across eighteen countries, of which 1,411 clusters are represented in the four hundred North American markets listed here.

## Overview

Regional Markets are the unit of analysis adopted for continental-scale comparison of commercial co-location patterns. A market may contain a single cluster — a tightly grouped set of retail and civic anchors — or, in the case of state- and province-level aggregations, several dozen clusters that share a common name attribution. The ranking is designed to surface markets where multiple high-tier co-locations coincide with civic infrastructure and lie sufficiently far from a primary metropolitan core to function as independent regional service centres rather than as extensions of a nearby city.

The three tiers used in the ranking describe progressively richer combinations of anchor categories. *Tier 1* (T1) clusters contain at least one hypermarket (such as Walmart Supercenter, Target Supercenter, Meijer, or H-E-B), one home-improvement warehouse (Home Depot, Lowe's), and one warehouse club (Costco, Sam's Club, BJ's). *Tier 2* (T2) clusters contain a hypermarket and a hardware anchor without a warehouse club. *Tier 3* (T3) clusters contain a single recognised anchor category, typically a hypermarket. The civic dimension of the ranking is captured separately: a market is marked civic-positive when its clusters include at least one healthcare or higher-education facility, reflecting the empirical observation that suburban and exurban commercial co-locations frequently develop alongside hospital systems, community colleges, and university satellite campuses.

## Ranking Methodology

The composite score is the product of four factors. *Tier score* counts four points per Tier 1 cluster, two points per Tier 2 cluster, and one point per Tier 3 cluster within the market. *Civic multiplier* is 1.5 when at least one cluster in the market contains a medical or academic anchor and 1.0 otherwise; medical anchors include hospitals, clinics, university medical centres, and emergency services, while academic anchors include universities, colleges, community colleges, and research campuses. *Metropolitan-distance multiplier* scales between 0.5 and 2.0 based on the great-circle distance from the market centroid to the nearest of sixty major metropolitan reference centroids; the multiplier reaches its maximum at distances of one hundred kilometres or more, reflecting the design intent to reward markets that serve regional populations not captured by core urban retail catchments. *Confidence factor* is 1.0 for markets validated against high-confidence settlement boundaries and 0.7 for markets where the geographic boundary remains provisional.

The full formula is: *Score = tier_score × civic_multiplier × metro_multiplier × confidence_factor*. The maximum score in the present dataset is 300.0 (Ontario, Canada) and the minimum among the top four hundred is 12.0. Three markets exceed a score of 100, ten more exceed 50, and the long tail of the top four hundred sits between 12 and 50.

## Top 25

The twenty-five highest-ranked Regional Markets in North America are listed below. Markets marked with a dagger (†) are state- or province-level aggregations that combine several same-named or geographically dispersed clusters under a single label; readers seeking city-level detail for these markets should refer to the entries for individual cities within the relevant state or province.

| Rank | Market | State/Province | Country | T1 | T2 | T3 | Civic | Score |
|------|--------|----------------|---------|----|----|----|-------|-------|
| 1 | Ontario † | Ontario | CA | 10 | 19 | 22 | Yes | 300.0 |
| 2 | Texas † | Texas | US | 3 | 6 | 47 | Yes | 143.4 |
| 3 | Québec † | Québec | CA | 13 | 3 | 6 | Yes | 110.3 |
| 4 | Jacksonville, FL | FL | US | 2 | 9 | 5 | Yes | 93.0 |
| 5 | El Paso, TX | TX | US | 5 | 4 | 3 | Yes | 93.0 |
| 6 | Columbus, OH | OH | US | 8 | 5 | 0 | Yes | 68.6 |
| 7 | Springfield, IL | IL | US | 1 | 7 | 4 | Yes | 66.0 |
| 8 | Franklin, NC | NC | US | 3 | 4 | 2 | Yes | 66.0 |
| 9 | Columbia, MO | MO | US | 4 | 2 | 1 | Yes | 63.0 |
| 10 | Aurora, CO | CO | US | 4 | 2 | 0 | Yes | 60.0 |
| 11 | Manchester, CT | CT | US | 3 | 3 | 0 | Yes | 54.0 |
| 12 | Houston, TX | TX | US | 10 | 11 | 7 | Yes | 51.8 |
| 13 | Fayetteville, NC | NC | US | 1 | 6 | 1 | Yes | 51.0 |
| 14 | Colorado Springs, CO | CO | US | 2 | 4 | 1 | Yes | 48.8 |
| 15 | Madison, MS | MS | US | 2 | 3 | 2 | Yes | 48.0 |
| 16 | Wichita, KS | KS | US | 4 | 0 | 0 | Yes | 48.0 |
| 17 | Greenville, SC | SC | US | 1 | 5 | 1 | Yes | 45.0 |
| 18 | Bakersfield, CA | CA | US | 2 | 3 | 1 | Yes | 45.0 |
| 19 | Huntsville, AL | AL | US | 2 | 3 | 1 | Yes | 45.0 |
| 20 | Tulsa, OK | OK | US | 3 | 0 | 3 | Yes | 45.0 |
| 21 | Florence, SC | SC | US | 3 | 1 | 1 | Yes | 45.0 |
| 22 | Louisiana † | Louisiana | US | 2 | 2 | 9 | Yes | 44.1 |
| 23 | Gainesville, VA | VA | US | 2 | 3 | 1 | Yes | 42.0 |
| 24 | Omaha, NE | NE | US | 1 | 4 | 2 | Yes | 42.0 |
| 25 | Glendale, AZ | AZ | US | 2 | 3 | 0 | Yes | 42.0 |

**†** Market spans multiple same-named settlements or aggregates clusters at the state or province level; geographic detail for constituent cities is recorded in the individual cluster source records.

## Full Rankings (26–400)

The remaining markets ranked 26 through 400 are listed below in compact form. Daggers continue to mark state- and province-level aggregations.

| Rank | Market | Country | T1 | T2 | T3 | Score |
|------|--------|---------|----|----|----|-------|
| 26 | Jackson, TN | US | 2 | 3 | 0 | 42.0 |
| 27 | Halifax, Nova Scotia | CA | 2 | 3 | 0 | 42.0 |
| 28 | Lebanon, NH | US | 1 | 4 | 1 | 39.0 |
| 29 | Westminster, CA | US | 2 | 2 | 1 | 39.0 |
| 30 | Mount Pleasant, TX | US | 2 | 2 | 1 | 39.0 |
| 31 | Lincoln, NE | US | 2 | 2 | 1 | 39.0 |
| 32 | Juárez | MX | 1 | 0 | 8 | 36.0 |
| 33 | Greensboro, NC | US | 1 | 3 | 2 | 36.0 |
| 34 | Lancaster, PA | US | 1 | 3 | 2 | 36.0 |
| 35 | Lafayette, CO | US | 2 | 0 | 4 | 36.0 |
| 36 | Roseville, CA | US | 2 | 1 | 2 | 36.0 |
| 37 | Monroe, GA | US | 1 | 4 | 0 | 36.0 |
| 38 | Querétaro, Querétaro | MX | 2 | 1 | 2 | 36.0 |
| 39 | Albany, NY | US | 2 | 2 | 0 | 36.0 |
| 40 | Concord, NH | US | 3 | 0 | 0 | 36.0 |
| 41 | Covington, LA | US | 1 | 3 | 1 | 33.0 |
| 42 | Amarillo, TX | US | 1 | 3 | 1 | 33.0 |
| 43 | Arlington, TX | US | 2 | 1 | 1 | 33.0 |
| 44 | Sonora, CA | US | 2 | 1 | 1 | 33.0 |
| 45 | New York, NY | US | 5 | 5 | 13 | 32.2 |
| 46 | Portland, OR | US | 1 | 2 | 2 | 30.0 |
| 47 | Knoxville, TN | US | 1 | 2 | 2 | 30.0 |
| 48 | Winchester, VA | US | 1 | 3 | 0 | 30.0 |
| 49 | Fort Wayne, IN | US | 1 | 3 | 0 | 30.0 |
| 50 | Auburn, IN | US | 1 | 3 | 0 | 30.0 |
| 51 | Medford, OR | US | 2 | 0 | 2 | 30.0 |
| 52 | Woodstock, VA | US | 1 | 3 | 0 | 30.0 |
| 53 | Louisville/Jefferson County metro government (balance), KY | US | 1 | 3 | 0 | 30.0 |
| 54 | Fullerton, CA | US | 2 | 1 | 0 | 30.0 |
| 55 | Winston-Salem, NC | US | 2 | 1 | 0 | 30.0 |
| 56 | Avon, OH | US | 2 | 1 | 0 | 30.0 |
| 57 | Oxford, MA | US | 2 | 1 | 0 | 30.0 |
| 58 | Bellevue, WI | US | 2 | 1 | 0 | 30.0 |
| 59 | Chesapeake, VA | US | 2 | 1 | 0 | 30.0 |
| 60 | Salem, OR | US | 0 | 4 | 2 | 30.0 |
| 61 | Clinton, UT | US | 0 | 5 | 0 | 30.0 |
| 62 | San Antonio, TX | US | 4 | 8 | 7 | 29.2 |
| 63 | Marietta, OH | US | 1 | 2 | 1 | 27.0 |
| 64 | Reno, NV | US | 1 | 2 | 1 | 27.0 |
| 65 | Burlington, NJ | US | 1 | 2 | 1 | 27.0 |
| 66 | Rochester, MN | US | 1 | 2 | 1 | 27.0 |
| 67 | Baton Rouge, LA | US | 1 | 2 | 1 | 27.0 |
| 68 | Blaine, BC | CA | 1 | 2 | 1 | 27.0 |
| 69 | Shreveport, LA | US | 1 | 2 | 1 | 27.0 |
| 70 | Meridian, ID | US | 1 | 2 | 1 | 27.0 |
| 71 | London | CA | 1 | 2 | 1 | 27.0 |
| 72 | Hendersonville, NC | US | 2 | 0 | 1 | 27.0 |
| 73 | McAllen, TX | US | 2 | 0 | 1 | 27.0 |
| 74 | Woodbury, NY | US | 2 | 0 | 1 | 27.0 |
| 75 | Guadalupe | MX | 2 | 0 | 1 | 27.0 |
| 76 | Peoria, AZ | US | 0 | 3 | 3 | 27.0 |
| 77 | Washington, MO | US | 0 | 4 | 1 | 27.0 |
| 78 | Bristol, TN | US | 2 | 1 | 1 | 26.7 |
| 79 | Richmond, VA | US | 0 | 4 | 1 | 26.4 |
| 80 | British Columbia † | CA | 2 | 4 | 7 | 25.5 |
| 81 | Laredo | US | 1 | 2 | 0 | 24.0 |
| 82 | Mobile, AL | US | 1 | 2 | 0 | 24.0 |
| 83 | Conway, SC | US | 1 | 2 | 0 | 24.0 |
| 84 | Fremont, CA | US | 1 | 2 | 0 | 24.0 |
| 85 | Mount Vernon, WA | US | 1 | 2 | 0 | 24.0 |
| 86 | Anderson, SC | US | 1 | 2 | 0 | 24.0 |
| 87 | Danville, KY | US | 1 | 2 | 0 | 24.0 |
| 88 | Boise City, ID | US | 1 | 2 | 0 | 24.0 |
| 89 | Pasadena, MD | US | 1 | 2 | 0 | 24.0 |
| 90 | St. John's | CA | 1 | 2 | 0 | 24.0 |
| 91 | Bloomington, IN | US | 2 | 0 | 0 | 24.0 |
| 92 | Saratoga Springs, UT | US | 2 | 0 | 0 | 24.0 |
| 93 | Kentwood, MI | US | 2 | 0 | 0 | 24.0 |
| 94 | Morelia | MX | 2 | 0 | 0 | 24.0 |
| 95 | San Luis Potosí † | MX | 2 | 0 | 0 | 24.0 |
| 96 | Roswell, GA | US | 0 | 3 | 2 | 24.0 |
| 97 | Jasper, GA | US | 0 | 4 | 0 | 24.0 |
| 98 | Hamilton | CA | 0 | 6 | 1 | 23.8 |
| 99 | Murrieta, CA | US | 2 | 0 | 0 | 23.1 |
| 100 | Waterloo, ON | CA | 1 | 2 | 0 | 22.9 |
| 101 | Toronto, ON | CA | 3 | 6 | 6 | 22.5 |
| 102 | Los Angeles, CA | US | 3 | 7 | 4 | 22.5 |
| 103 | Redmond, OR | US | 1 | 2 | 0 | 22.5 |
| 104 | Brighton, NY | US | 1 | 2 | 0 | 22.0 |
| 105 | Calgary, Alberta | CA | 3 | 8 | 1 | 21.8 |
| 106 | Austin, TX | US | 4 | 6 | 1 | 21.8 |
| 107 | Fort Collins, CO | US | 1 | 2 | 0 | 21.6 |
| 108 | Henderson, NC | US | 1 | 3 | 1 | 21.5 |
| 109 | Dallas, TX | US | 5 | 2 | 4 | 21.0 |
| 110 | Sinaloa † | MX | 2 | 0 | 2 | 21.0 |
| 111 | Mérida | MX | 1 | 0 | 3 | 21.0 |
| 112 | Westfield, MA | US | 1 | 1 | 1 | 21.0 |
| 113 | Antioch, CA | US | 1 | 1 | 1 | 21.0 |
| 114 | Centerville, UT | US | 1 | 1 | 1 | 21.0 |
| 115 | Alexandria, LA | US | 1 | 1 | 1 | 21.0 |
| 116 | Brownsville, TX | US | 1 | 1 | 1 | 21.0 |
| 117 | Temple, TX | US | 1 | 1 | 1 | 21.0 |
| 118 | Kennewick, WA | US | 1 | 1 | 1 | 21.0 |
| 119 | Bellingham, WA | US | 1 | 1 | 1 | 21.0 |
| 120 | St. Charles, MO | US | 1 | 1 | 1 | 21.0 |
| 121 | Longview, WA | US | 1 | 1 | 1 | 21.0 |
| 122 | Plymouth, MI | US | 1 | 1 | 1 | 21.0 |
| 123 | Midland, MI | US | 1 | 1 | 1 | 21.0 |
| 124 | Corpus Christi, TX | US | 1 | 1 | 1 | 21.0 |
| 125 | Mexicali | MX | 1 | 1 | 1 | 21.0 |
| 126 | Virginia Beach, VA | US | 0 | 2 | 3 | 21.0 |
| 127 | Sanford, FL | US | 0 | 3 | 1 | 21.0 |
| 128 | Savannah, GA | US | 0 | 3 | 1 | 21.0 |
| 129 | Spring Hill, TN | US | 0 | 3 | 1 | 21.0 |
| 130 | Portsmouth, NH | US | 1 | 2 | 0 | 20.8 |
| 131 | Clovis, CA | US | 1 | 2 | 0 | 20.6 |
| 132 | Belleville, MI | US | 0 | 4 | 0 | 20.3 |
| 133 | Nashua, NH | US | 2 | 2 | 0 | 20.1 |
| 134 | Phoenix, AZ | US | 3 | 5 | 4 | 19.5 |
| 135 | Charlotte, NC | US | 3 | 6 | 2 | 19.5 |
| 136 | Nuevo León † | MX | 6 | 4 | 5 | 19.4 |
| 137 | Frisco, TX | US | 3 | 1 | 0 | 18.4 |
| 138 | Springdale, AR | US | 1 | 0 | 2 | 18.0 |
| 139 | Culiacán | MX | 1 | 0 | 2 | 18.0 |
| 140 | Centro | MX | 1 | 0 | 2 | 18.0 |
| 141 | Norwalk, CA | US | 1 | 1 | 0 | 18.0 |
| 142 | Naples, FL | US | 1 | 1 | 0 | 18.0 |
| 143 | Cedar Rapids, IA | US | 1 | 1 | 0 | 18.0 |
| 144 | Homewood, AL | US | 1 | 1 | 0 | 18.0 |
| 145 | Roanoke, TX | US | 1 | 1 | 0 | 18.0 |
| 146 | Lexington-Fayette, KY | US | 1 | 1 | 0 | 18.0 |
| 147 | Cape Coral, FL | US | 1 | 1 | 0 | 18.0 |
| 148 | Union City, CA | US | 1 | 1 | 0 | 18.0 |
| 149 | Spartanburg, SC | US | 1 | 1 | 0 | 18.0 |
| 150 | St. George, UT | US | 1 | 1 | 0 | 18.0 |
| 151 | Calexico | US | 1 | 1 | 0 | 18.0 |
| 152 | Hudson, WI | US | 1 | 1 | 0 | 18.0 |
| 153 | Homestead, FL | US | 1 | 1 | 0 | 18.0 |
| 154 | Ocala, FL | US | 1 | 1 | 0 | 18.0 |
| 155 | Appleton, WI | US | 1 | 1 | 0 | 18.0 |
| 156 | Augusta, ME | US | 1 | 1 | 0 | 18.0 |
| 157 | Macon-Bibb County, GA | US | 1 | 1 | 0 | 18.0 |
| 158 | Waverly, MI | US | 1 | 1 | 0 | 18.0 |
| 159 | Chillicothe, MO | US | 1 | 1 | 0 | 18.0 |
| 160 | Jefferson City, MO | US | 1 | 1 | 0 | 18.0 |
| 161 | Superior, WI | US | 1 | 1 | 0 | 18.0 |
| 162 | St. Cloud, FL | US | 1 | 1 | 0 | 18.0 |
| 163 | Warrenton, VA | US | 1 | 1 | 0 | 18.0 |
| 164 | Mansfield, PA | US | 1 | 1 | 0 | 18.0 |
| 165 | Rome, NY | US | 1 | 1 | 0 | 18.0 |
| 166 | Sioux Falls, SD | US | 1 | 1 | 0 | 18.0 |
| 167 | Clarksville, TN | US | 1 | 1 | 0 | 18.0 |
| 168 | Fairlawn, OH | US | 1 | 1 | 0 | 18.0 |
| 169 | Watertown, NY | US | 1 | 1 | 0 | 18.0 |
| 170 | Baxter, MN | US | 1 | 1 | 0 | 18.0 |
| 171 | Evansville, IN | US | 1 | 1 | 0 | 18.0 |
| 172 | Pensacola, FL | US | 1 | 1 | 0 | 18.0 |
| 173 | Little Rock, AR | US | 1 | 1 | 0 | 18.0 |
| 174 | Nogales, SO | US | 1 | 1 | 0 | 18.0 |
| 175 | Sparks, NV | US | 1 | 1 | 0 | 18.0 |
| 176 | Dothan, AL | US | 1 | 1 | 0 | 18.0 |
| 177 | Pueblo, CO | US | 1 | 1 | 0 | 18.0 |
| 178 | Owensboro, KY | US | 1 | 1 | 0 | 18.0 |
| 179 | Farmington, NM | US | 1 | 1 | 0 | 18.0 |
| 180 | Paducah, KY | US | 1 | 1 | 0 | 18.0 |
| 181 | Bluffton, IN | US | 1 | 1 | 0 | 18.0 |
| 182 | Peterborough, ON | CA | 1 | 1 | 0 | 18.0 |
| 183 | Courtenay | CA | 1 | 1 | 0 | 18.0 |
| 184 | Fredericton | CA | 1 | 1 | 0 | 18.0 |
| 185 | Greater Sudbury, ON | CA | 1 | 1 | 0 | 18.0 |
| 186 | Aguascalientes, Aguascalientes | MX | 1 | 1 | 0 | 18.0 |
| 187 | León, Guanajuato | MX | 1 | 1 | 0 | 18.0 |
| 188 | Middletown, OH | US | 0 | 3 | 0 | 18.0 |
| 189 | Milwaukee, WI | US | 0 | 3 | 0 | 18.0 |
| 190 | Troy, OH | US | 0 | 3 | 0 | 18.0 |
| 191 | Athens, TN | US | 0 | 3 | 0 | 18.0 |
| 192 | Princeton, IN | US | 0 | 3 | 0 | 18.0 |
| 193 | Regina, SK | CA | 0 | 3 | 0 | 18.0 |
| 194 | Victorville, CA | US | 1 | 1 | 0 | 17.4 |
| 195 | Santa Clarita, CA | US | 2 | 2 | 0 | 17.4 |
| 196 | San Diego | US | 3 | 4 | 3 | 17.2 |
| 197 | Athens-Clarke County unified government (balance), GA | US | 1 | 1 | 0 | 17.1 |
| 198 | Chihuahua † | MX | 1 | 1 | 2 | 16.8 |
| 199 | New Brunswick † | CA | 2 | 0 | 0 | 16.8 |
| 200 | Philadelphia, PA | US | 3 | 4 | 1 | 16.8 |
| 201 | Lakewood, CA | US | 3 | 3 | 1 | 16.5 |
| 202 | Corona, CA | US | 2 | 0 | 0 | 16.5 |
| 203 | Edmonton | CA | 2 | 5 | 4 | 16.5 |
| 204 | Fontana, CA | US | 1 | 1 | 1 | 15.8 |
| 205 | Temecula, CA | US | 1 | 1 | 0 | 15.7 |
| 206 | Irvine, CA | US | 2 | 0 | 1 | 15.7 |
| 207 | Barrie, ON | CA | 1 | 1 | 0 | 15.0 |
| 208 | Indianapolis city (balance), IN | US | 3 | 3 | 2 | 15.0 |
| 209 | Slidell, LA | US | 1 | 0 | 1 | 15.0 |
| 210 | Palm Beach Gardens, FL | US | 1 | 0 | 1 | 15.0 |
| 211 | Canton, GA | US | 1 | 0 | 1 | 15.0 |
| 212 | Yuma, AZ | US | 1 | 0 | 1 | 15.0 |
| 213 | Mechanicsburg, PA | US | 1 | 0 | 1 | 15.0 |
| 214 | Idaho Falls, ID | US | 1 | 0 | 1 | 15.0 |
| 215 | Palm Desert, CA | US | 1 | 0 | 1 | 15.0 |
| 216 | Wilmington, OH | US | 1 | 0 | 1 | 15.0 |
| 217 | Missoula, MT | US | 1 | 0 | 1 | 15.0 |
| 218 | Port St. Lucie, FL | US | 1 | 0 | 1 | 15.0 |
| 219 | Johnson City, NY | US | 1 | 0 | 1 | 15.0 |
| 220 | Fort Smith, AR | US | 1 | 0 | 1 | 15.0 |
| 221 | Spokane Valley, WA | US | 1 | 0 | 1 | 15.0 |
| 222 | Rapid City, SD | US | 1 | 0 | 1 | 15.0 |
| 223 | Pooler, GA | US | 1 | 0 | 1 | 15.0 |
| 224 | Bismarck, ND | US | 1 | 0 | 1 | 15.0 |
| 225 | Lubbock, TX | US | 1 | 0 | 1 | 15.0 |
| 226 | Bend, OR | US | 1 | 0 | 1 | 15.0 |
| 227 | Grandville, MI | US | 1 | 0 | 1 | 15.0 |
| 228 | Acapulco de Juárez | MX | 1 | 0 | 1 | 15.0 |
| 229 | Celaya, Guanajuato | MX | 1 | 0 | 1 | 15.0 |
| 230 | Mazatlán | MX | 1 | 0 | 1 | 15.0 |
| 231 | Durango | MX | 1 | 0 | 1 | 15.0 |
| 232 | Boca del Río | MX | 1 | 0 | 1 | 15.0 |
| 233 | Hermosillo | MX | 1 | 0 | 1 | 15.0 |
| 234 | Ensenada | MX | 1 | 0 | 1 | 15.0 |
| 235 | Tuxt la Gutiérrez | MX | 1 | 0 | 1 | 15.0 |
| 236 | Reynosa | MX | 1 | 0 | 1 | 15.0 |
| 237 | New Orleans, LA | US | 0 | 1 | 3 | 15.0 |
| 238 | North Charleston, SC | US | 0 | 2 | 1 | 15.0 |
| 239 | West Palm Beach, FL | US | 0 | 2 | 1 | 15.0 |
| 240 | Chattanooga, TN | US | 0 | 2 | 1 | 15.0 |
| 241 | Georgetown, TX | US | 0 | 2 | 1 | 15.0 |
| 242 | Aberdeen, SD | US | 0 | 2 | 1 | 15.0 |
| 243 | Greenwood, IN | US | 0 | 2 | 1 | 15.0 |
| 244 | Bowling Green, OH | US | 0 | 2 | 1 | 15.0 |
| 245 | Asheville, NC | US | 0 | 2 | 1 | 15.0 |
| 246 | Decatur, IL | US | 0 | 2 | 1 | 15.0 |
| 247 | Hillsboro, OR | US | 0 | 2 | 1 | 15.0 |
| 248 | Modesto, CA | US | 1 | 1 | 0 | 15.0 |
| 249 | Shiloh, IL | US | 1 | 1 | 0 | 14.9 |
| 250 | Leesburg, VA | US | 1 | 1 | 0 | 14.7 |
| 251 | Rocky Mount, NC | US | 1 | 1 | 0 | 14.6 |
| 252 | Fredericksburg, VA | US | 1 | 1 | 0 | 14.6 |
| 253 | Plano, TX | US | 3 | 2 | 1 | 14.3 |
| 254 | Newport News, VA | US | 1 | 0 | 1 | 14.3 |
| 255 | Winnipeg | CA | 3 | 3 | 1 | 14.2 |
| 256 | Oklahoma City, OK | US | 4 | 1 | 1 | 14.2 |
| 257 | Jonesboro, AR | US | 1 | 0 | 1 | 14.2 |
| 258 | Mesa, AZ | US | 2 | 2 | 3 | 14.0 |
| 259 | McKinney, TX | US | 1 | 2 | 2 | 13.9 |
| 260 | St. Joseph, MO | US | 1 | 1 | 0 | 13.9 |
| 261 | Montgomery, IL | US | 1 | 0 | 2 | 13.8 |
| 262 | Salisbury, NC | US | 1 | 1 | 0 | 13.7 |
| 263 | Flint, MI | US | 1 | 0 | 1 | 13.6 |
| 264 | Joliet, IL | US | 2 | 0 | 0 | 13.6 |
| 265 | Chicago, IL | US | 1 | 6 | 2 | 13.5 |
| 266 | Ottawa | CA | 2 | 3 | 4 | 13.5 |
| 267 | Nashville-Davidson metropolitan government (balance), TN | US | 1 | 7 | 0 | 13.5 |
| 268 | Stockton, CA | US | 1 | 1 | 1 | 13.5 |
| 269 | Kitchener | CA | 0 | 2 | 1 | 13.5 |
| 270 | Santa Fe, NM | US | 1 | 0 | 1 | 12.9 |
| 271 | Lacey, WA | US | 1 | 1 | 0 | 12.8 |
| 272 | Garden City, SC | US | 1 | 1 | 0 | 12.8 |
| 273 | Tucson, AZ | US | 2 | 3 | 3 | 12.8 |
| 274 | San Jose, CA | US | 3 | 1 | 3 | 12.8 |
| 275 | Mississippi † | US | 1 | 0 | 2 | 12.6 |
| 276 | Manitoba † | CA | 3 | 3 | 1 | 12.4 |
| 277 | Cleveland, OH | US | 0 | 3 | 0 | 12.2 |
| 278 | Norfolk, VA | US | 1 | 1 | 0 | 12.1 |
| 279 | Kansas City, MO | US | 2 | 2 | 4 | 12.0 |
| 280 | Albuquerque, NM | US | 3 | 1 | 2 | 12.0 |
| 281 | Oakdale, PA | US | 1 | 1 | 0 | 12.0 |
| 282 | Portage, MI | US | 1 | 1 | 0 | 12.0 |
| 283 | Owasso, OK | US | 1 | 0 | 0 | 12.0 |
| 284 | La Quinta, CA | US | 1 | 0 | 0 | 12.0 |
| 285 | Lincoln Park, NY | US | 1 | 0 | 0 | 12.0 |
| 286 | Bayou Cane, LA | US | 1 | 0 | 0 | 12.0 |
| 287 | Lawton, OK | US | 1 | 0 | 0 | 12.0 |
| 288 | Gulfport, MS | US | 1 | 0 | 0 | 12.0 |
| 289 | Valdosta, GA | US | 1 | 0 | 0 | 12.0 |
| 290 | Champaign, IL | US | 1 | 0 | 0 | 12.0 |
| 291 | San Angelo, TX | US | 1 | 0 | 0 | 12.0 |
| 292 | Mishawaka, IN | US | 1 | 0 | 0 | 12.0 |
| 293 | California, MD | US | 1 | 0 | 0 | 12.0 |
| 294 | Sherman, TX | US | 1 | 0 | 0 | 12.0 |
| 295 | Hot Springs, AR | US | 1 | 0 | 0 | 12.0 |
| 296 | Helena, MT | US | 1 | 0 | 0 | 12.0 |
| 297 | Texarkana, TX | US | 1 | 0 | 0 | 12.0 |
| 298 | Coralville, IA | US | 1 | 0 | 0 | 12.0 |
| 299 | Richland, WA | US | 1 | 0 | 0 | 12.0 |
| 300 | Mankato, MN | US | 1 | 0 | 0 | 12.0 |
| 301 | Harrisonburg, VA | US | 1 | 0 | 0 | 12.0 |
| 302 | Chico, CA | US | 1 | 0 | 0 | 12.0 |
| 303 | West Des Moines, IA | US | 1 | 0 | 0 | 12.0 |
| 304 | East Peoria, IL | US | 1 | 0 | 0 | 12.0 |
| 305 | Eau Claire, WI | US | 1 | 0 | 0 | 12.0 |
| 306 | Rockford, IL | US | 1 | 0 | 0 | 12.0 |
| 307 | Wilkes-Barre, PA | US | 1 | 0 | 0 | 12.0 |
| 308 | Traverse City, MI | US | 1 | 0 | 0 | 12.0 |
| 309 | Topeka, KS | US | 1 | 0 | 0 | 12.0 |
| 310 | Twin Falls, ID | US | 1 | 0 | 0 | 12.0 |
| 311 | Bellmead, TX | US | 1 | 0 | 0 | 12.0 |
| 312 | Bangor, ME | US | 1 | 0 | 0 | 12.0 |
| 313 | Halfway, MD | US | 1 | 0 | 0 | 12.0 |
| 314 | West Vero Corridor, FL | US | 1 | 0 | 0 | 12.0 |
| 315 | Fort Myers, FL | US | 1 | 0 | 0 | 12.0 |
| 316 | Davenport, IA | US | 1 | 0 | 0 | 12.0 |
| 317 | Papillion, NE | US | 1 | 0 | 0 | 12.0 |
| 318 | Odessa, TX | US | 1 | 0 | 0 | 12.0 |
| 319 | Salina, KS | US | 1 | 0 | 0 | 12.0 |
| 320 | Ames, IA | US | 1 | 0 | 0 | 12.0 |
| 321 | Grand Forks, ND | US | 1 | 0 | 0 | 12.0 |
| 322 | Hattiesburg, MS | US | 1 | 0 | 0 | 12.0 |
| 323 | Council Bluffs, IA | US | 1 | 0 | 0 | 12.0 |
| 324 | Santa Maria, CA | US | 1 | 0 | 0 | 12.0 |
| 325 | Bossier City, LA | US | 1 | 0 | 0 | 12.0 |
| 326 | Endwell, NY | US | 1 | 0 | 0 | 12.0 |
| 327 | Terre Haute, IN | US | 1 | 0 | 0 | 12.0 |
| 328 | Logan, UT | US | 1 | 0 | 0 | 12.0 |
| 329 | Cheyenne, WY | US | 1 | 0 | 0 | 12.0 |
| 330 | Elizabethtown, KY | US | 1 | 0 | 0 | 12.0 |
| 331 | Latham, NY | US | 1 | 0 | 0 | 12.0 |
| 332 | Fargo, ND | US | 1 | 0 | 0 | 12.0 |
| 333 | Mitchell, SD | US | 1 | 0 | 0 | 12.0 |
| 334 | Jensen Beach, FL | US | 1 | 0 | 0 | 12.0 |
| 335 | Kalispell, MT | US | 1 | 0 | 0 | 12.0 |
| 336 | Casper, WY | US | 1 | 0 | 0 | 12.0 |
| 337 | Dickson City, PA | US | 1 | 0 | 0 | 12.0 |
| 338 | Muskegon, MI | US | 1 | 0 | 0 | 12.0 |
| 339 | Villas, FL | US | 1 | 0 | 0 | 12.0 |
| 340 | Zilwaukee, MI | US | 1 | 0 | 0 | 12.0 |
| 341 | Victoria, TX | US | 1 | 0 | 0 | 12.0 |
| 342 | Grand Junction, CO | US | 1 | 0 | 0 | 12.0 |
| 343 | Janesville, WI | US | 1 | 0 | 0 | 12.0 |
| 344 | Oneonta, NY | US | 1 | 0 | 0 | 12.0 |
| 345 | Bozeman, MT | US | 1 | 0 | 0 | 12.0 |
| 346 | Gretna, LA | US | 1 | 0 | 0 | 12.0 |
| 347 | West Buechel, KY | US | 1 | 0 | 0 | 12.0 |
| 348 | Beckley, WV | US | 1 | 0 | 0 | 12.0 |
| 349 | Lufkin, TX | US | 1 | 0 | 0 | 12.0 |
| 350 | South Portland, ME | US | 1 | 0 | 0 | 12.0 |
| 351 | Hampton, VA | US | 1 | 0 | 0 | 12.0 |
| 352 | Waterloo, IA | US | 1 | 0 | 0 | 12.0 |
| 353 | Vienna, WV | US | 1 | 0 | 0 | 12.0 |
| 354 | Royal Palm Beach, FL | US | 1 | 0 | 0 | 12.0 |
| 355 | Wright, FL | US | 1 | 0 | 0 | 12.0 |
| 356 | Green Bay, WI | US | 1 | 0 | 0 | 12.0 |
| 357 | Maryville, TN | US | 1 | 0 | 0 | 12.0 |
| 358 | Redding, CA | US | 1 | 0 | 0 | 12.0 |
| 359 | Cookeville, TN | US | 1 | 0 | 0 | 12.0 |
| 360 | El Centro, CA | US | 1 | 0 | 0 | 12.0 |
| 361 | Bluefield, VA | US | 1 | 0 | 0 | 12.0 |
| 362 | Normal, IL | US | 1 | 0 | 0 | 12.0 |
| 363 | North Little Rock, AR | US | 1 | 0 | 0 | 12.0 |
| 364 | Menomonee Falls, WI | US | 1 | 0 | 0 | 12.0 |
| 365 | Augusta-Richmond County consolidated government (balance), GA | US | 1 | 0 | 0 | 12.0 |
| 366 | Wichita Falls, TX | US | 1 | 0 | 0 | 12.0 |
| 367 | Denham Springs, LA | US | 1 | 0 | 0 | 12.0 |
| 368 | Joplin, MO | US | 1 | 0 | 0 | 12.0 |
| 369 | Grand Island, NE | US | 1 | 0 | 0 | 12.0 |
| 370 | Pewaukee, WI | US | 1 | 0 | 0 | 12.0 |
| 371 | Cape Girardeau, MO | US | 1 | 0 | 0 | 12.0 |
| 372 | Coeur d'Alene, ID | US | 1 | 0 | 0 | 12.0 |
| 373 | Elmwood, LA | US | 1 | 0 | 0 | 12.0 |
| 374 | Port Charlotte, FL | US | 1 | 0 | 0 | 12.0 |
| 375 | Sun Prairie, WI | US | 1 | 0 | 0 | 12.0 |
| 376 | Erie, PA | US | 1 | 0 | 0 | 12.0 |
| 377 | Paxtonia, PA | US | 1 | 0 | 0 | 12.0 |
| 378 | Sevierville, TN | US | 1 | 0 | 0 | 12.0 |
| 379 | Goleta, CA | US | 1 | 0 | 0 | 12.0 |
| 380 | San Luis Obispo, CA | US | 1 | 0 | 0 | 12.0 |
| 381 | Three Oaks, FL | US | 1 | 0 | 0 | 12.0 |
| 382 | Waukesha, WI | US | 1 | 0 | 0 | 12.0 |
| 383 | Nampa, ID | US | 1 | 0 | 0 | 12.0 |
| 384 | Grafton, WI | US | 1 | 0 | 0 | 12.0 |
| 385 | College Station, TX | US | 1 | 0 | 0 | 12.0 |
| 386 | Yakima, WA | US | 1 | 0 | 0 | 12.0 |
| 387 | Lethbridge, AB | CA | 1 | 0 | 0 | 12.0 |
| 388 | Saint John | CA | 1 | 0 | 0 | 12.0 |
| 389 | Rimouski | CA | 1 | 0 | 0 | 12.0 |
| 390 | Kelowna, BC | CA | 1 | 0 | 0 | 12.0 |
| 391 | Kamloops, BC | CA | 1 | 0 | 0 | 12.0 |
| 392 | Prince George, BC | CA | 1 | 0 | 0 | 12.0 |
| 393 | Gran de Prairie, AB | CA | 1 | 0 | 0 | 12.0 |
| 394 | Zamora | MX | 1 | 0 | 0 | 12.0 |
| 395 | Ahome | MX | 1 | 0 | 0 | 12.0 |
| 396 | Uruapan | MX | 1 | 0 | 0 | 12.0 |
| 397 | Coatzacoalcos | MX | 1 | 0 | 0 | 12.0 |
| 398 | Xalapa | MX | 1 | 0 | 0 | 12.0 |
| 399 | Chihuahua, Chihuahua | MX | 1 | 0 | 0 | 12.0 |
| 400 | Cajeme | MX | 1 | 0 | 0 | 12.0 |

## By Country

### United States (340 markets)

The United States accounts for 340 of the four hundred ranked markets. The states with the largest representation are CA (30), TX (27), FL (19), VA (14), WI (13), TN (11), NY (11), LA (11), MI (11), IL (10). The geographic distribution reflects strong regional retail development across the Sun Belt, the Midwest, and the upper Mid-Atlantic, with secondary concentrations in the Pacific Coast and the Mountain West. Several entries — Texas, Louisiana, and Mississippi — appear as state-level aggregations and represent rural and small-town markets grouped under a state label rather than discrete cities.

### Canada (30 markets)

Canada is represented by 30 markets distributed across ON (5), BC (4), AB (2), Ontario (1), Québec (1), Nova Scotia (1), British Columbia (1), Alberta (1), SK (1), New Brunswick (1), Manitoba (1). Ontario and Québec appear at the head of the list as province-level aggregations, reflecting the concentration of multi-anchor co-locations in the Greater Toronto and Greater Montreal regions and along the broader Windsor–Québec City corridor. Halifax, London, Hamilton, Waterloo, Calgary, Edmonton, Winnipeg, and Ottawa appear as individual mid-sized regional centres with one or more validated co-location clusters.

### Mexico (30 markets)

Mexico contributes 30 markets, including a mix of state-level aggregations (Nuevo León, Sinaloa, Chihuahua, San Luis Potosí, Aguascalientes, Querétaro, Guanajuato) and individual cities such as Juárez, Guadalupe, Morelia, Mérida, Mexicali, Culiacán, León, Acapulco de Juárez, Celaya, Mazatlán, Durango, Boca del Río, Hermosillo, Ensenada, Reynosa, Chihuahua, Cajeme, Zamora, Ahome, Uruapan, Coatzacoalcos, and Xalapa. Coverage reflects the distribution of large-format hypermarket chains (Walmart de México, Soriana, Chedraui) and the federal healthcare networks of IMSS, ISSSTE, and Cruz Roja Mexicana.

## Distribution by Tier

Of the four hundred markets, 368 (92.0 percent) contain at least one Tier 1 cluster, indicating that the great majority of top-ranked markets feature the full hypermarket–hardware–warehouse-club combination at least once. A smaller cohort — 35 markets — contains three or more Tier 1 clusters, and a handful of markets at the very top of the ranking contain ten or more Tier 1 clusters as a consequence of state- or province-level aggregation. Tier 2 clusters are widely distributed across the long tail: 223 markets contain at least one Tier 2 cluster, and Tier 2 frequently appears in markets without Tier 1 presence. Tier 3 clusters round out the lower portion of the ranking, typically appearing in markets where a hypermarket exists alongside civic infrastructure but no co-located hardware or warehouse club has yet been recorded.

Score distribution across the four hundred markets is heavily right-skewed. One market exceeds 200 points (Ontario, Canada, at 300.0), two more sit in the 100–199 range (Texas and Québec), ten sit between 50 and 99, sixty-seven sit between 25 and 49, and the remaining three hundred and twenty sit below 25.

## Civic Infrastructure

Civic anchor presence is the defining characteristic of the top four hundred markets: 398 of 400 (99.5 percent) include at least one medical or academic anchor in their co-location clusters. The dataset records 6,147 medical anchors and 4,537 education anchors across the constituent clusters of the four hundred markets. Medical anchors are dominated by general hospitals (with the bare label *hospital* occurring 455 times), supplemented by recognisable healthcare systems and named facilities such as Mount Sinai Hospital, University Hospital, LifeCare Hospitals, University Medical Center, and — in Mexico — IMSS, ISSSTE, Cruz Roja Mexicana, Poliplaza Médica, and Hospital de la Mujer.

Academic anchors range from major research universities to community-college branch campuses. Examples present in the top-twenty-five markets include the University of Ottawa, Carleton University, McMaster University, University of Toronto, the Claremont Colleges, Université Laval, Université de Montréal, McGill University, Rice University, Southern Methodist University, the University of Texas system, the University of Florida, the University of Missouri, the University of Nebraska Medical Center, the University of Colorado Anschutz Medical Campus, Wichita State University, and several state-system regional campuses. The co-occurrence of major healthcare campuses and large university medical centres in markets such as Aurora, Colorado; Omaha, Nebraska; Columbia, Missouri; and Houston, Texas is consistent with the broader pattern in which higher-education and tertiary-care infrastructure cluster within the same suburban service catchment.

Only two markets in the top four hundred lack a civic anchor of either kind — Oakdale, Pennsylvania (rank 281) and Portage, Michigan (rank 282) — both of which qualify on the strength of multiple Tier 1 retail clusters alone.

## Data Sources

- Co-location cluster dataset: 6,493 clusters across eighteen countries, derived from a continental retail-and-anchor co-location analysis run in 2026.
- Population reference: Kontur Population 2023 raster, distributed at H3 resolution 8 under CC BY 4.0.
- Retail point-of-interest data: OpenStreetMap contributors (ODbL 1.0) and Overture Maps Foundation (CDLA Permissive 2.0).
- Civic anchor data: Overture Maps Foundation healthcare and education datasets, supplemented with named-anchor extractions from OpenStreetMap.
- Metropolitan reference centroids: the sixty largest North American urban areas by population, used as anchor points for the metropolitan-distance multiplier.

## See Also

- *Top 400 Regional Markets — Europe* — companion list covering the European fifteen-country dataset.
- *Co-location Intelligence Overview* — methodology paper describing the underlying cluster-detection procedure.
- *Tier Nomenclature and Anchor Composition* — the controlled vocabulary defining Tier 1, Tier 2, and Tier 3 anchor combinations.
- *Regional Markets — Definition and Geographic Resolution* — the methodology note that defines the Regional Market unit and its relationship to settlement and metropolitan boundaries.
