export interface IMediaControlEvent {
  event: "mediaControl";
  payload: IMediaControlEventPayload;
  id: number;
}

export enum MediaControlStatus {
  PLAYING = "Playing",
  PAUSED = "Paused",
  STOPPED = "Stopped",
  CHANGING = "Changing",
  CLOSED = "Closed",
  OPENED = "Opened",
}

export interface IMediaControlEventPayload {
  artist: string;
  duration: number;
  media_status: MediaControlStatus;
  position: number;
  status_code: number;
  thumbnail: string;
  title: string;
}
