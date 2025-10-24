# Stegarst

![GitHub branch status](https://img.shields.io/github/checks-status/eprislac/stegarst/master) |

---

Stegarst is a simple command-line tool for hiding and extracting text messages
within PNG images using steganography techniques, written in Rust.
It's also now a library that can be pulled into other rust projects.

## Features

- Hide text messages inside PNG images.
- Extract hidden text messages from PNG images.
- Simple and easy-to-use command-line interface.
- External crate for integration into other projects.

## Roadmap

- [ ] Support for other image formats (JPEG, BMP).
- [ ] Encryption of hidden messages for added security.
- [x] Convert to library, for integration into API services.

\*\* NOTICE:
Provided as-is, for educational purposes only. Use at your own risk.

## Acknowledgements

This initial idea and code from this project was in large part taken from the
2023 Medium article [Let's build a steganography CLI from scratch](https://medium.com/better-programming/lets-build-an-steganography-cli-from-scratch-f91e80de595c),
by [Andres Alejandro Coronel Rodrigues](https://medium.com/@andrescoronel1209?source=post_page---byline--f91e80de595c---------------------------------------).
It's not a direct fork of [his repo](https://github.com/ACR1209/rust-steganography), but still owes much to his work...
so much so, that I've included him as a co-author on this project.

Much thanks to Andres for writing the article and sharing his code!

## License

This project is licensed under the GNU General Public License v3.0. See
the [LICENSE](License.txt) file for details.
