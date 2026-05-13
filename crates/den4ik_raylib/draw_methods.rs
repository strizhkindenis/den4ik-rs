// 2D
    pub fn draw_pixel(&mut self, pos_x: i32, pos_y: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawPixel(pos_x, pos_y, color.to_rl_color()) } 
    }

    pub fn draw_pixel_v(&mut self, position: crate::math::Vector2, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawPixelV(position.into(), color.to_rl_color()) } 
    }

    pub fn draw_line(&mut self, start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawLine(start_pos_x, start_pos_y, end_pos_x, end_pos_y, color.to_rl_color()) } 
    }

    pub fn draw_line_v(&mut self, start_pos: crate::math::Vector2, end_pos: crate::math::Vector2, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawLineV(start_pos.into(), end_pos.into(), color.to_rl_color()) } 
    }

    pub fn draw_line_ex(&mut self, start_pos: crate::math::Vector2, end_pos: crate::math::Vector2, thick: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawLineEx(start_pos.into(), end_pos.into(), thick, color.to_rl_color()) } 
    }

    pub fn draw_line_bezier(&mut self, start_pos: crate::math::Vector2, end_pos: crate::math::Vector2, thick: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawLineBezier(start_pos.into(), end_pos.into(), thick, color.to_rl_color()) } 
    }

    pub fn draw_line_dashed(&mut self, start_pos: crate::math::Vector2, end_pos: crate::math::Vector2, dash_size: i32, space_size: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawLineDashed(start_pos.into(), end_pos.into(), dash_size, space_size, color.to_rl_color()) } 
    }

    pub fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCircle(center_x, center_y, radius, color.to_rl_color()) } 
    }

    pub fn draw_circle_v(&mut self, center: crate::math::Vector2, radius: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCircleV(center.into(), radius, color.to_rl_color()) } 
    }

    pub fn draw_circle_gradient(&mut self, center: crate::math::Vector2, radius: f32, inner: impl crate::color::ToRlColor, outer: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCircleGradient(center.into(), radius, inner.to_rl_color(), outer.to_rl_color()) } 
    }

    pub fn draw_circle_sector(&mut self, center: crate::math::Vector2, radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCircleSector(center.into(), radius, start_angle, end_angle, segments, color.to_rl_color()) } 
    }

    pub fn draw_circle_sector_lines(&mut self, center: crate::math::Vector2, radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCircleSectorLines(center.into(), radius, start_angle, end_angle, segments, color.to_rl_color()) } 
    }

    pub fn draw_circle_lines(&mut self, center_x: i32, center_y: i32, radius: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCircleLines(center_x, center_y, radius, color.to_rl_color()) } 
    }

    pub fn draw_circle_lines_v(&mut self, center: crate::math::Vector2, radius: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCircleLinesV(center.into(), radius, color.to_rl_color()) } 
    }

    pub fn draw_ellipse(&mut self, center_x: i32, center_y: i32, radius_h: f32, radius_v: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawEllipse(center_x, center_y, radius_h, radius_v, color.to_rl_color()) } 
    }

    pub fn draw_ellipse_v(&mut self, center: crate::math::Vector2, radius_h: f32, radius_v: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawEllipseV(center.into(), radius_h, radius_v, color.to_rl_color()) } 
    }

    pub fn draw_ellipse_lines(&mut self, center_x: i32, center_y: i32, radius_h: f32, radius_v: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawEllipseLines(center_x, center_y, radius_h, radius_v, color.to_rl_color()) } 
    }

    pub fn draw_ellipse_lines_v(&mut self, center: crate::math::Vector2, radius_h: f32, radius_v: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawEllipseLinesV(center.into(), radius_h, radius_v, color.to_rl_color()) } 
    }

    pub fn draw_ring(&mut self, center: crate::math::Vector2, inner_radius: f32, outer_radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRing(center.into(), inner_radius, outer_radius, start_angle, end_angle, segments, color.to_rl_color()) } 
    }

    pub fn draw_ring_lines(&mut self, center: crate::math::Vector2, inner_radius: f32, outer_radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRingLines(center.into(), inner_radius, outer_radius, start_angle, end_angle, segments, color.to_rl_color()) } 
    }

    pub fn draw_rectangle(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRectangle(pos_x, pos_y, width, height, color.to_rl_color()) } 
    }

    pub fn draw_rectangle_v(&mut self, position: crate::math::Vector2, size: crate::math::Vector2, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRectangleV(position.into(), size.into(), color.to_rl_color()) } 
    }

    pub fn draw_rectangle_rec(&mut self, rec: crate::math::Rectangle, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRectangleRec(rec.into(), color.to_rl_color()) } 
    }

    pub fn draw_rectangle_pro(&mut self, rec: crate::math::Rectangle, origin: crate::math::Vector2, rotation: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRectanglePro(rec.into(), origin.into(), rotation, color.to_rl_color()) } 
    }

    pub fn draw_rectangle_gradient_v(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, top: impl crate::color::ToRlColor, bottom: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRectangleGradientV(pos_x, pos_y, width, height, top.to_rl_color(), bottom.to_rl_color()) } 
    }

    pub fn draw_rectangle_gradient_h(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, left: impl crate::color::ToRlColor, right: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRectangleGradientH(pos_x, pos_y, width, height, left.to_rl_color(), right.to_rl_color()) } 
    }

    pub fn draw_rectangle_gradient_ex(&mut self, rec: crate::math::Rectangle, top_left: impl crate::color::ToRlColor, bottom_left: impl crate::color::ToRlColor, bottom_right: impl crate::color::ToRlColor, top_right: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRectangleGradientEx(rec.into(), top_left.to_rl_color(), bottom_left.to_rl_color(), bottom_right.to_rl_color(), top_right.to_rl_color()) } 
    }

    pub fn draw_rectangle_lines(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRectangleLines(pos_x, pos_y, width, height, color.to_rl_color()) } 
    }

    pub fn draw_rectangle_lines_ex(&mut self, rec: crate::math::Rectangle, line_thick: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRectangleLinesEx(rec.into(), line_thick, color.to_rl_color()) } 
    }

    pub fn draw_rectangle_rounded(&mut self, rec: crate::math::Rectangle, roundness: f32, segments: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRectangleRounded(rec.into(), roundness, segments, color.to_rl_color()) } 
    }

    pub fn draw_rectangle_rounded_lines(&mut self, rec: crate::math::Rectangle, roundness: f32, segments: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRectangleRoundedLines(rec.into(), roundness, segments, color.to_rl_color()) } 
    }

    pub fn draw_rectangle_rounded_lines_ex(&mut self, rec: crate::math::Rectangle, roundness: f32, segments: i32, line_thick: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawRectangleRoundedLinesEx(rec.into(), roundness, segments, line_thick, color.to_rl_color()) } 
    }

    pub fn draw_triangle(&mut self, v1: crate::math::Vector2, v2: crate::math::Vector2, v3: crate::math::Vector2, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawTriangle(v1.into(), v2.into(), v3.into(), color.to_rl_color()) } 
    }

    pub fn draw_triangle_gradient(&mut self, v1: crate::math::Vector2, v2: crate::math::Vector2, v3: crate::math::Vector2, c1: impl crate::color::ToRlColor, c2: impl crate::color::ToRlColor, c3: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawTriangleGradient(v1.into(), v2.into(), v3.into(), c1.to_rl_color(), c2.to_rl_color(), c3.to_rl_color()) } 
    }

    pub fn draw_triangle_lines(&mut self, v1: crate::math::Vector2, v2: crate::math::Vector2, v3: crate::math::Vector2, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawTriangleLines(v1.into(), v2.into(), v3.into(), color.to_rl_color()) } 
    }

    pub fn draw_poly(&mut self, center: crate::math::Vector2, sides: i32, radius: f32, rotation: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawPoly(center.into(), sides, radius, rotation, color.to_rl_color()) } 
    }

    pub fn draw_poly_lines(&mut self, center: crate::math::Vector2, sides: i32, radius: f32, rotation: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawPolyLines(center.into(), sides, radius, rotation, color.to_rl_color()) } 
    }

    pub fn draw_poly_lines_ex(&mut self, center: crate::math::Vector2, sides: i32, radius: f32, rotation: f32, line_thick: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawPolyLinesEx(center.into(), sides, radius, rotation, line_thick, color.to_rl_color()) } 
    }

    pub fn draw_spline_segment_linear(&mut self, p1: crate::math::Vector2, p2: crate::math::Vector2, thick: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawSplineSegmentLinear(p1.into(), p2.into(), thick, color.to_rl_color()) } 
    }

    pub fn draw_spline_segment_basis(&mut self, p1: crate::math::Vector2, p2: crate::math::Vector2, p3: crate::math::Vector2, p4: crate::math::Vector2, thick: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawSplineSegmentBasis(p1.into(), p2.into(), p3.into(), p4.into(), thick, color.to_rl_color()) } 
    }

    pub fn draw_spline_segment_catmull_rom(&mut self, p1: crate::math::Vector2, p2: crate::math::Vector2, p3: crate::math::Vector2, p4: crate::math::Vector2, thick: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawSplineSegmentCatmullRom(p1.into(), p2.into(), p3.into(), p4.into(), thick, color.to_rl_color()) } 
    }

    pub fn draw_spline_segment_bezier_quadratic(&mut self, p1: crate::math::Vector2, c2: crate::math::Vector2, p3: crate::math::Vector2, thick: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawSplineSegmentBezierQuadratic(p1.into(), c2.into(), p3.into(), thick, color.to_rl_color()) } 
    }

    pub fn draw_spline_segment_bezier_cubic(&mut self, p1: crate::math::Vector2, c2: crate::math::Vector2, c3: crate::math::Vector2, p4: crate::math::Vector2, thick: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawSplineSegmentBezierCubic(p1.into(), c2.into(), c3.into(), p4.into(), thick, color.to_rl_color()) } 
    }

    pub fn draw_fps(&mut self, pos_x: i32, pos_y: i32) {
        unsafe { crate::ffi::DrawFPS(pos_x, pos_y) } 
    }

    pub fn draw_text(&mut self, text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawText(std::ffi::CString::new(text).unwrap().as_ptr(), pos_x, pos_y, font_size, color.to_rl_color()) } 
    }

