// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright © 2022 Adrian <adrian.eddy at gmail>

fn undistort_point(pos: vec2<f32>, k: vec4<f32>, amount: f32) -> vec2<f32> {
    let NEWTON_EPS = 0.00001;

    let rd = length(pos);
    if (rd == 0.0) { return vec2<f32>(0.0, 0.0); }

    var ru = rd;
    for (var i: i32 = 0; i < 10; i = i + 1) {
        let fru = ru * (k.x * ru * ru * ru + k.y * ru * ru + k.z * ru + 1.0) - rd;
        if (fru >= -NEWTON_EPS && fru < NEWTON_EPS) {
            break;
        }
        if (i > 5) {
            // Does not converge, no real solution in this area?
            return vec2<f32>(0.0, 0.0);
        }

        ru = ru - (fru / (4.0 * k.x * ru * ru * ru + 3.0 * k.y * ru * ru + 2.0 * k.z * ru + 1.0));
    }
    if (ru < 0.0) {
        return vec2<f32>(0.0, 0.0);
    }

    ru = ru / rd;

    // Apply only requested amount
    ru = 1.0 + (ru - 1.0) * (1.0 - amount);

    return pos * ru;
}

fn distort_point(pos: vec2<f32>, k: vec4<f32>) -> vec2<f32> {
    let ru2 = (pos.x * pos.x + pos.y * pos.y);
    let r = sqrt(ru2);
    let poly3 = k.x * ru2 * r + k.y * ru2 + k.z * r + 1.0;
    return pos * poly3;
}