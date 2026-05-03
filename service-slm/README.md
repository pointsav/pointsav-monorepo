# 🚪 SERVICE-SLM
**Vendor:** PointSav Digital Systems™
**Standard:** The Doorman Protocol
**Tier:** 5 (API Gateway)

## I. ARCHITECTURAL MANDATE
This component operates strictly as an API Gateway. It is the secure port bridging an external Large Language Model (LLM) to the isolated Totebox Archive. 

It does not generate text. When queried, it fetches context vectors from the three sovereign ledgers (`service-email`, `service-people`, `service-content`), compiles a dense factual payload, and transmits it to the LLM. 
