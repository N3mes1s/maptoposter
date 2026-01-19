"""Logging configuration for MapToPoster."""

import logging
import sys
from typing import Optional


def setup_logging(
    level: str = "INFO",
    format_string: Optional[str] = None,
    json_format: bool = False
) -> logging.Logger:
    """
    Configure and return the application logger.

    Args:
        level: Logging level (DEBUG, INFO, WARNING, ERROR, CRITICAL)
        format_string: Custom format string (optional)
        json_format: Use JSON-structured logging for production

    Returns:
        Configured logger instance
    """
    logger = logging.getLogger("maptoposter")

    # Avoid duplicate handlers
    if logger.handlers:
        return logger

    logger.setLevel(getattr(logging, level.upper(), logging.INFO))

    # Create handler
    handler = logging.StreamHandler(sys.stdout)
    handler.setLevel(logger.level)

    # Set format
    if format_string:
        formatter = logging.Formatter(format_string)
    elif json_format:
        formatter = logging.Formatter(
            '{"timestamp": "%(asctime)s", "level": "%(levelname)s", '
            '"module": "%(module)s", "message": "%(message)s"}'
        )
    else:
        formatter = logging.Formatter(
            "%(asctime)s - %(name)s - %(levelname)s - %(message)s",
            datefmt="%Y-%m-%d %H:%M:%S"
        )

    handler.setFormatter(formatter)
    logger.addHandler(handler)

    # Prevent propagation to root logger
    logger.propagate = False

    return logger


def get_logger(name: str = "maptoposter") -> logging.Logger:
    """
    Get a logger instance for the given name.

    Args:
        name: Logger name (will be prefixed with 'maptoposter.')

    Returns:
        Logger instance
    """
    if name == "maptoposter":
        return logging.getLogger(name)
    return logging.getLogger(f"maptoposter.{name}")


# Initialize default logger on import
logger = setup_logging()
