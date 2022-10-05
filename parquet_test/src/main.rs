use parquet::file::reader::{FileReader, SerializedFileReader};
use std::{fs::File, path::Path};

fn f_print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn f_exmaple_read_single_file(){

    let s_file_name = String::from("ztf_000468_zi_c16_q4_dr7.parquet");
    let o_path = Path::new(&s_file_name);
    
    if let Ok(file) = File::open(&o_path) {
        let reader = SerializedFileReader::new(file).unwrap();
    
        let parquet_metadata = reader.metadata();
        // assert_eq!(parquet_metadata.num_row_groups(), 1);
        // println!("parquet_metadata {:?}", parquet_metadata);
        let row_group_reader = reader.get_row_group(0).unwrap();
        // assert_eq!(row_group_reader.num_columns(), 1);
    
        println!("row_group_reader.num_columns() {:?}", row_group_reader.num_columns());
    
    }
}
fn f_example_read_multiple_files(){

    let paths = vec![
        "ztf_000468_zi_c16_q4_dr7.parquet",
    ];
    // Create a reader for each file and flat map rows
    let rows = paths.iter()
        .map(|p| SerializedFileReader::try_from(*p).unwrap())
        .flat_map(|r| r.into_iter());

    for row in rows {
        f_print_type_of(&row);

        for (n_index, (s_prop_name, value)) in row.get_column_iter().enumerate() {
            println!("n_index: {:?}", n_index);
            println!("s_prop_name: {:?}", s_prop_name);
            println!("value: {:?}", value);
        }
        // println!("{}", row);
        // println!("{}", row["objectid"]);
    }
}
fn f_example_read_and_write(){
    
    let paths = vec![
        "ztf_000468_zi_c16_q4_dr7.parquet",
    ];
    // Create a reader for each file and flat map rows
    let rows = paths.iter()
        .map(|p| SerializedFileReader::try_from(*p).unwrap())
        .flat_map(|r| r.into_iter());

    for row in rows {
        f_print_type_of(&row);

        for (n_index, (s_prop_name, value)) in row.get_column_iter().enumerate() {
            println!("n_index: {:?}", n_index);
            println!("s_prop_name: {:?}", s_prop_name);
            println!("value: {:?}", value);
        }
        // println!("{}", row);
        // println!("{}", row["objectid"]);
    }
}
fn main() {

    // f_exmaple_read_single_file();
    f_example_read_multiple_files();

}
