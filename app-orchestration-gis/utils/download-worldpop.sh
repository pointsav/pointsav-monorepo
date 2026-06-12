#!/bin/bash
# Batch download WorldPop 2026 Constrained grids
# Pattern: https://data.worldpop.org/GIS/Population/Global_2015_2030/R2025A/2026/{ISO}/v1/100m/constrained/{iso}_pop_2026_CN_100m_R2025A_v1.tif

ISO3_CODES=("USA" "CAN" "MEX" "GBR" "DEU" "FRA" "NLD" "AUT" "PRT" "GRC" "DNK" "ISL" "POL")
OUT_DIR="/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-census/raw/"

mkdir -p $OUT_DIR

for ISO3 in "${ISO3_CODES[@]}"; do
    ISO_LOWER=$(echo "$ISO3" | tr '[:upper:]' '[:lower:]')
    URL="https://data.worldpop.org/GIS/Population/Global_2015_2030/R2025A/2026/${ISO3}/v1/100m/constrained/${ISO_LOWER}_pop_2026_CN_100m_R2025A_v1.tif"
    echo "Downloading ${ISO3}..."
    wget -O "${OUT_DIR}/${ISO_LOWER}_pop_2026.tif" "$URL"
done
