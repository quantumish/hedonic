import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import geopandas as gpd
from shapely.geometry import Point, Polygon

pd.set_option('display.max_rows', None)  
df = pd.read_csv("./homes_count.csv")
df = df.loc[df["bed"] < 6]
df = df.loc[df["area"] < 6000]
df = df.loc[df["bath"] < 6]
df = df.loc[~df["est"].isnull()]

street_map = gpd.read_file("./geo_export_62f3aea2-a6f5-43e2-b7c2-384f6b2cc284.shp")
# designate coordinate system
# crs = {"init":"espc:4326"} 
geometry = [Point(xy) for xy in zip(df["long"], df["lat"])]# create GeoPandas dataframe
geo_df = gpd.GeoDataFrame(df,
 # crs = crs,
 geometry = geometry)

fig, ax = plt.subplots(figsize=(15,15))
street_map.plot(ax=ax, alpha=0.4,color="grey")
geo_df.plot(ax=ax,alpha=0.5, legend=True,markersize=10)
# plt.xlim(-74.02,-73.925)
# plt.ylim( 40.7,40.8)

# gdf = gpd.GeoDataFrame(df, geometry=gpd.points_from_xy(tmp.long, tmp.lat))
#world = gpd.read_file(gpd.datasets.get_path('naturalearth_lowres'))

# ax = world[world.continent == 'North America'].plot(color='white', edgecolor='black')
# gdf.plot(ax=ax, color='red')

plt.show()
# df.to_csv("./cleaned_homes.csv")
