#version 330 core
 vec3 hsv2rgb( vec3 c);
bool fast_check( vec2 z);

in  vec4 gl_FragCoord;


layout (std140) uniform Mandel {
  uniform  vec2 u_MousePos;
  uniform  vec2 u_Center;
  uniform  vec2 u_Dimensions;
  uniform float u_Time;
  uniform float u_Zoom;
};

out  vec4 Target0;

 vec2 mouse_pos_to_im() {
  return u_MousePos / 1077 * 4.0 - 2.0;
}

 vec2 frag_coord_to_im() {
   vec2 ret = gl_FragCoord.xy - u_Dimensions/2.0;
  ret /= (240 * u_Zoom);
  ret -= (u_Center);
  return ret;
}

void main() {

    vec2 c = frag_coord_to_im().xy;
  // vec2 c = mouse_pos_to_im();

   


   vec2 z = vec2(0.0, 0.0);


  int max_iter = 100;

   vec2 D =  vec2(1.0, 1.0);
  float i;
  for (i = 0; i < max_iter; i++) {

    // z = z * z + c;
    z =  vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;

    // a different fast-check
    // if((pow(D.x, 2) + pow(D.y, 2)) < 0.001) {
    //   Target0 =  vec4(i/max_iter, 0.0, 0.0, 1.0);
    //   return;
    // }


    //  vec2 tmp;
    // tmp.x = D.x*z.x - D.y*z.y;
    // tmp.y = D.x*z.y + D.y*z.x;
    // D.x = 2.0*tmp.x;
    // D.y = 2.0*tmp.y;

    if(length(z) >= 5.0) {
      break;
    }
    // if (!fast_check(z)) { return;}
  }

  if (i == max_iter) {
    Target0 =  vec4(1.0, 1.0, 1.0, 1.0);
  } else {
    float dist = length(z) * length(z);

    dist  = log(log(dist))/log(2.0);
    float val = i - dist;
    // float val = i/ float(max_iter);
    Target0 =  vec4(hsv2rgb( vec3((val*0.1 - u_Time/5.0), 1.0, 1.0 )), 1.0);
  }
}


// dirty check that coveres points close enough to an attractor
bool fast_check( vec2 Z) {
    float r = sqrt(pow(Z.x - 0.25, 2) + pow(Z.y, 2));
    if (Z.x < r - 2 * pow(r, 2) + 0.25) {
        Target0 =  vec4(0, 1.0, 1.0, 1.0);
        return false;
    }

    if (pow(Z.x + 1, 2) + pow(Z.y, 2) < 0.0625) {
        Target0 =  vec4(1.0, 1.0, 1.0, 1.0);
        return false;
    }

    return true;
}



// hue-rotates an rgb-value
 vec3 hsv2rgb( vec3 c) {
     vec4 K =  vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
     vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www );
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}
