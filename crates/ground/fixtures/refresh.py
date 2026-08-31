#!/usr/bin/env python3
"""Rebuild `sections.json` from the county's live layer, for the ground the corpus stands on.

Deliberately not the whole county. This corpus's stated use of Allen County's spatial layers
is that it quotes facts about named units rather than redistributing a dataset — see
`.yidam/decisions/auditor-parcels-access-terms.yml` — so the fixture carries the sections the
corpus has actually cited and grows when it cites more.

    mise run ground-fixture

Reads every `place` and `site` node's `coordinates` or `centroid`, projects each onto
EPSG:3734, and keeps the section polygons those points land in.
"""
import json, math, re, sys, urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
LAYER = "https://gis.allencountyohio.com/arcgis/rest/services/AllenCountyGIS/MapServer/55/query"

# EPSG:3734 — NAD83 / Ohio North (US survey feet). Mirrors crates/ground/src/project.rs; the
# Rust side is the one under test, and this exists so the fixture can be rebuilt without it.
A, INV_F = 6378137.0, 298.257222101
E = math.sqrt(2 / INV_F - (1 / INV_F) ** 2)
LAT1, LAT2 = math.radians(41 + 42 / 60), math.radians(40 + 26 / 60)
LAT0, LON0 = math.radians(39 + 40 / 60), math.radians(-82.5)
FE, M2FT = 1968500.0, 3937 / 1200
_m = lambda p: math.cos(p) / math.sqrt(1 - E * E * math.sin(p) ** 2)
_t = lambda p: math.tan(math.pi / 4 - p / 2) / ((1 - E * math.sin(p)) / (1 + E * math.sin(p))) ** (E / 2)
N = (math.log(_m(LAT1)) - math.log(_m(LAT2))) / (math.log(_t(LAT1)) - math.log(_t(LAT2)))
F = _m(LAT1) / (N * _t(LAT1) ** N)
RHO0 = A * F * _t(LAT0) ** N


def project(lat, lon):
    rho = A * F * _t(math.radians(lat)) ** N
    th = N * (math.radians(lon) - LON0)
    return FE + rho * math.sin(th) * M2FT, (RHO0 - rho * math.cos(th)) * M2FT


def inside(x, y, ring):
    c, j = False, len(ring) - 1
    for i in range(len(ring)):
        xi, yi = ring[i][0], ring[i][1]
        xj, yj = ring[j][0], ring[j][1]
        if (yi > y) != (yj > y) and x < (xj - xi) * (y - yi) / (yj - yi) + xi:
            c = not c
        j = i
    return c


def main():
    url = f"{LAYER}?where=1%3D1&outFields=*&returnGeometry=true&outSR=3734&f=json"
    with urllib.request.urlopen(url, timeout=180) as r:
        data = json.load(r)
    if data.get("exceededTransferLimit"):
        sys.exit("layer 55 now exceeds one page; this script must learn to page")
    feats = [f for f in data["features"] if f["attributes"]["SHAPE.area"] > 1]

    points = []
    for p in sorted((ROOT / ".yidam/corpus").glob("*/*.yml")):
        m = re.search(r"^\s+(?:coordinates|centroid):\s*\"?([-\d.]+),\s*([-\d.]+)\"?\s*$",
                      p.read_text(), re.M)
        if m:
            points.append((float(m.group(1)), float(m.group(2))))

    keep, out = set(), []
    for lat, lon in points:
        x, y = project(lat, lon)
        for f in feats:
            if any(inside(x, y, r) for r in f["geometry"]["rings"]):
                keep.add(f["attributes"]["T_R_S"])
    for f in feats:
        a = f["attributes"]
        if a["T_R_S"] not in keep:
            continue
        out.append({
            "trs": a["T_R_S"], "township": int(a["Township"]), "range": int(a["Range"]),
            "section": int(a["TEXTSTRING"]), "area_sqft": round(a["SHAPE.area"], 1),
            "rings": [[[round(p[0], 2), round(p[1], 2)] for p in r] for r in f["geometry"]["rings"]],
        })
    out.sort(key=lambda d: (d["township"], d["range"], d["section"]))
    dest = Path(__file__).with_name("sections.json")
    dest.write_text(json.dumps({
        "source": "AllenCountyGIS/MapServer/55 Section Numbers", "srs": "EPSG:3734",
        "retrieved": data.get("retrieved", "see the catalog entry"),
        "note": "Only the sections this corpus stands on. Rebuild with `mise run ground-fixture`.",
        "sections": out,
    }, indent=1))
    print(f"{len(points)} points -> {len(out)} sections -> {dest}")


if __name__ == "__main__":
    main()
