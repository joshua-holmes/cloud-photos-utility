# Cloud Photos Utility

*This project is very early in development*

This is a new GUI tool written in Rust with the intention of helping users transfer photos away from popular cloud providers, like Google Photos, and optionally import into a self-hosted service like Immich.

The scope of this project could change at any time, but right now I'm thinking of a feature set like this:

* Fix metadata from Google Photos Takeout exports.
    * Google Photos Takeout delivers images with the metadata (like location of photo, time it was taken, etc.) in JSON files, instead of attached to the photos. This program will read the metadata and attach it to the photos.
* Offer an option to convert image types, in case you have image types like HEIC that are not readable on every system.
* Import photos to an Immich server, if you have one setup.

Each of these features are standalone features, so you could use any one of them without using the others. If you have suggestions for features you'd like to see, please let me know.