// 3D
    pub fn draw_line3_d(&mut self, start_pos: crate::math::Vector3, end_pos: crate::math::Vector3, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawLine3D(start_pos.into(), end_pos.into(), color.to_rl_color()) } 
    }

    pub fn draw_point3_d(&mut self, position: crate::math::Vector3, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawPoint3D(position.into(), color.to_rl_color()) } 
    }

    pub fn draw_circle3_d(&mut self, center: crate::math::Vector3, radius: f32, rotation_axis: crate::math::Vector3, rotation_angle: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCircle3D(center.into(), radius, rotation_axis.into(), rotation_angle, color.to_rl_color()) } 
    }

    pub fn draw_triangle3_d(&mut self, v1: crate::math::Vector3, v2: crate::math::Vector3, v3: crate::math::Vector3, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawTriangle3D(v1.into(), v2.into(), v3.into(), color.to_rl_color()) } 
    }

    pub fn draw_cube(&mut self, position: crate::math::Vector3, width: f32, height: f32, length: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCube(position.into(), width, height, length, color.to_rl_color()) } 
    }

    pub fn draw_cube_v(&mut self, position: crate::math::Vector3, size: crate::math::Vector3, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCubeV(position.into(), size.into(), color.to_rl_color()) } 
    }

    pub fn draw_cube_wires(&mut self, position: crate::math::Vector3, width: f32, height: f32, length: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCubeWires(position.into(), width, height, length, color.to_rl_color()) } 
    }

    pub fn draw_cube_wires_v(&mut self, position: crate::math::Vector3, size: crate::math::Vector3, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCubeWiresV(position.into(), size.into(), color.to_rl_color()) } 
    }

    pub fn draw_sphere(&mut self, center_pos: crate::math::Vector3, radius: f32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawSphere(center_pos.into(), radius, color.to_rl_color()) } 
    }

    pub fn draw_sphere_ex(&mut self, center_pos: crate::math::Vector3, radius: f32, rings: i32, slices: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawSphereEx(center_pos.into(), radius, rings, slices, color.to_rl_color()) } 
    }

    pub fn draw_sphere_wires(&mut self, center_pos: crate::math::Vector3, radius: f32, rings: i32, slices: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawSphereWires(center_pos.into(), radius, rings, slices, color.to_rl_color()) } 
    }

    pub fn draw_cylinder(&mut self, position: crate::math::Vector3, radius_top: f32, radius_bottom: f32, height: f32, slices: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCylinder(position.into(), radius_top, radius_bottom, height, slices, color.to_rl_color()) } 
    }

    pub fn draw_cylinder_ex(&mut self, start_pos: crate::math::Vector3, end_pos: crate::math::Vector3, start_radius: f32, end_radius: f32, sides: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCylinderEx(start_pos.into(), end_pos.into(), start_radius, end_radius, sides, color.to_rl_color()) } 
    }

    pub fn draw_cylinder_wires(&mut self, position: crate::math::Vector3, radius_top: f32, radius_bottom: f32, height: f32, slices: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCylinderWires(position.into(), radius_top, radius_bottom, height, slices, color.to_rl_color()) } 
    }

    pub fn draw_cylinder_wires_ex(&mut self, start_pos: crate::math::Vector3, end_pos: crate::math::Vector3, start_radius: f32, end_radius: f32, slices: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCylinderWiresEx(start_pos.into(), end_pos.into(), start_radius, end_radius, slices, color.to_rl_color()) } 
    }

    pub fn draw_capsule(&mut self, start_pos: crate::math::Vector3, end_pos: crate::math::Vector3, radius: f32, rings: i32, slices: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCapsule(start_pos.into(), end_pos.into(), radius, rings, slices, color.to_rl_color()) } 
    }

    pub fn draw_capsule_wires(&mut self, start_pos: crate::math::Vector3, end_pos: crate::math::Vector3, radius: f32, rings: i32, slices: i32, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawCapsuleWires(start_pos.into(), end_pos.into(), radius, rings, slices, color.to_rl_color()) } 
    }

    pub fn draw_plane(&mut self, center_pos: crate::math::Vector3, size: crate::math::Vector2, color: impl crate::color::ToRlColor) {
        unsafe { crate::ffi::DrawPlane(center_pos.into(), size.into(), color.to_rl_color()) } 
    }

    pub fn draw_grid(&mut self, slices: i32, spacing: f32) {
        unsafe { crate::ffi::DrawGrid(slices, spacing) } 
    }

