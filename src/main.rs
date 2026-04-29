fn main() {
    println!("Hello, world!");
    
    struct ArgEntry {
        identifier: &'static str,
        name: &'static str,
        description: &'static str
    }

    struct Argy<const N:usize> {
        args: phf::Map<&'static str, (&'static str, &'static str)>,
        fargs: [&'static str; N]
    }

    macro_rules! make_argy_map {
        (    $(  ( $($identifier:expr),+ => $name:expr, $description:expr  )    ),*    ) => {
                phf::phf_map! { $(   $($identifier) | +  =>    ($name,  $description)     ),* }
        }
    }

    static ARGS: phf::Map<&'static str, (&'static str, &'static str)> = make_argy_map!(
        ("a", "d" => "b", "c"), ("g" => "b", "c")
    ); 

    type ArgyMap = phf::Map<&'static str, (&'static str, &'static str)>;
    impl <const N: usize>Argy<N> {

        parse_args_from_array<T, N:usize>(&mut self,args_array: [T,N]) {
            args_array.iter().for_each(|entry| {
                if map.contains_key(entry) {}
            }



            )
        }


    }

    macro_rules! make_argies {
        (  $($argy_map:expr),+  ) => {
            (
                $(
                    Argy {
                        args:$argy_map,
                        fargs: [""; $argy_map.len()]
                    }
                ),+
            )
        }
    }   

    let  (x,a)  =make_argies!( 
        make_argy_map!(("a", "d" => "b", "c"), ("g" => "b", "c") ),
        make_argy_map!(("a", "d" => "b", "c"), ("g" => "b", "c") ) 
    ); 
    
}