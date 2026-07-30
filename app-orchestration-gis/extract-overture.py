import duckdb

# Configuration for Overture Cloud Query
# Target: Filter Overture places for Canada as a pilot
OUTPUT_PARQUET = "/srv/foundry/deployments/cluster-totebox-personnel-1/service-fs/service-mobility/filtered-places-CAN.parquet"

def extract_overture_places(country_code="CA"):
    print(f"Executing cloud-filter for Overture Places: {country_code}...")
    
    # DuckDB query using httpfs to query S3 directly
    # We filter by country code and export to local Parquet
    query = f"""
    COPY (
        SELECT * 
        FROM read_parquet('s3://overturemaps-us-west-2/release/2024-05-16.0/theme=places/type=*/*')
        WHERE admin_country = '{country_code}'
    ) TO '{OUTPUT_PARQUET}' (FORMAT PARQUET);
    """
    
    con = duckdb.connect()
    con.execute("INSTALL httpfs; LOAD httpfs;")
    con.execute("INSTALL spatial; LOAD spatial;")
    con.execute(query)
    
    print(f"Places exported to {OUTPUT_PARQUET}")

if __name__ == "__main__":
    extract_overture_places()
