"""Road styling based on highway type hierarchy."""

from typing import List, Dict, Any

import networkx as nx

from ..logging_config import get_logger

logger = get_logger("rendering.roads")

# Road width constants (in points)
ROAD_WIDTH_MOTORWAY = 1.2
ROAD_WIDTH_PRIMARY = 1.0
ROAD_WIDTH_SECONDARY = 0.8
ROAD_WIDTH_TERTIARY = 0.6
ROAD_WIDTH_RESIDENTIAL = 0.4

# Default road color (fallback)
DEFAULT_ROAD_COLOR = "#3A3A3A"


def get_edge_colors_by_type(G: nx.MultiDiGraph, theme: Dict[str, Any]) -> List[str]:
    """
    Assigns colors to edges based on road type hierarchy.

    Colors are determined by the OSM highway tag, with major roads
    (motorways, primaries) typically getting darker/more prominent colors.

    Args:
        G: NetworkX MultiDiGraph from OSMnx containing road network
        theme: Theme dictionary with road color definitions. Expected keys:
            - road_motorway: Color for motorways
            - road_primary: Color for primary/trunk roads
            - road_secondary: Color for secondary roads
            - road_tertiary: Color for tertiary roads
            - road_residential: Color for residential/local roads
            - road_default: Fallback color for unknown road types

    Returns:
        List of color strings corresponding to each edge in the graph
    """
    edge_count = G.number_of_edges()
    logger.debug(f"Assigning colors to {edge_count} road edges")

    edge_colors: List[str] = []

    for u, v, data in G.edges(data=True):
        highway = data.get("highway", "unclassified")

        # Handle list of highway types (take the first one)
        if isinstance(highway, list):
            highway = highway[0] if highway else "unclassified"

        # Assign color based on road type using .get() for safety
        if highway in ["motorway", "motorway_link"]:
            color = theme.get("road_motorway", DEFAULT_ROAD_COLOR)
        elif highway in ["trunk", "trunk_link", "primary", "primary_link"]:
            color = theme.get("road_primary", DEFAULT_ROAD_COLOR)
        elif highway in ["secondary", "secondary_link"]:
            color = theme.get("road_secondary", DEFAULT_ROAD_COLOR)
        elif highway in ["tertiary", "tertiary_link"]:
            color = theme.get("road_tertiary", DEFAULT_ROAD_COLOR)
        elif highway in ["residential", "living_street", "unclassified"]:
            color = theme.get("road_residential", DEFAULT_ROAD_COLOR)
        else:
            color = theme.get("road_default", DEFAULT_ROAD_COLOR)

        edge_colors.append(color)

    logger.debug(f"Assigned colors to {len(edge_colors)} edges")
    return edge_colors


def get_edge_widths_by_type(G: nx.MultiDiGraph) -> List[float]:
    """
    Assigns line widths to edges based on road type.

    Major roads (motorways, primaries) get thicker lines to create
    visual hierarchy in the map.

    Args:
        G: NetworkX MultiDiGraph from OSMnx containing road network

    Returns:
        List of line width floats corresponding to each edge in the graph

    Width hierarchy:
        - Motorways: 1.2pt
        - Primary/Trunk: 1.0pt
        - Secondary: 0.8pt
        - Tertiary: 0.6pt
        - Residential/Other: 0.4pt
    """
    edge_count = G.number_of_edges()
    logger.debug(f"Assigning widths to {edge_count} road edges")

    edge_widths: List[float] = []

    for u, v, data in G.edges(data=True):
        highway = data.get("highway", "unclassified")

        if isinstance(highway, list):
            highway = highway[0] if highway else "unclassified"

        # Assign width based on road importance
        if highway in ["motorway", "motorway_link"]:
            width = ROAD_WIDTH_MOTORWAY
        elif highway in ["trunk", "trunk_link", "primary", "primary_link"]:
            width = ROAD_WIDTH_PRIMARY
        elif highway in ["secondary", "secondary_link"]:
            width = ROAD_WIDTH_SECONDARY
        elif highway in ["tertiary", "tertiary_link"]:
            width = ROAD_WIDTH_TERTIARY
        else:
            width = ROAD_WIDTH_RESIDENTIAL

        edge_widths.append(width)

    logger.debug(f"Assigned widths to {len(edge_widths)} edges")
    return edge_widths
