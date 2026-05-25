# DESIGN TOKENS: Routing Protocol

This document defines the authoritative routing rules for design tokens and assets within the Woodfine/PointSav ecosystem.

## Routing Rules
To maintain clear separation between core platform logic and customer-specific branding, all tokens must follow these routing rules:

1. **Generic Tokens:** 
   * **Definition:** Baseline UI components, agnostic spatial primitives, and non-branded functional variables.
   * **Route:** `/pointsav-design-system`
2. **PointSav Branded Tokens:** 
   * **Definition:** Direct brand assets (logos, signets) and color palettes specifically for the PointSav vendor identity.
   * **Route:** `/pointsav-media-assets`
3. **Woodfine Branded Tokens:** 
   * **Definition:** Customer-specific overrides, institutional palettes (Wall Street aesthetic), and corporate wordmarks.
   * **Route:** `/woodfine-media-assets`

## Maintenance
Any new design token created during application development must be audited against these rules and extracted to the appropriate repository. Hard-coded CSS hex values are prohibited in project-specific source code.
