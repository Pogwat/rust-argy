fn main() {
    println!("Hello, world!");
    macro_rules! gen_phf_arg_table {
        
           ( $( (
                name: $name:literal,
                ident: $ident1:literal $(,$ident2plus:literal)*,
                description: $description:expr
            ) ),* )
         => {
            phf::phf_map!{
                $(
                    $ident1 $( | $ident2plus)* => [$name, $description]


                ),*
            }
        };

        ( $( (
                ident: $ident1:literal $(,$ident2plus:literal)*,
                data: $data:expr
            ) ),* )
         => {
            phf::phf_map!{
                $(
                    $ident1 $( | $ident2plus)* => $data


                ),*
            }
        }

    }
    #[derive(Clone)]
    struct ArgEntry<'a> {
        name: &'a str,
        description: &'a str
    }

    static TEST: ArgEntry<'static> = ArgEntry{
        name: "by",
        description: "no"
    };

    static PHF_TABLE: phf::Map<&'static str, [&'static str; 2]> = gen_phf_arg_table! {
        (name: "help", ident: "helpy", description: "Print help"),
         (name: "helpa", ident: "helpya", description: "Print help")
    };

    static PHF_TABLE2: phf::Map<&'static str, &'static ArgEntry> = gen_phf_arg_table! {
        (ident: "helpy", data: &TEST)
    };

    let value = &PHF_TABLE["helpy"];
    value.iter().for_each(|v|println!("{}",v))

    
    
}
