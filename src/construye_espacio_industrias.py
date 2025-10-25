import pandas as pd
import json

nodes = pd.read_csv("nodes.csv")
links = pd.read_csv("links.csv")
categories = pd.read_csv("categories.csv")


datos_espacio_producto = {
                "nodes" : json.loads(nodes.to_json(orient="records")),
                "links" : json.loads(links.to_json(orient="records")),
                "categories" : json.loads(categories.to_json(orient="records"))
}

with open("espacio_producto_format.json", "w") as file:
    json.dump(datos_espacio_producto, file,indent=4)
