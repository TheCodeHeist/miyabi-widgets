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
  start_time: number;
  end_time: number;
  media_status: MediaControlStatus;
  position: number;
  status_code: number;
  thumbnail: string;
  title: string;
  app_id: string;
  main_color: Array<number>;
}
