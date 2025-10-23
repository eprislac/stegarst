# Stegarst

Stegarst is a simple command-line tool for hiding and extracting text messages
within PNG images using steganography techniques, written in Rust.

## Features

- Hide text messages inside PNG images.
- Extract hidden text messages from PNG images.
- Simple and easy-to-use command-line interface.

## Planned Features

- Support for other image formats (JPEG, BMP).
- Encryption of hidden messages for added security.
- Convert to library, for integration into API services.

## Acknowledgements

This project is in large part taken from the 2023 Medium article [Let's build a steganography CLI from scratch](https://medium.com/better-programming/lets-build-an-steganography-cli-from-scratch-f91e80de595c), by [Andres Alejandro Coronel Rodrigues](https://medium.com/@andrescoronel1209?source=post_page---byline--f91e80de595c---------------------------------------).
It's not a direct fork of [his repo](ACR1209/rust-steganography), (it took some minor updates to compile),
but still contains a lot of his work.

Much thanks to Andres for writing the article and sharing his code!

The unit tests are all mine.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](License.txt) file for details.
