use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

struct App
{
    window: Option<Window>,
    title: &'static str,
    sender: Option<Sender<&'static str>>,
}

impl App
{
    fn new(title: &'static str, sender: Sender<&'static str>) -> Self
    {
        Self
        {
            window: None,
            title,
            sender: Some(sender),
        }
    }
}

impl ApplicationHandler for App
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        let mut attributes = Window::default_attributes();
        attributes.title = String::from(self.title);
        self.window = Some(event_loop.create_window(attributes).unwrap());

        // Inicia a thread aqui, após a janela abrir
        let tx = self.sender.take().unwrap();
        thread::spawn(move || {
            println!("thread started!");
            thread::sleep(Duration::from_secs(10));
            tx.send("hello from resources thread").unwrap();
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent)
    {
        match event
        {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => self.window.as_ref().unwrap().request_redraw(),
            _ => (),
        }
    }
}

fn main()
{
    let (tx, rx): (Sender<&'static str>, Receiver<&'static str>) = mpsc::channel();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new("Minha Janela", tx);

    thread::spawn(move ||
    {
        if let Ok(msg) = rx.recv() {
            println!("Recebido na main thread: {}", msg);
        }
    });

    let _ = event_loop.run_app(&mut app);
}