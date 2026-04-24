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

// macro_rules! concat_const_arrays {
//     ($type:ty, $($arr:expr),*) => {{
//         const LEN: usize = $($arr.len() + )* 0;
//         const RES: [$type; LEN] = {
//             let mut res = [0; LEN];
//             let mut offset = 0;
//             $(
//                 let arr = $arr;
//                 let mut i = 0;
//                 while i < arr.len() {
//                     res[offset + i] = arr[i];
//                     i += 1;
//                 }
//                 offset += arr.len();
//             )*
//             res
//         };
//         RES
//     }};
// }

    const fn argy_entry(identity: &'static  str , name:&'static str, description:&'static str) -> (&'static str, ArgEntry) {
        (identity, ArgEntry { name, description})
    }

    static MY_ENTRY: (&'static str, ArgEntry) = argy_entry("id", "name", "desc");   
    println!("{}",MY_ENTRY.1.description);

    static arrayb: [(&'static str, ArgEntry); 2] = [
        argy_entry("id", "name", "desc"),
        argy_entry("id2", "name2", "desc2")
    ];

 
    macro_rules! make_big_const_array {
        ( varible_type: $var_type:ident,  element_type: $element_type:ty, entries:  $($entry:expr)*,)=> {
            $var_type array: [$element_type;  ${count($entry)}] = [$($entry,)*]
        }
    }

    // macro_rules! argv_to_n_s {
        
    //        ( lifetime: $life:tt , $( (
    //             name: $name:literal,
    //             ident: $ident1:literal $(,$ident2plus:literal)*,
    //             description: $description:expr
    //         ) ),* )
    //      => {{   
    //             const LEN: usize = 0 $(+ { $name; 1 })*;

    //             let mut tmp_table = [(ArgEntry<$life>); LEN];
    //             let mut tmp_index = 0;


    //             $( 

                
    //             static ent: ArgEntry<$life> = {
    //                 ArgEntry {
    //                     name: &$name,
    //                     description: &$description,
    //                     // ... other fields
    //                 }
    //             };
                
    //             tmp_table[tmp_index] = ($name, ent);
    //             tmp_index = tmp_index+1;


    //             )*
    //      }}
    //     }

        //let cx = argv_to_n_s!(lifetime: 'a , (name:"a", ident:"b", description:"c" ));


    // const  x:[u8;2] = [1,2];
    // const  y:[u8;2]= [3,4];
    // let xy = concat_const_arrays!(u8,x,y);

    #[derive(Clone)]
    struct ArgEntry {
        name: &'static str,
        description: &'static str
    }


    // static PHF_TABLE: phf::Map<&'static str, [&'static str; 2]> = gen_phf_arg_table! {
    //     (name: "help", ident: "helpy", description: "Print help"),
    //      (name: "helpa", ident: "helpya", description: "Print help")
    // };

    // static PHF_TABLE2: phf::Map<&'static str, &'static ArgEntry> = gen_phf_arg_table! {
    //     (ident: "helpy", data: &TEST)
    // };

    // let value = &PHF_TABLE["helpy"];
    // value.iter().for_each(|v|println!("{}",v));



    
    
}
