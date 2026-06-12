import json
import math
from collections import defaultdict

class ClusterFilter:
    def __init__(self, clusters_meta_path, threshold_km=150.0):
        with open(clusters_meta_path, 'r') as f:
            data = json.load(f)
        
        self.clusters = []
        self.buckets = defaultdict(list)
        self.threshold_km = threshold_km
        
        # Latitude: 1 degree approx 111km
        # Longitude: 1 degree approx 111km * cos(lat)
        # At 150km, we need at least 2 degrees latitude buffer.
        # At 60 degrees latitude (Canada/Nordics), cos(60)=0.5, so 1 degree lon approx 55km.
        # 150km would be approx 2.7 degrees.
        # To be safe globally, we'll use a 3-degree buffer for buckets.
        
        buffer = math.ceil(threshold_km / 50.0) # approx degrees at high lat
        if buffer < 2: buffer = 2

        for c in data:
            item = (c['lon'], c['lat'])
            self.clusters.append(item)
            lon, lat = c['lon'], c['lat']
            
            # Populate buckets
            for i in range(int(lon) - buffer, int(lon) + buffer + 1):
                for j in range(int(lat) - buffer, int(lat) + buffer + 1):
                    self.buckets[(i, j)].append(item)
        
    def is_active(self, lon, lat):
        """Returns True if the point is within threshold_km of any cluster centroid."""
        bucket_key = (int(lon), int(lat))
        candidates = self.buckets.get(bucket_key, [])
        
        for c_lon, c_lat in candidates:
            if self.haversine_km(lon, lat, c_lon, c_lat) <= self.threshold_km:
                return True
        return False

    @staticmethod
    def haversine_km(lon1, lat1, lon2, lat2):
        R = 6371.0
        phi1, phi2 = math.radians(lat1), math.radians(lat2)
        dphi = math.radians(lat2 - lat1)
        dlambda = math.radians(lon2 - lon1)
        a = math.sin(dphi / 2)**2 + \
            math.cos(phi1) * math.cos(phi2) * math.sin(dlambda / 2)**2
        return 2 * R * math.atan2(math.sqrt(a), math.sqrt(1 - a))

if __name__ == "__main__":
    import os
    # Adjust path if running from utils/
    path = '../www/data/clusters-meta.json'
    if not os.path.exists(path): 
        path = '/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json'
    
    cf = ClusterFilter(path, threshold_km=150.0)
    print(f"Loaded {len(cf.clusters)} clusters into {len(cf.buckets)} buckets.")
    # Test point: Sherwood Park Walmart (53.5689, -113.2792)
    print(f"Is Sherwood Park active (150km)? {cf.is_active(-113.2792, 53.5689)}")
