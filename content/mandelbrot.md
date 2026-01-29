---
title: "Stumbling into the Mandelbrot..."
date: "01/28/26"
tagline: "and what it means to be an artist."
---

# Stumbling into the Mandelbrot

## Basics

A coordinate _c_ is said to be within the Mandelbrot set if the function $z_n = z_{n-1}^2 + c$ does *not* diverge to infinity when iterated. _c_ is a complex number, which means that it is represented by a real and imaginary component: $a + bi$.

So, let's plug in the value $1 + 0i$, starting at $z = 0$.

$$
\begin{aligned}
z_1 &= 0^2 + (1 + 0i) = 1 \\
z_2 &= 1^2 + (1 + 0i) = 2 \\
z_3 &= 2^2 + (1 + 0i) = 5 \\
z_4 &= 5^2 + (1 + 0i) = 26
\end{aligned}
$$
<div class="dot-stack">
  <span></span>
  <span></span>
  <span></span>
</div>

The coordinate $1 + 0i$ grows unbounded and, as such, is _outside_ the set. Let's look at the coordinate $-1 + 0i$.

$$
\begin{aligned}
z_1 &= 0^2 + (-1 + 0i) = -1 \\
z_2 &= (-1)^2 + (-1 + 0i) = 0 \\
z_3 &= 0^2 + (-1 + 0i) = -1 \\
z_4 &= (-1)^2 + (-1 + 0i) = 0 \\
\end{aligned}
$$
<div class="dot-stack">
  <span></span>
  <span></span>
  <span></span>
</div>

The coordinate $-1 + 0i$ is clearly stuck in a pattern, oscillating between $-1$ and $1$ forever. We can safely label $-1 + 0i$ as a point _within_ the set.

Given this definition, we can plot the complex number plane where each point _c_ is either within the set, black, or outside, white.

<img src="static/binary.webp" height="400">

## Escape Time

This binary classification is rather boring. It fails to express interesting [emergent behaviors](https://en.wikipedia.org/wiki/Mandelbrot_set#Image_gallery_of_a_zoom_sequence) of the system. A good place to start is with the _speed_ of divergence, that is, _how fast does z approach infinity?_ Let's define an escape radius, _r_, that represents the maximum distance that _z_ can stray from the origin and record the iteration _i_ for which the magnitude of _z_ surpasses _r_.

```rust
while i < max_iteration {
    let x2 = x * x;
    let y2 = y * y;

    if x2 + y2 > r {
        break;
    }

    y = 2.0 * x * y + y0;
    x = x2 - y2 + x0;
    i += + 1;
}
```

The escaped iteration _i_ can be mapped to the range `0..max_iteration` to create a grayscale image[^1].

```rust
let luminance = i as f32 / max_iteration as f32;
let color = Rgb::new(luminance, luminance, luminance);
```

<div class="image-row">
  <figure>
    <img src="static/greys-small.webp" style="height: 400px;">
    <figcaption><math><msup><mi>r</mi><mn>2</mn></msup><mo>=</mo><mn>4</mn></math></figcaption>
  </figure>
  <figure>
    <img src="static/greys.webp" style="height: 400px;">
    <figcaption><math><msup><mi>r</mi><mn>2</mn></msup><mo>=</mo><mn>10,000</mn></math></figcaption>
  </figure>
</div>

## Smooth Coloring

The escape time algorithm suffers from color banding where a group of coordinates trigger the break condition despite having different "escape velocities". In other words, two values can exceed the escape radius on the _same_ iteration with _different_ magnitudes.

I do not understand the [mathmatics](https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set#Continuous_\(smooth\)_coloring) behind interpolating _i_ but fortunately it is quite simple in practice.

```rust
let log_zn = (x * x + y * y).log() / 2.0;
let nu = (log_zn / 2f32.log()).log() / 2f32.log();
let si = iteration as f32 + 1.0 - nu;
```

Now, sustituting _i_ with _si_ results in a smooth color gradient. 

```rust
let luminance = si / max_iteration as f32;
let color = Rgb::new(luminance, luminance, luminance);
```

<div class="image-row">
  <figure>
    <img src="static/smooth-greys-small.webp" style="height: 400px;">
    <figcaption><math><msup><mi>r</mi><mn>2</mn></msup><mo>=</mo><mn>4</mn></math></figcaption>
  </figure>
  <figure>
    <img src="static/smooth-greys.webp" style="height: 400px;">
    <figcaption><math><msup><mi>r</mi><mn>2</mn></msup><mo>=</mo><mn>10,000</mn></math></figcaption>
  </figure>
</div>

## Color Palettes

Given _i_, the the corresponding color is a simple index into the palette.

```rust
let index = (i as f32 / max_iteration as f32) as usize;
let color = palette[index * palette.len()];
```

Given _si_, the color should be interpolated by the fractional component of _si_.

```rust
let index = (si / max_iteration as f32) as usize;
let c1 = palette[index];
let c2 = palette[(index + 1) % palette.len()];
let color = mix(c1, c2, si.fract());
```

By parameterizing the scale of the color, different features are extracted and moved around the palette.

```rust
let index = (si / (max_iteration as f32 * color_scale)) as usize;
```

<div class="image-row">
  <figure>
    <img src="static/chain1.webp" style="height: 400px;">
    <figcaption><math><mi>color_scale</mi><mo>=</mo><mn>1.0</mn></math></figcaption>
  </figure>
  <figure>
    <img src="static/chain2.webp" style="height: 400px;">
    <figcaption><math><mi>color_scale</mi><mo>=</mo><mn>12.0</mn></math></figcaption>
  </figure>
</div>

## So What?

There is something ineffible in the zoom of a fractal. The way in which self similar patterns emerge, re-emerge, disintegrate, _transform_ into unrecognizable yet familiar structures. A stream of change unrolls and you stumble into a melting reality but beating underneath is a thread of curiosity for what is just beyond the fold. A questioning answer, a meaning that is never understood. And that is a pure form of art. Abstract art. The kind of art that you work at. But beneath that still is the absolute horror of the immensity of the universe.

> Awake. Falling... Still falling, still awake? Never awake. Always falling. Darkness shrouds my body. Where are my hands? Did I ever have hands? Ay, to fall is to be still.

> How long have you been there, watching me? I move my hand over you but it does not block your bleeding glow. You must be imaginary. Ay, to imagine to to fall.

> Do you hate me? Do you feel like I feel? Do you fall? Silence. Ay, to hear is to fall.

> Do you feel me scraping at your walls? Yes, you are bigger now than you were. I am falling into you? Should I look away? Ay, to fall is to look away.

> Inevitable. Time is inevitable and I did not ask it to be. Are you time? Ay, to fall is to be time.

<img src="static/last-breath.webp" style="height: 800px;">

$$
z_n = z_{n-1}^2 + c
$$

[^1]: Note that increasing _r_ will increase the accuracy of the escape time algorithm, so large values are recommended.
