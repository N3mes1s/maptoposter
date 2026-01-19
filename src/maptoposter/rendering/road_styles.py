"""Road styling based on highway type hierarchy."""

from typing import List, Dict, Any


def get_edge_colors_by_type(G, theme: Dict[str, Any]) -> List[str]:
    """
    Assigns colors to edges based on road type hierarchy.

    Args:
        G: NetworkX graph from OSMnx
        theme: Theme dictionary with road color definitions

    Returns:
        List of colors corresponding to each edge in the graph
    """
    edge_colors = []

    for u, v, data in G.edges(data=True):
        highway = data.get('highway', 'unclassified')

        # Handle list of highway types (take the first one)
        if isinstance(highway, list):
            highway = highway[0] if highway else 'unclassified'

        # Assign color based on road type
        if highway in ['motorway', 'motorway_link']:
            color = theme['road_motorway']
        elif highway in ['trunk', 'trunk_link', 'primary', 'primary_link']:
            color = theme['road_primary']
        elif highway in ['secondary', 'secondary_link']:
            color = theme['road_secondary']
        elif highway in ['tertiary', 'tertiary_link']:
            color = theme['road_tertiary']
        elif highway in ['residential', 'living_street', 'unclassified']:
            color = theme['road_residential']
        else:
            color = theme['road_default']

        edge_colors.append(color)

    return edge_colors


def get_edge_widths_by_type(G) -> List[float]:
    """
    Assigns line widths to edges based on road type.
    Major roads get thicker lines.

    Args:
        G: NetworkX graph from OSMnx

    Returns:
        List of line widths corresponding to each edge
    """
    edge_widths = []

    for u, v, data in G.edges(data=True):
        highway = data.get('highway', 'unclassified')

        if isinstance(highway, list):
            highway = highway[0] if highway else 'unclassified'

        # Assign width based on road importance
        if highway in ['motorway', 'motorway_link']:
            width = 1.2
        elif highway in ['trunk', 'trunk_link', 'primary', 'primary_link']:
            width = 1.0
        elif highway in ['secondary', 'secondary_link']:
            width = 0.8
        elif highway in ['tertiary', 'tertiary_link']:
            width = 0.6
        else:
            width = 0.4

        edge_widths.append(width)

    return edge_widths
