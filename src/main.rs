fn main() {
    println!("Hello, world!");
    
    struct ArgEntry {
        identifier: &'static str,
        name: &'static str,
        description: &'static str
    }

    macro_rules! make_argy {
        (    $(  ( $($identifier:expr),+ => $name:expr, $description:expr  )    ),*    ) => {
                phf::phf_map! { $(   $($identifier) | +  =>    ($name,  $description)     ),* }
        }
    }

    static ARGS: phf::Map<&'static str, (&'static str, &'static str)> = make_argy!(
        ("a", "d" => "b", "c")
    
    
    
    ); 
    
}
