use wasapi::*;

fn main() {
    initialize_mta().unwrap();

    println!("========== OUTPUT DEVICES ==========");
    println!("Found the following output devices:");
    for device in &DeviceCollection::new(&Direction::Render).unwrap() {
        let dev = device.unwrap();
        let state = &dev.get_state().unwrap();
        println!(
            "Device: {:?}. State: {:?}",
            &dev.get_friendlyname().unwrap(),
            state
        );
    }

    print!("\n");

    println!("Default output devices:");
    [Role::Console, Role::Multimedia, Role::Communications]
        .iter()
        .for_each(|role| {
            println!(
                "{:?}: {:?}",
                role,
                get_default_device_for_role(&Direction::Render, role)
                    .unwrap()
                    .get_friendlyname()
                    .unwrap()
            );
        });

    print!("\n\n");

    println!("========== INPUT DEVICES ==========");
    println!("Found the following input devices:");
    for device in &DeviceCollection::new(&Direction::Capture).unwrap() {
        let dev = device.unwrap();
        let state = &dev.get_state().unwrap();
        println!(
            "Device: {:?}. State: {:?}",
            &dev.get_friendlyname().unwrap(),
            state
        );
    }

    print!("\n");

    println!("Default input devices:");
    [Role::Console, Role::Multimedia, Role::Communications]
        .iter()
        .for_each(|role| {
            println!(
                "{:?}: {:?}",
                role,
                get_default_device_for_role(&Direction::Capture, role)
                    .unwrap()
                    .get_friendlyname()
                    .unwrap()
            );
        });

}
