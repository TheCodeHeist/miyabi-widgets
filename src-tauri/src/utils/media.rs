use std::fmt;
use std::ops::Deref;

use base64::{engine::general_purpose, Engine as _};
use image::{DynamicImage, ImageBuffer, RgbaImage};
use std::io::Cursor;
use tauri::{async_runtime, App, Emitter, Listener};
use windows::Foundation::TypedEventHandler;
use windows::Media::Control::{
  GlobalSystemMediaTransportControlsSessionPlaybackStatus as WinPlaybackStatus,
  MediaPropertiesChangedEventArgs, PlaybackInfoChangedEventArgs,
  TimelinePropertiesChangedEventArgs,
};
use windows::{
  Graphics::Imaging::BitmapDecoder,
  Media::{
    Control::{
      GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
      GlobalSystemMediaTransportControlsSessionMediaProperties,
      GlobalSystemMediaTransportControlsSessionTimelineProperties,
    },
    Playback::MediaPlayer,
  },
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MediaStatus {
  Closed,
  Opened,
  Changing,
  Stopped,
  Playing,
  Paused,
}

impl fmt::Display for MediaStatus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl From<WinPlaybackStatus> for MediaStatus {
  fn from(a: WinPlaybackStatus) -> Self {
    match a {
      WinPlaybackStatus::Closed => MediaStatus::Closed,
      WinPlaybackStatus::Opened => MediaStatus::Opened,
      WinPlaybackStatus::Changing => MediaStatus::Changing,
      WinPlaybackStatus::Stopped => MediaStatus::Stopped,
      WinPlaybackStatus::Playing => MediaStatus::Playing,
      WinPlaybackStatus::Paused => MediaStatus::Paused,
      // there do not exist any more cases
      _ => panic!("Unknown Playback Status! {:?}", a),
    }
  }
}

#[derive(Debug, Clone)]
pub struct MediaSession {
  // mp_inst: GlobalSystemMediaTransportControlsSessionManager,
  session: GlobalSystemMediaTransportControlsSession,
  properties: GlobalSystemMediaTransportControlsSessionMediaProperties,
  timeline: GlobalSystemMediaTransportControlsSessionTimelineProperties,
  // info: MediaSessionInfo,
}

impl MediaSession {
  pub async fn new() -> Result<Self, windows::core::Error> {
    let mp = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.await?;
    let session = mp.GetCurrentSession()?;
    let properties = session.TryGetMediaPropertiesAsync()?.await?;
    let timeline = session.GetTimelineProperties()?;
    Ok(Self {
      // mp_inst: mp,
      session,
      properties,
      timeline,
      // info: MediaSessionInfo {
      //   status_code: 200,
      //   title: "".to_string(),
      //   artist: "".to_string(),
      //   position: 0,
      //   duration: 0,
      //   media_status: MediaStatus::Closed,
      //   thumbnail: "".to_string(),
      // },
    })
  }

  // pub fn refresh_session_details(&mut self) -> Result<(), windows::core::Error> {
  //   self.session = self.mp_inst.GetCurrentSession()?;
  //   self.properties = self.session.TryGetMediaPropertiesAsync()?.get()?;
  //   self.timeline = self.session.GetTimelineProperties()?;
  //   Ok(())
  // }

  // pub fn init_event_handler(&self) -> Result<(), windows::core::Error> {
  //   let session = self.session.clone();

  //   let playback_info_handler =
  //     TypedEventHandler::new(move |_, args: &Option<PlaybackInfoChangedEventArgs>| {
  //       let details = args.as_ref().unwrap();
  //       println!("PlaybackInfoChanged event fired! {:?}", details);
  //       Ok(())
  //     });

  //   let timeline_properties_handler = TypedEventHandler::new(
  //     move |_, args: &Option<TimelinePropertiesChangedEventArgs>| {
  //       let details = args.as_ref().unwrap();
  //       println!("TimelinePropertiesChanged event fired! {:?}", details);
  //       Ok(())
  //     },
  //   );

  //   let media_properties_handler =
  //     TypedEventHandler::new(move |_, args: &Option<MediaPropertiesChangedEventArgs>| {
  //       let details = args.as_ref().unwrap();
  //       println!("MediaPropertiesChanged event fired! {:?}", details);
  //       Ok(())
  //     });

  //   session.PlaybackInfoChanged(&playback_info_handler)?;
  //   session.TimelinePropertiesChanged(&timeline_properties_handler)?;
  //   session.MediaPropertiesChanged(&media_properties_handler)?;

  //   Ok(())
  // }

  pub fn get_artist(&self) -> String {
    self.properties.Artist().unwrap_or_default().to_string()
  }

  pub fn get_album(&self) -> String {
    self.properties.AlbumTitle().unwrap_or_default().to_string()
  }

  pub fn get_title(&self) -> String {
    self.properties.Title().unwrap_or_default().to_string()
  }

  pub fn get_app_id(&self) -> String {
    self
      .session
      .SourceAppUserModelId()
      .unwrap_or_default()
      .to_string()
  }

  // pub fn get_position(&self) -> HumanDurationData {
  //     self.timeline.Position().unwrap_or_default().cleanup()
  // }

  // pub fn get_position(&self) -> i64 {
  //   self.timeline.Position().unwrap_or_default().Duration / 10_i64.pow(7)
  // }

  // pub fn get_duration(&self) -> HumanDurationData {
  //     self.timeline.EndTime().unwrap_or_default().cleanup()
  // }

  pub fn get_start_time(&self) -> i64 {
    self.timeline.StartTime().unwrap_or_default().Duration / 10_i64.pow(7)
  }

  pub fn get_end_time(&self) -> i64 {
    self.timeline.EndTime().unwrap_or_default().Duration / 10_i64.pow(7)
  }

  pub fn get_status(&self) -> MediaStatus {
    if let Ok(p) = self.session.GetPlaybackInfo() {
      if let Ok(s) = p.PlaybackStatus() {
        return MediaStatus::from(s);
      }
    }
    MediaStatus::Closed
  }

  pub fn get_thumbnail(&self) -> (String, Vec<u8>) {
    match self.properties.Thumbnail() {
      Ok(thumbnail) => {
        let thumbnail_read_async = thumbnail.OpenReadAsync().unwrap_or_else(|_| {
          panic!("Failed to open read stream for {}", self.get_title());
        });
        let thumbnail_read_stream = thumbnail_read_async.get().unwrap_or_else(|_| {
          panic!("Failed to open read stream for {}", self.get_title());
        });
        let thumbnail_stream = thumbnail_read_stream.CloneStream().unwrap_or_else(|_| {
          panic!("Failed to clone stream for {}", self.get_title());
        });

        let bitmap_decoder = BitmapDecoder::CreateAsync(&thumbnail_stream)
          .unwrap_or_else(|_| {
            panic!("Failed to create bitmap decoder for {}", self.get_title());
          })
          .get()
          .unwrap_or_else(|_| {
            panic!("Failed to create bitmap decoder for {}", self.get_title());
          });

        let pixel_data = bitmap_decoder
          .GetPixelDataAsync()
          .unwrap_or_else(|_| {
            panic!("Failed to get pixel data for {}", self.get_title());
          })
          .get()
          .unwrap_or_else(|_| {
            panic!("Failed to get pixel data for {}", self.get_title());
          });

        let bytes_bgra = Vec::from(
          pixel_data
            .DetachPixelData()
            .unwrap_or_else(|_| {
              panic!("Failed to detach pixel data for {}", self.get_title());
            })
            .as_ref() as &[u8],
        );

        // Convert BGRA to RGBA
        let mut bytes = Vec::with_capacity(bytes_bgra.len());
        for i in (0..bytes_bgra.len()).step_by(4) {
          bytes.push(bytes_bgra[i + 2]);
          bytes.push(bytes_bgra[i + 1]);
          bytes.push(bytes_bgra[i]);
          bytes.push(bytes_bgra[i + 3]);
        }

        // Find the average color of the image
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for i in (0..bytes.len()).step_by(4) {
          r += bytes[i] as u32;
          g += bytes[i + 1] as u32;
          b += bytes[i + 2] as u32;
        }

        let len = (bytes.len() / 4) as u32;
        let r = (r / len) as u8;
        let g = (g / len) as u8;
        let b = (b / len) as u8;

        let width = bitmap_decoder.OrientedPixelWidth().unwrap_or_else(|_| {
          panic!("Failed to get pixel width for {}", self.get_title());
        });
        let height = bitmap_decoder.OrientedPixelHeight().unwrap_or_else(|_| {
          panic!("Failed to get pixel height for {}", self.get_title());
        });
        let img: RgbaImage = ImageBuffer::from_vec(width, height, bytes).unwrap_or_else(|| {
          panic!("Failed to create image buffer for {}", self.get_title());
        });

        let dyn_img = DynamicImage::ImageRgba8(img);

        let mut buf = Cursor::new(Vec::new());
        dyn_img
          .write_to(&mut buf, image::ImageFormat::Png)
          .unwrap_or_else(|_| {
            panic!(
              "Failed to write image buffer to cursor for {}",
              self.get_title()
            );
          });

        let base64 = general_purpose::STANDARD.encode(buf.get_ref());

        let base64_img_url = format!("data:image/png;base64,{}", base64);

        (base64_img_url, Vec::from(&[r, g, b]))

        // img.save("D:\\media_session_thumbnail.png").unwrap();
      }
      Err(_) => ("".to_string(), vec![255, 255, 255]),
    }
  }

  pub fn play(&self) -> bool {
    if let Ok(res) = self.session.TryPlayAsync() {
      res.get().unwrap_or(false)
    } else {
      false
    }
  }

  pub fn pause(&self) -> bool {
    if let Ok(res) = self.session.TryPauseAsync() {
      res.get().unwrap_or(false)
    } else {
      false
    }
  }

  pub fn toggle(&self) -> bool {
    if let Ok(res) = self.session.TryTogglePlayPauseAsync() {
      res.get().unwrap_or(false)
    } else {
      false
    }
  }

  pub fn next_track(&self) -> bool {
    if let Ok(res) = self.session.TrySkipNextAsync() {
      res.get().unwrap_or(false)
    } else {
      false
    }
  }

  pub fn previous_track(&self) -> bool {
    if let Ok(res) = self.session.TrySkipPreviousAsync() {
      res.get().unwrap_or(false)
    } else {
      false
    }
  }

  // pub fn stop() -> bool {
  //     todo!()
  // }

  // pub fn skip() -> bool {
  //     todo!()
  // }

  // pub fn previous() -> bool {
  //     todo!()
  // }

  // pub fn set_position(new_pos: u64) -> bool {
  //     todo!()
  // }
}

impl fmt::Display for MediaSession {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} - {} ({})",
      self.get_title(),
      self.get_artist(),
      self.get_start_time()
    )
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaSessionInfo {
  pub status_code: i32, // 200: OK, 402: No media playing
  pub app_id: String,
  pub title: String,
  pub artist: String,
  pub album: String,
  pub start_time: i64,
  pub end_time: i64,
  pub media_status: MediaStatus,
  pub thumbnail: String,
  pub main_color: Vec<u8>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaControlCommand {
  pub command: String,
}

pub fn initiate_media_control(app: &App) -> Result<(), String> {
  let app_handle = app.handle().clone();
  let app_handle2 = app.handle().clone();

  async_runtime::spawn(async move {
    loop {
      match MediaSession::new().await {
        Ok(media_session) => {
          // media_session.init_event_handler().unwrap_or_else(|e| {
          //   eprintln!("Failed to initialize event handler: {}", e);
          // });
          let app_handle_clone = app_handle.clone();
          let ms_clone = media_session.clone();

          app_handle_clone.listen("mediaPlayerCommand", move |event| {
            let res =
              serde_json::from_str::<MediaControlCommand>(event.payload()).unwrap_or_else(|e| {
                eprintln!("Failed to parse media player command: {}", e);
                MediaControlCommand {
                  command: "".to_string(),
                }
              });

            match res.command.as_str() {
              "play_pause" => {
                ms_clone.toggle();
              }
              "next" => {
                ms_clone.next_track();
              }
              "previous" => {
                ms_clone.previous_track();
              }
              _ => {}
            }
          });

          let thumbnail = media_session.get_thumbnail();

          app_handle2
            .emit(
              "mediaControl",
              MediaSessionInfo {
                status_code: 200,
                title: media_session.get_title(),
                app_id: media_session.get_app_id(),
                artist: media_session.get_artist(),
                album: media_session.get_album(),
                start_time: media_session.get_start_time(),
                end_time: media_session.get_end_time(),
                media_status: media_session.get_status(),
                thumbnail: thumbnail.0,
                main_color: thumbnail.1,
              },
            )
            .unwrap_or_else(|e| {
              eprintln!("Failed to emit media control event: {}", e);
            });
        }
        Err(e) => {
          let app_handle_clone = app_handle.clone();

          app_handle_clone
            .emit(
              "mediaControl",
              MediaSessionInfo {
                status_code: 402,
                title: e.to_string(),
                app_id: "".to_string(),
                artist: "".to_string(),
                album: "".to_string(),
                start_time: 0,
                end_time: 0,
                media_status: MediaStatus::Closed,
                thumbnail: "".to_string(),
                main_color: vec![255, 255, 255],
              },
            )
            .unwrap_or_else(|e| {
              eprintln!("Failed to emit media control event: {}", e);
            });
        }
      }

      std::thread::sleep(std::time::Duration::from_millis(500));
    }
  });

  Ok(())
}
