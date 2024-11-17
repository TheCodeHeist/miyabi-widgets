<script lang="ts">
  let { data } = $props();

  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import type { IMediaControlEventPayload } from "$lib/utils/interfaces";
  import { listen, emit, type Event } from "@tauri-apps/api/event";

  let currentEvent = $state({} as IMediaControlEventPayload);
  let grabbing = $state(false);

  listen("mediaControl", (event: Event<IMediaControlEventPayload>) => {
    currentEvent = event.payload;

    // console.log(currentEvent);
  });

  const sec_to_min = (sec: number) => {
    const minutes = Math.floor(sec / 60);
    const seconds = sec - minutes * 60;
    return `${minutes}:${seconds < 10 ? "0" : ""}${seconds}`;
  };

  let config;

  async function get_widget_config() {
    config = await invoke("get_widget_config", {
      widgetId: data.id,
    });
  }

  async function toggle_play_pause() {
    await emit("mediaPlayerCommand", {
      command: "play_pause",
    });
  }

  async function next_track() {
    await emit("mediaPlayerCommand", {
      command: "next",
    });
  }

  async function previous_track() {
    await emit("mediaPlayerCommand", {
      command: "previous",
    });
  }

  onMount(() => {
    get_widget_config();
  });
</script>

<div
  style={"background-image: url(" + currentEvent.thumbnail + ");"}
  class="h-screen w-full bg-cover bg-center shadow-[inset_rgba(60,70,85,0.5)_0px_0px_40px_0px,_inset_rgba(60,_70,_85,_0.5)_0px_0px_40px_0px,_inset_rgba(0,0,0,1)_0px_0px_36px_-24px]"
>
  <div
    class="h-full w-full flex flex-col gap-4 justify-center items-center text-center text-white p-4 backdrop-blur-md bg-opacity-50 bg-black backdrop-invert"
  >
    <div
      class={"fixed w-full top-0 flex items-center justify-center p-2 transition-all bg-transparent hover:bg-[#00000036] select-none z-50 " +
        (grabbing ? "cursor-grabbing" : "cursor-grab")}
      role="button"
      tabindex="0"
      onmousedown={() => (grabbing = true)}
      onmouseup={() => (grabbing = false)}
      onmouseover={() => (grabbing = false)}
      onfocus={() => (grabbing = false)}
      data-tauri-drag-region
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 448 512"
        class="fill-white w-4 pointer-events-none"
      >
        <!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
        <path
          d="M32 288c-17.7 0-32 14.3-32 32s14.3 32 32 32l384 0c17.7 0 32-14.3 32-32s-14.3-32-32-32L32 288zm0-128c-17.7 0-32 14.3-32 32s14.3 32 32 32l384 0c17.7 0 32-14.3 32-32s-14.3-32-32-32L32 160z"
        />
      </svg>
    </div>

    {#if Object.keys(currentEvent).length === 0}
      <p class="text-3xl">•••</p>
    {:else if currentEvent.status_code === 402}
      <p class="text-red-500">No media playing</p>
    {:else}
      {#if currentEvent.thumbnail === ""}
        <div
          class="w-[200px] h-[200px] rounded-lg shadow-lg shadow-black bg-slate-800 flex items-center justify-center"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 512 512"
            class="fill-white w-20 h-20"
          >
            <!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
            <path
              d="M0 256a256 256 0 1 1 512 0A256 256 0 1 1 0 256zm256 32a32 32 0 1 1 0-64 32 32 0 1 1 0 64zm-96-32a96 96 0 1 0 192 0 96 96 0 1 0 -192 0zM96 240c0-35 17.5-71.1 45.2-98.8S205 96 240 96c8.8 0 16-7.2 16-16s-7.2-16-16-16c-45.4 0-89.2 22.3-121.5 54.5S64 194.6 64 240c0 8.8 7.2 16 16 16s16-7.2 16-16z"
            />
          </svg>
        </div>
      {:else}
        <img
          src={currentEvent.thumbnail}
          alt="Song Cover"
          class="rounded-lg shadow-lg shadow-black"
          width="200"
          height="200"
        />
      {/if}

      <div class="w-full flex flex-col gap-1">
        <h2 class="text-lg font-bold">{currentEvent.title}</h2>
        <p class="text-base font-medium">{currentEvent.artist}</p>
        <p class="text-xs">{sec_to_min(currentEvent.end_time)}</p>
      </div>

      <div class="flex gap-6 items-center">
        <button
          type="button"
          class="w-4 h-4 flex items-center justify-center fill-white opacity-75 transition-all hover:opacity-100"
          aria-label="previous_track"
          onclick={previous_track}
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512">
            <!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
            <path
              d="M267.5 440.6c9.5 7.9 22.8 9.7 34.1 4.4s18.4-16.6 18.4-29l0-320c0-12.4-7.2-23.7-18.4-29s-24.5-3.6-34.1 4.4l-192 160L64 241 64 96c0-17.7-14.3-32-32-32S0 78.3 0 96L0 416c0 17.7 14.3 32 32 32s32-14.3 32-32l0-145 11.5 9.6 192 160z"
            />
          </svg>
        </button>

        <button
          type="button"
          class="w-10 h-10 flex items-center justify-center fill-white opacity-75 transition-all hover:opacity-100"
          aria-label="play_pause"
          onclick={toggle_play_pause}
        >
          {#if currentEvent.media_status === "Playing"}
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
              <!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
              <path
                d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM224 192l0 128c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-128c0-17.7 14.3-32 32-32s32 14.3 32 32zm128 0l0 128c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-128c0-17.7 14.3-32 32-32s32 14.3 32 32z"
              />
            </svg>
          {:else}
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
              <!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
              <path
                d="M0 256a256 256 0 1 1 512 0A256 256 0 1 1 0 256zM188.3 147.1c-7.6 4.2-12.3 12.3-12.3 20.9l0 176c0 8.7 4.7 16.7 12.3 20.9s16.8 4.1 24.3-.5l144-88c7.1-4.4 11.5-12.1 11.5-20.5s-4.4-16.1-11.5-20.5l-144-88c-7.4-4.5-16.7-4.7-24.3-.5z"
              />
            </svg>
          {/if}
        </button>

        <button
          type="button"
          class="w-4 h-4 flex items-center justify-center fill-white opacity-75 transition-all hover:opacity-100"
          aria-label="next_track"
          onclick={next_track}
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512">
            <!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
            <path
              d="M52.5 440.6c-9.5 7.9-22.8 9.7-34.1 4.4S0 428.4 0 416L0 96C0 83.6 7.2 72.3 18.4 67s24.5-3.6 34.1 4.4l192 160L256 241l0-145c0-17.7 14.3-32 32-32s32 14.3 32 32l0 320c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-145-11.5 9.6-192 160z"
            />
          </svg>
        </button>
      </div>
    {/if}

    <footer
      class="fixed w-full bottom-0 flex items-center justify-center text-sm p-2 bg-[#0000001f] select-none z-50"
    >
      <p>Playing From</p>
      &nbsp;
      <p class="font-semibold">{currentEvent.app_id}</p>
    </footer>
  </div>
</div>
