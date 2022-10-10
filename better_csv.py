import json
import csv

counts = json.loads(open("./hashmap.json", "r").read())
with open('homes_count.csv', 'w', newline='') as csvfile:    
    writer = csv.writer(csvfile, delimiter=',', quoting=csv.QUOTE_NONE)
    writer.writerow(["lat", "long", "city", "area", "neigh", "zip", "income", "est", "price", "bed", "bath", "tree10m", "tree25m", "tree50m", "tree100m", "tree200m"])
    with open('homes.csv', 'r', newline='') as inp:
        lines = inp.readlines()
        for i,x in enumerate(lines):
            vals = x.split(",")
            vals[-1] = vals[-1].strip()
            writer.writerow(vals+[str(i) for i in counts[str(i)]])                        
