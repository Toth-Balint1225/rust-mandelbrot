/*
  Copyright (C) 2023  Tóth Bálint

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
#version 330 core

in vec2 tex_coord;
out vec4 color;

uniform mat3 mvp;
uniform int max_iter;

int calc_pixel(vec3 coord) {
    float x_0 = coord.x;
    float y_0 = coord.y;
    int iter = 0;
    float x = 0.0, y = 0.0;
    float x_2 = 0.0;
    float y_2 = 0.0;
    while (x_2 + y_2 <= 4 && iter < max_iter) {
        y = 2.0 * x * y + y_0;
        x = x_2 - y_2 + x_0;
        x_2 = x * x;
        y_2 = y *y;
        iter++;
    }
    return iter;
}

void main() {
    vec3 corrected = mvp * vec3(tex_coord, 1.0);
    int iters = calc_pixel(corrected);
    // float scale = float(iters) / float(max_iter);
    // color = vec4(scale, 1.0 - scale, 0.5, 1.0);
    // ---
    float n = float(iters);
    color = vec4(0.5 * sin(n) + 0.5, 0.5 * sin(n + 2.094) + 0.5, 0.5 * sin(n + 4.188) + 0.5, 1.0);
}
