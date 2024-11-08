<script lang="ts">
  import { onMount } from "svelte";
  import SongCard from "$lib/components/SongCard.svelte";

  onMount(() => {
    console.log("Page mounted");
  });
</script>

<div
  class="text-white py-8 px-4 flex flex-col justify-center items-center h-full w-full"
>
  <p
    class="text-center text-5xl font-extrabold py-3 bg-gradient-to-br from-blue-600 via-indigo-600 to-purple-600 bg-clip-text text-transparent"
  >
    MiyabiWidgets
  </p>
  <p class="text-center text-lg">Your Ultimate Desktop Widget Tool</p>

  <SongCard></SongCard>
</div>
