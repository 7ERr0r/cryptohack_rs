pub mod a_general;
pub mod b_mathematics;

macro_rules! run_function {
    // Makro wypisuje nazwe funkcji i ja uruchamia
    ($func_name:expr) => {
        println!("\n{}", stringify!($func_name));
        $func_name();
    };
}

fn main() {
    // GENERAL
    if true {
        // ENCODING
        run_function!(a_general::a_ascii);
        run_function!(a_general::b_hex);
        run_function!(a_general::c_base64);
        run_function!(a_general::d_bytes_big_integers);
        run_function!(a_general::e_tcp);
        run_function!(a_general::f_xor_starter);
        run_function!(a_general::g_xor_properties);
        run_function!(a_general::h_favourite_byte);
        run_function!(a_general::i_either);
        

        
    }

    // MATHEMATICS
    if false {
        // MODULAR MATH
        run_function!(b_mathematics::a_modular_math_quadratic_residues);
        run_function!(b_mathematics::b_modular_math_legendre_symbol);
        //run_function!(b_mathematics::c_modular_math_modular_square_root);
        //run_function!(b_mathematics::d_modular_math_chinese_remainder_theorem);

        // LATTICES
        run_function!(b_mathematics::e_lattices_vectors);
        run_function!(b_mathematics::f_lattices_size_basis);
        run_function!(b_mathematics::g_lattices_gram_schmidt);
    }
}