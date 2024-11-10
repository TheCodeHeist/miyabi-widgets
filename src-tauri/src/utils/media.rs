use std::fmt;

use base64::{engine::general_purpose, Engine as _};
use image::{DynamicImage, ImageBuffer, RgbaImage};
use std::io::Cursor;
use tauri::{async_runtime, App, Emitter};
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus as WinPlaybackStatus;
use windows::{
  Graphics::Imaging::BitmapDecoder,
  Media::Control::{
    GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties,
    GlobalSystemMediaTransportControlsSessionTimelineProperties,
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

pub struct MediaSession {
  session: GlobalSystemMediaTransportControlsSession,
  properties: GlobalSystemMediaTransportControlsSessionMediaProperties,
  timeline: GlobalSystemMediaTransportControlsSessionTimelineProperties,
}

impl MediaSession {
  pub async fn new() -> Result<Self, windows::core::Error> {
    let mp = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.await?;
    let session = mp.GetCurrentSession()?;
    let properties = session.TryGetMediaPropertiesAsync()?.await?;
    let timeline = session.GetTimelineProperties()?;
    Ok(Self {
      session,
      properties,
      timeline,
    })
  }

  pub fn get_artist(&self) -> String {
    self.properties.Artist().unwrap_or_default().to_string()
  }

  pub fn get_title(&self) -> String {
    self.properties.Title().unwrap_or_default().to_string()
  }

  // pub fn get_position(&self) -> HumanDurationData {
  //     self.timeline.Position().unwrap_or_default().cleanup()
  // }

  pub fn get_position(&self) -> i64 {
    self.timeline.Position().unwrap_or_default().Duration / 10_i64.pow(7)
  }

  // pub fn get_duration(&self) -> HumanDurationData {
  //     self.timeline.EndTime().unwrap_or_default().cleanup()
  // }

  pub fn get_duration(&self) -> i64 {
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

  pub fn get_thumbnail(&self) -> String {
    let thumbnail = self.properties.Thumbnail().unwrap_or_else(|_| {
      panic!("Failed to get thumbnail for {}", self.get_title());
    });
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

    format!("data:image/png;base64,{}", base64)

    // img.save("D:\\media_session_thumbnail.png").unwrap();
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
      self.get_position()
    )
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaSessionInfo {
  pub status_code: i32, // 200: OK, 402: No media playing
  pub title: String,
  pub artist: String,
  pub position: i64,
  pub duration: i64,
  pub media_status: MediaStatus,
  pub thumbnail: String,
}

pub fn initiate_media_control(app: &App) -> Result<(), String> {
  let app_handle = app.handle().clone();

  async_runtime::spawn(async move {
    loop {
      match MediaSession::new().await {
        Ok(media_session) => {
          app_handle
            .emit(
              "mediaControl",
              MediaSessionInfo {
                status_code: 200,
                title: media_session.get_title(),
                artist: media_session.get_artist(),
                position: media_session.get_position(),
                duration: media_session.get_duration(),
                media_status: media_session.get_status(),
                thumbnail: media_session.get_thumbnail(),
              },
            )
            .unwrap_or_else(|e| {
              eprintln!("Failed to emit media control event: {}", e);
            });
        }
        Err(e) => {
          app_handle
            .emit(
              "mediaControl",
              MediaSessionInfo {
                status_code: 402,
                title: e.to_string(),
                artist: "".to_string(),
                position: 0,
                duration: 0,
                media_status: MediaStatus::Closed,
                thumbnail: "".to_string(),
              },
            )
            .unwrap_or_else(|e| {
              eprintln!("Failed to emit media control event: {}", e);
            });
        }
      }

      std::thread::sleep(std::time::Duration::from_secs(5));
    }
  });

  Ok(())
}
