"""The base of the URG exception hierarchy."""


class UrgError(Exception):
    """A URG stage refused its inputs or failed in a way that it can report."""
