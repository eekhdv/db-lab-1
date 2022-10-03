pub mod qq {
    use std::fs::File;
    use std::io::Write;
    pub fn init_temp_file(static_names: String, testing_table: String) {
        let mut temp_file = File::options()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(std::path::Path::new("../generated_tables/.temp.txt"))
            .unwrap();
        let buf_names = static_names.split("\n");
        let buf_tests = testing_table.split("\n");

        let mut data1 = buf_names.clone().into_iter();
        let mut data2 = buf_tests.clone().into_iter();

        let length = buf_tests.count();
        for _ in 0..length - 1 {
            let mut _data2 = data2.next().unwrap().split(",").into_iter();
            let data2write = format!(
                "{},{},{}\n",
                _data2.next().unwrap(),
                data1.next().unwrap().replace("\r", ""),
                _data2.next().unwrap(),
            );
            temp_file.write(data2write.as_bytes()).unwrap();
        }
    }
}
