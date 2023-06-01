use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use std::env;
use std::process::Command;
use winit::event_loop::{ControlFlow, EventLoopBuilder};

fn main() {
    println!("F2已被夺取，防止某些屎山应用占用");
    let event_loop = EventLoopBuilder::new().build();

    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();

    let f2 = HotKey::new(None, Code::F2);

    hotkeys_manager.register(f2).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    println!("等待某些屎山启动，本程序自动退出");
    for argument in env::args() {
        println!("{}", argument);
        let status = Command::new(&argument)
            .status()
            .expect("command failed to start");

        if status.success() {
            println!("{} executed successfully", argument);
        } else {
            println!(
                "{} failed with status code {}",
                argument,
                status.code().unwrap_or(-1)
            );
        }
    }

    println!("某些屎山已启动，本程序自动退出");

    //开启新线程2秒后关闭本程序
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(2));
        std::process::exit(0);
    });

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Ok(event) = global_hotkey_channel.try_recv() {
            println!("{event:?}");
        }
    })
}
