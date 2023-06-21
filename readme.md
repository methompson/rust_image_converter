# rust_image_converter

An image converter plugin using Rust meant for compiling to Web Assembly.

My current file server uses ImageMagick to make conversions on the back end, but that has blown up the Docker image size to over 400 megabytes. It would also be nice to be able to reduce image upload size and also reduce load on my web server.

This plug in is meant to work within a web site and take images and convert to other sizes.

This is a Work in progress. Further details will be worked out over time.