.......A simple math programming math example 
use std::f64::consts::PI;

fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

fn main() {
    // Angles in degree
    let test_angles_deg = [30.0, 60.0, 72.0, 36.0, 57.3];
    
    // Convert degrees to radians manually
    let test_angles_rad: Vec<f64> = test_angles_deg.iter().map(|&d| deg_to_rad(d)).collect();
    
    println!("Trigonometric Identity Verification:");
    println!("LHS = tanA/(1 - cotA) + cotA/(1 - tanA)");
    println!("RHS = secA·cosecA + 1\n");
    
    for &a in &test_angles_rad {
        // Check for undefined points (avoid tanA=1 or undefined trig values)
        if a.tan().is_nan() || (a.tan() - 1.0).abs() < 1e-10 {
            println!("⚠️ Skipping A = {:.4} (undefined or tanA=1)", a);
            continue;
        }
        
        // Calculate all trig functions
        let (sin_a, cos_a) = a.sin_cos();
        let tan_a = sin_a / cos_a;
        let cot_a = cos_a / sin_a;
        let sec_a = 1.0 / cos_a;
        let cosec_a = 1.0 / sin_a;
        
        // Calculate LHS carefully with handling for denominator zeros
        let lhs_term1 = if (1.0 - cot_a).abs() > 1e-10 {
            tan_a / (1.0 - cot_a)
        } else {
            f64::NAN
        };
        
        let lhs_term2 = if (1.0 - tan_a).abs() > 1e-10 {
            cot_a / (1.0 - tan_a)
        } else {
            f64::NAN
        };
        
        // Skip calculation if we have division by zero
        if lhs_term1.is_nan() || lhs_term2.is_nan() {
            println!("⚠️ Skipping A = {:.4} (division by zero)", a);
            continue;
        }
        
        let lhs = lhs_term1 + lhs_term2;
        let rhs = sec_a * cosec_a + 1.0;
        
        // Compare with tolerance for floating-point precision
        let diff = (lhs - rhs).abs();
        
        println!("For A = {:.4} rad ({:.1}°):", a, a.to_degrees());
        println!("LHS = {:.12}", lhs);
        println!("RHS = {:.12}", rhs);
        println!("Difference = {:.12}", diff);
        println!("Verification: {}", if diff < 1e-10 { "✅ SUCCESS" } else { "❌ FAILED" });
        println!("----------------------------------------");
    }
}
