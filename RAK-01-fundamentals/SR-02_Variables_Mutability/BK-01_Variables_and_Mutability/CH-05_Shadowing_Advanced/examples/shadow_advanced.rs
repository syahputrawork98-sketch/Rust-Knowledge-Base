fn main() {
    let x = 5;
    
    // Shadowing 1
    let x = x + 1; 
    println!("x di main scope: {}", x); // 6

    {
        // Shadowing dalam scope (INNER SCOPE)
        let x = x * 2; 
        println!("x di inner scope: {}", x); // 12
        
        // x di sini adalah variabel yang berbeda sama sekali di level memori,
        // meskipun namanya sama.
    } // x (12) lenyap di sini

    println!("x kembali ke nilai main scope: {}", x); // 6
}
