# Mandelbrot/Julia set generator
Generates images from the [julia set](https://en.wikipedia.org/wiki/Julia_set).<br />
Output image is saved as `output.png`. Due to the big resolution of the output image,
it is recommended to run with `--release` flag on, or set the resolution to a smaller number.

## Pallete
A default pallete is provided, but you can set your 
own colors by modifying the `pallete` variable on `main.rs:71`.

## Dimentions
You can set your own dimentions by changing the variables `width` and `height` on `main.rs:68`.
Soon, there will be a command argument for that.

## Other
Scale, x/y offsets, as well as the constants can be customized with 
the responding variables in `main.rs:82`. As with dimentions, they will be 
customizable via a command argument
