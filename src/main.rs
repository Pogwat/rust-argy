fn main() {
    println!("Hello, world!");
    
    struct ArgEntry {
        name: &'static str,
        description: &'static str,
        arg_type: &'static str
    }

    struct Argy<const N:usize
    //, const F:usize
    > {
        args: phf::Map<&'static str, ArgEntry>,
        fargs: [&'static str; N]
        //func_asserts: [(&'static str, fn (&str) -> bool); F]
    }

    //const defualt_asserts: [(&'static str, fn (&str)); 3]

    macro_rules! make_argy_map {
        (    $(  ( $($identifier:expr),+ => $name:expr, $description:expr, $arg_type:expr  )    ),*    ) => {
                phf::phf_map! { $(   $($identifier) | +  =>    ArgEntry { name:$name,  description:$description, arg_type:$arg_type }     ),* }
        }
    }

    static ARGS: phf::Map<&'static str, ArgEntry> = make_argy_map!(
        ("a", "d" => "b", "c", "e"), ("g" => "b", "c", "e")
    ); 

    type ArgyMap = phf::Map<&'static str, ArgEntry>;
    impl <const N: usize>Argy<N> {

        // fn parse_args_from_array<T, N:usize>(&mut self,args_array: [T,N]) {
        //     args_array.iter().for_each(|entry| {
        //         if self.args.contains_key(entry) {}
        //     }



        //     )
        // }

        // fn assert_flag(&self, string: &str) ->bool {
        //     if let Some(key) = self.args.key(string) {
        //         assert_eq!(key.arg_type, "flag")
        //     }
        // }

        // fn assert_


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
        make_argy_map!(("a", "d" => "b", "c", "e"), ("g" => "b", "c", "e") ),
        make_argy_map!(("a", "d" => "b", "c", "e"), ("g" => "b", "c", "e") ) 
    ); 
    
}