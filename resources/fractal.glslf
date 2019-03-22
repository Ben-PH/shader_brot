#version 420 core
vec3 hsv2rgb( vec3 c);
bool fast_check( dvec2 z);
dvec2 coord_to_im(dvec2 coord);

in  vec4 gl_FragCoord;


layout (std140) uniform Mandel {
  uniform dvec2 u_MousePos;
  uniform dvec2 u_Center;
  uniform dvec2 u_Dimension;
  uniform dvec2 u_Resolution;
  uniform float u_Time;
  uniform int u_MaxIter;
  uniform int u_IsMandel;
};

out  vec4 Target0;

dvec2 get_c() {
  dvec2 ret;
  if(u_IsMandel != 0) {
    ret = coord_to_im(gl_FragCoord.xy);
  } else {
    ret = coord_to_im(u_MousePos);
  }
  
  return ret;
}

dvec2 get_z() {
  dvec2 ret;
  if(u_IsMandel != 0) {
    ret = dvec2(0.0, 0.0);
  } else {
    ret = coord_to_im(gl_FragCoord.xy);
  }
  
  return ret;
}

dvec2 coord_to_im(dvec2 coord) {
  dvec2 ret = coord - u_Resolution/2.0;
  ret /= u_Resolution/(u_Dimension);
  ret += u_Center;
  // ret -= u_Dimension/2.0;
  // ret -= u_Center/2.0;
  // dvec2 scale = u_Resolution/(dvec2(3.5, 2.0));
  // ret /= scale;
  return ret;
}

void main() {
  
  dvec2 z = get_z();
  dvec2 c = get_c();
  int i;

  dvec2 tmp;
  // dvec2 zsq = dvec2(z.x*z.x - z.y*z.y, 2*z.x*z.y);
  for (i = 0; i < u_MaxIter; i++) {

    tmp.x = z.x*z.x - z.y*z.y;
    tmp.y = 2.0*z.x*z.y;
    z.x = tmp.x;
    z.y = tmp.y;
    z.x += c.x;
    z.y += c.y;

    // z = z * z + c;
    // z =  dvec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;

    // z.y = (z.x + z.y)*(z.x + z.y) - zsq.x - zsq.y;
    // z.y += c.y;
    // z.x = zsq.x - zsq.y + c.x;
    // zsq.x = z.x * z.x;
    // zsq.y = z.y * z.y;


    // a different fast-check
    // if((pow(D.x, 2) + pow(D.y, 2)) < 0.001) {
    //   Target0 =  vec4(i/u_MaxIter, 0.0, 0.0, 1.0);
    //   return;
    // }


    //  dvec2 tmp;
    // tmp.x = D.x*z.x - D.y*z.y;
    // tmp.y = D.x*z.y + D.y*z.x;
    // D.x = 2.0*tmp.x;
    // D.y = 2.0*tmp.y;

    if((z.x*z.x + z.y*z.y) > 18) {
      break;
    }
    // if (!fast_check(z)) { return;}
  }


  if (i == u_MaxIter) {
    Target0 =  vec4(1.0, 1.0, 1.0, 1.0);
  } else {
    float dist = float(z.x*z.x + z.y*z.y);

    dist  = log(log(dist))/log(2.0);
    float val = float(i) - dist;
    // float val = i/ float(u_MaxIter);
    Target0 =  vec4(hsv2rgb( vec3((val*0.1 - u_Time/5.0), 1.0, 1.0 )), 1.0);
  }
}
/*z.r = 0;
  z.i = 0;
  zrsqr = z.r * z.r;
  zisqr = z.i * z.i;
  while (zrsqr + zisqr <= 4.0)
  {
  z.i = z.r * z.i;
  z.i += z.i; // Multiply by two
  z.i += c.i;
  z.r = zrsqr – zisqr + c.r;
  zrsqr = square(z.r);
  zisqr = square(z.i);
}*/

// dirty check that coveres points close enough to an attractor
// bool fast_check( dvec2 Z) {
//   float r = sqrt(pow(Z.x - 0.25, 2) + pow(Z.y, 2));
//   if (Z.x < r - 2 * pow(r, 2) + 0.25) {
//     Target0 =  vec4(0, 1.0, 1.0, 1.0);
//     return false;
//   }

//   if (pow(Z.x + 1, 2) + pow(Z.y, 2) < 0.0625) {
//     Target0 =  vec4(1.0, 1.0, 1.0, 1.0);
//     return false;
//   }

//   return true;
// }



// hue-rotates an rgb-value
vec3 hsv2rgb( vec3 c) {
  vec4 K =  vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
  vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www );
  return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}
