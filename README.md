# Endfind

Endfind is a simple application designed to help you find the End Portal in Minecraft. It uses triangulation to calculate the location.

## Features

- Simple GUI
- Takes ender eye throw coordinates from your clipboard

## Installation

You can install Endfind using cargo:

```bash
cargo install endfind
```

## Usage

To start Endfind, run the following command:

```bash
endfind
```
> ⚠️ Ensure your cargo bin directory is in your PATH.

Once the application is running, press the "Start measurement" button and set your FOV to 30.
Then throw one ender eye, look directly at it and press F3+C to copy the coordinates to your clipboard.
After that throw a second ender eye, look at it and press F3+C again. The application will then calculate and display the location of the End Portal.

## License

This project is licensed under the GNU General Public License Version 3. See the [LICENSE](LICENSE) file for details.