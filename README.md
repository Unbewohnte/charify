# charify - a command line utility to convert an image to an array of characters

## Usage

`charify [OPTIONS] --image <image> --destination <destination>`

where 

- `image` is the path to an existing image
- `destination` is the path to a newly created text file

and 

OPTIONS:
- `-c, --charset <charset>`
  Set a new character set to use [default: "[' ', '░', '▒', '▓', '█']"]

- `-h, --help`
  Print help information

- `-r, --new_dimensions <new_dimensions>`
  Resize source image to specified dimensions

### Examples

- `charify ~/wallpapers/image.png ~/wallpapers/my_wallpaper_but_in_text.txt` 
- `charify image.jpg textfile.txt -r 500x450`
- `charify img.png charified.txt -c abcdefg`