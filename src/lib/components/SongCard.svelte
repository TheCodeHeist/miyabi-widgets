<script lang="ts">
  import type { IMediaControlEventPayload } from "$lib/utils/interfaces";
  import { listen, emit, type Event } from "@tauri-apps/api/event";

  let currentEvent = $state({} as IMediaControlEventPayload);

  listen("mediaControl", (event: Event<IMediaControlEventPayload>) => {
    currentEvent = event.payload;
  });

  const sec_to_min = (sec: number) => {
    const minutes = Math.floor(sec / 60);
    const seconds = sec - minutes * 60;
    return `${minutes}:${seconds < 10 ? "0" : ""}${seconds}`;
  };
</script>

<div
  class="w-72 p-5 border border-gray-300 rounded-lg shadow-md text-center flex flex-col justify-evenly items-center"
>
  <!-- <div class="w-full h-[150px]">
  </div> -->
  {#if Object.keys(currentEvent).length === 0}
    <p class="text-3xl">•••</p>
  {:else if currentEvent.status_code === 402}
    <p class="text-red-500">No media playing</p>
  {:else}
    <img
      src={currentEvent.thumbnail}
      alt="Song Cover"
      class="object-cover rounded-lg"
    />
    <h2 class="mt-4 text-xl font-semibold">{currentEvent.title}</h2>
    <p class="text-gray-500">{currentEvent.artist}</p>
    <div class="mt-5">
      <p>Status: {currentEvent.media_status}</p>
      <p>Duration: {sec_to_min(currentEvent.duration)}</p>
    </div>
  {/if}
</div>
