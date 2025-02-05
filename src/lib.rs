#![deny(clippy::all)]

use napi_derive::napi;
use napi::bindgen_prelude::*;
use wasapi::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::{self, SyncSender, Receiver};
use std::time::{Duration, Instant};
use std::thread;


extern crate napi_derive;

// #[napi]
// pub enum Direction {
//   Render,
//   Capture,
// }

#[napi(object)]
pub  struct Device {
  pub name: String,
  pub description: String,
  pub state: String,
  pub id: String,
}

#[napi]
struct WaveFormatStruct {
  storebits: u32,
  validbits: u32,
  sample_type: u32,
  samplerate: u32,
  channels: u32,

  direction: Direction,
  sender: SyncSender<Vec<u8>>,
  receiver: Receiver<Vec<u8>>,
}


#[napi]
impl WaveFormatStruct {
  #[napi(constructor)]
  pub fn new(storebits: u32, validbits: u32, sample_type: u32, samplerate: u32, channels: u32) -> Self {
    let (sender, receiver) = mpsc::sync_channel(2);
    WaveFormatStruct { storebits, validbits, sample_type, samplerate, channels, direction: Direction::Render, sender, receiver  }
  }

  #[napi]
  pub fn init(&mut self) {
    println!("init");
    initialize_mta().ok();
  }

  #[napi]
  pub fn start<T: Fn(Buffer) -> Result<()>>(&mut self, callback: T) {
    let sender_clone = self.sender.clone();
    let _handle = thread::spawn(move || {
      let _result = WaveFormatStruct::capture_loop(sender_clone);

    });
    loop {
      match self.receiver.recv() {
          Ok(chunk) => {
              callback(chunk.clone().into());
          }
          Err(err) => {
              println!("Some error {}", err);
              return ();
          }
      }
    }
  }

  fn capture_loop(tx_capt: std::sync::mpsc::SyncSender<Vec<u8>>) -> () {
    let chunksize = 128;
    let device = get_default_device(&Direction::Render).unwrap();

    let mut audio_client = device.get_iaudioclient().unwrap();

    let desired_format = WaveFormat::new(16, 16, &SampleType::Int, 48000, 2, None);

    let blockalign = desired_format.get_blockalign();

    let (def_time, min_time) = audio_client.get_periods().unwrap();
    // let mut outfile = File::create("recorded.raw").unwrap();

    audio_client.initialize_client(
        &desired_format,
        min_time,
        &Direction::Capture,
        &ShareMode::Shared,
        true,
    ).unwrap();

    let h_event = audio_client.set_get_eventhandle().unwrap();

    let buffer_frame_count = audio_client.get_bufferframecount().unwrap();

    let render_client = audio_client.get_audiocaptureclient().unwrap();
    let mut sample_queue: VecDeque<u8> = VecDeque::with_capacity(
        100 * blockalign as usize * (1024 + 2 * buffer_frame_count as usize),
    );
    let session_control = audio_client.get_audiosessioncontrol().unwrap();

    audio_client.start_stream().unwrap();
   
    let start_time = Instant::now();
    println!("start {:#?}", start_time);
    loop {
        while sample_queue.len() > (blockalign as usize * chunksize) {
            let mut chunk = vec![0u8; blockalign as usize * chunksize];
            for element in chunk.iter_mut() {
                *element = sample_queue.pop_front().unwrap();
            }
            print!("chunk {:#?}, time {:#?}", chunk.len(), start_time.elapsed());
            tx_capt.send(chunk);
            // outfile.write_all(&chunk).unwrap();
        }
        render_client.read_from_device_to_deque(&mut sample_queue).unwrap();
        // if h_event.wait_for_event(3000).is_err() {
        //     audio_client.stop_stream().unwrap();
        //     break;
        // }
        // if self.status == 0 {
        //   audio_client.stop_stream().unwrap();
        //   break;
        // }
        if start_time.elapsed() >= Duration::from_secs(10) {
            audio_client.stop_stream().unwrap();
            break;
        }
    }
    return ()
  }

  // #[napi]
  // pub fn get_status(&self) -> u32 {
  //   self.status
  // }

  // #[napi]
  // pub fn set_status(&mut self, val: u32) {
  //     self.status = val;
  // }

  #[napi]
  pub fn get_static() -> u32 {
      2
  }

  #[napi]
  pub fn get_device(&self) -> Device {
    let dev = get_default_device(&Direction::Render);
    let name =  dev.as_ref().expect("name").get_friendlyname().unwrap();
    let state =  dev.as_ref().expect("state").get_state().unwrap();
    let description =  dev.as_ref().expect("description").get_description().unwrap();
    let id =  dev.as_ref().expect("id").get_id().unwrap();
    Device{ 
      name: name.to_string(), 
      description: description.to_string(), 
      state: state.to_string(), 
      id: id.to_string()
    }
  }
}