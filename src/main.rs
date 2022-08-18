use hdf5;

const PER_GROUP: u64 = 10;
const GROUPS: u64 = 400000;


fn main() {
    let fake_data: [u16; 512] = [255; 512];

    let hdffile = hdf5::File::create("AFile.h5").unwrap();
        
    for i in 0..GROUPS
    {
        let groupname = format!("{}", i);
        
        let hdfgroup = hdffile.create_group(&groupname).unwrap();

        for i in 0..PER_GROUP {
            let ds_name = format!("{}", i);

            hdfgroup
                .new_dataset_builder()
                .with_data(&fake_data)
                .create(&*ds_name)
                .unwrap();
        }
    }
    hdffile.close().unwrap();
    println!("Finished!");
    loop {
        std::hint::spin_loop();
    }

}
