// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#version 420 core

// Squared distance from origin to terminate iteration
// Set to 4 for standard behaviour.
// Higher values blend escape band
#define ESCAPE_BOUNDARY 18.0
// Color rotation speed.
#define FREQUENCY 0.8

// Escap-steps per color rotation band
#define COL_BAND_SCALE 1.0

in highp vec4 gl_FragCoord;
out vec4 Target0;


// Naive approach to guaranteeing double precision
dvec2 vec2_to_dvec2(vec2 floatvec2);         
dvec2 viewport_coord_to_complex(dvec2 coord);

dvec2 mouse_pos_to_complex();
dvec2 fragment_coord_to_complex();

vec3 hsv2rgb( vec3 c);

// Values defined by the front-end
layout (std140) uniform MandelShaderUniforms {

    // View-port values
    uniform dvec2 u_Center;     // Center-pixel location
    uniform dvec2 u_Dimension;  // width-height
    uniform dvec2 u_Resolution; // Pixel-resolution of viewport

    // exploration controlls
    uniform vec2 u_MousePos;    // Used for julia-set, and hybrids
    uniform float u_Time;       // Time elapsed in seconds
    uniform int u_MaxIteration;
    uniform int u_MandelMode;   // (A bit hacky, TODO) MandelBrot / Julia swap
};



void main() {

    // load up our iteration values
    dvec2 complex_mouse = mouse_pos_to_complex();
    dvec2 complex_fragment = fragment_coord_to_complex();
    dvec2 z, c;

    // Determine which set we are rendering
    switch (u_MandelMode) {

        // julia
    case 1:
        z = complex_fragment;
        c = complex_mouse;
        break;

        // hybrid
    case 2:
        z = dvec2(0.0, 0.0) + complex_mouse * 0.9;
        c = complex_fragment;
        break;

        // mandelbrot
    case 0:
    default:
        z = dvec2(0.0, 0.0);
        c = complex_fragment;
        break;

    }

    int step_count;
    dvec2 tmp;
    for (step_count = 0; step_count < u_MaxIteration; step_count++) {

        // Vast majority of performance gets absorbed here
        // TODO micro-optimise 2019-03-30
        // `z = z * z + c`, with z and c being complex numbers

        tmp.x = z.x*z.x - z.y*z.y;
        tmp.y = 2.0 * z.x*z.y;
        z.x = tmp.x;
        z.y = tmp.y;
        z.x += c.x;
        z.y += c.y;

        if((z.x*z.x + z.y*z.y) > ESCAPE_BOUNDARY) {
            break;
        }
    }


    // steps-color mapping
    if (step_count == u_MaxIteration) { 
        // unescaped color
        Target0 =  vec4(1.0, 1.0, 1.0, 1.0);
    } else {

        // Follow [this link](https://www.linas.org/art-gallery/escape/smooth.html) for more info
        float dist = float(length(z));

        float two = float(2.0);
        dist  = log(log(dist)) / log(two);

        float val = float(step_count) - dist;
        float hue = val/COL_BAND_SCALE - u_Time * FREQUENCY;

        Target0 = vec4(hsv2rgb(vec3((hue), 1.0, 1.0)), 1.0);
    }
}

// enhance...
dvec2 vec2_to_dvec2(vec2 floatvec2) {
    return dvec2(floatvec2);
}

// center yourself, THEN grow your mind, then you will be moved on the plane of imagination
dvec2 viewport_coord_to_complex(dvec2 coord) {

    // set the origin to the center of the screen
    dvec2 result =  dvec2(coord) - u_Resolution/2.0;

    // Scale down to a 1.0 by 1.0 view of the complex plane
    result /= u_Resolution;

    // match the real and im dimensions to user-input
    result *= u_Dimension;

    // shift the center reference to where the user asked it to be
    result += u_Center;
    return result;
}


// TODO set it up so that responsibility between mandel and julia decoupled
//      at the moment seperate areoas are tightly coupled
dvec2 fragment_coord_to_complex() {
    return viewport_coord_to_complex(gl_FragCoord.xy);
}

dvec2 mouse_pos_to_complex() {
    return viewport_coord_to_complex(u_MousePos);
}


// hue, saturation, value to reg, green, blue
vec3 hsv2rgb(vec3 c) {
    vec4 K =  vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www );
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}
