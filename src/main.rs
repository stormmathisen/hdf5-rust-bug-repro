use hdf5;

const PER_GROUP: u64 = 10;
const GROUPS: u64 = 400000;


fn main() {
    let fake_data: [u16; 512] = [255; 512];
    //Create file
    let hdffile = hdf5::File::create("AFile.h5").unwrap();
    
        
    for i in 0..GROUPS
    {
        let groupname = format!("{}", i);
        //Create group
        let hdfgroup = hdffile.create_group(&groupname).unwrap();
        

        for i in 0..PER_GROUP {
            //Write PER_GROUP 8kb arrays as individual datasets
            let ds_name = format!("{}", i);

            hdfgroup
                .new_dataset_builder()
                .with_data(&fake_data)
                .create(&*ds_name)
                .unwrap();

            /*heaptrack will show that the call to create will cause the majority
            of memory leaks, when it creates the plist for access properties*/
        }
    }
    //Close the HDF file
    hdffile.close().unwrap();
    println!("Finished!");
    loop {
        //Hang about until quit.
        //Memory useage will remain high, would expect dropping the file to stop that
        std::hint::spin_loop();
    }

}
