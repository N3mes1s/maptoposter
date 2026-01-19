#!/usr/bin/env python3
"""
Command-line interface for MapToPoster.
Backward-compatible wrapper using the refactored module structure.
"""

import argparse
import sys
from datetime import datetime
from pathlib import Path

from src.maptoposter.config import settings
from src.maptoposter.core.geocoding import get_coordinates
from src.maptoposter.core.poster_generator import PosterGenerator, PosterRequest
from src.maptoposter.themes.loader import load_theme, get_available_themes, get_all_themes_with_details
from src.maptoposter.exceptions import GeocodingError, ThemeNotFoundError


def generate_output_filename(city: str, theme_name: str) -> Path:
    """Generate unique output filename with city, theme, and datetime."""
    settings.POSTERS_DIR.mkdir(exist_ok=True)
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    city_slug = city.lower().replace(' ', '_')
    filename = f"{city_slug}_{theme_name}_{timestamp}.png"
    return settings.POSTERS_DIR / filename


def print_examples():
    """Print usage examples."""
    print("""
City Map Poster Generator
=========================

Usage:
  python cli.py --city <city> --country <country> [options]

Examples:
  # Iconic grid patterns
  python cli.py -c "New York" -C "USA" -t noir -d 12000           # Manhattan grid
  python cli.py -c "Barcelona" -C "Spain" -t warm_beige -d 8000   # Eixample district grid

  # Waterfront & canals
  python cli.py -c "Venice" -C "Italy" -t blueprint -d 4000       # Canal network
  python cli.py -c "Amsterdam" -C "Netherlands" -t ocean -d 6000  # Concentric canals

  # Organic old cities
  python cli.py -c "Tokyo" -C "Japan" -t japanese_ink -d 15000    # Dense organic streets
  python cli.py -c "Rome" -C "Italy" -t warm_beige -d 8000        # Ancient street layout

  # List themes
  python cli.py --list-themes

Options:
  --city, -c        City name (required)
  --country, -C     Country name (required)
  --theme, -t       Theme name (default: feature_based)
  --distance, -d    Map radius in meters (default: 15000)
  --list-themes     List all available themes

Distance guide:
  4000-6000m   Small/dense cities (Venice, Amsterdam old center)
  8000-12000m  Medium cities, focused downtown (Paris, Barcelona)
  15000-20000m Large metros, full city view (Tokyo, Mumbai)
""")


def list_themes():
    """List all available themes with descriptions."""
    themes = get_all_themes_with_details()
    if not themes:
        print("No themes found in 'themes/' directory.")
        return

    print("\nAvailable Themes:")
    print("-" * 60)
    for theme in themes:
        print(f"  {theme.get('id', 'unknown')}")
        print(f"    {theme.get('name', theme.get('id', 'Unknown'))}")
        if theme.get('description'):
            print(f"    {theme['description']}")
        print()


def main():
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(
        description="Generate beautiful map posters for any city",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python cli.py --city "New York" --country "USA"
  python cli.py --city Tokyo --country Japan --theme midnight_blue
  python cli.py --city Paris --country France --theme noir --distance 15000
  python cli.py --list-themes
        """
    )

    parser.add_argument('--city', '-c', type=str, help='City name')
    parser.add_argument('--country', '-C', type=str, help='Country name')
    parser.add_argument('--theme', '-t', type=str, default='feature_based',
                        help='Theme name (default: feature_based)')
    parser.add_argument('--distance', '-d', type=int, default=15000,
                        help='Map radius in meters (default: 15000)')
    parser.add_argument('--list-themes', action='store_true',
                        help='List all available themes')

    args = parser.parse_args()

    # If no arguments provided, show examples
    if len(sys.argv) == 1:
        print_examples()
        sys.exit(0)

    # List themes if requested
    if args.list_themes:
        list_themes()
        sys.exit(0)

    # Validate required arguments
    if not args.city or not args.country:
        print("Error: --city and --country are required.\n")
        print_examples()
        sys.exit(1)

    # Validate theme exists
    available_themes = get_available_themes()
    if args.theme not in available_themes:
        print(f"Error: Theme '{args.theme}' not found.")
        print(f"Available themes: {', '.join(available_themes)}")
        sys.exit(1)

    print("=" * 50)
    print("City Map Poster Generator")
    print("=" * 50)

    try:
        # Load theme
        theme = load_theme(args.theme)
        print(f"✓ Loaded theme: {theme.get('name', args.theme)}")
        if theme.get('description'):
            print(f"  {theme['description']}")

        # Get coordinates
        print("\nLooking up coordinates...")
        coords = get_coordinates(args.city, args.country)
        print(f"✓ Coordinates: {coords[0]:.4f}, {coords[1]:.4f}")

        # Generate poster
        print(f"\nGenerating map for {args.city}, {args.country}...")
        generator = PosterGenerator(theme)
        request = PosterRequest(
            city=args.city,
            country=args.country,
            theme_name=args.theme,
            distance=args.distance,
            dpi=settings.OUTPUT_DPI
        )

        def progress_handler(progress):
            print(f"  [{int(progress.progress * 100):3d}%] {progress.message}")

        buffer = generator.generate(request, coords, progress_handler)

        # Save to file
        output_file = generate_output_filename(args.city, args.theme)
        with open(output_file, 'wb') as f:
            f.write(buffer.getvalue())

        print("\n" + "=" * 50)
        print(f"✓ Poster saved as {output_file}")
        print("=" * 50)

    except GeocodingError as e:
        print(f"\n✗ Geocoding Error: {e}")
        sys.exit(1)
    except ThemeNotFoundError as e:
        print(f"\n✗ Theme Error: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"\n✗ Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
