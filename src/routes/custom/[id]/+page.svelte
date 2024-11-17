<script>
  import { invoke } from "@tauri-apps/api/core";

  function resize() {
    invoke("resize_window");
  }
</script>

<p class="text-xl text-white">This is a custom widget ig?</p>

<button type="button" on:click={() => resize()}>Click me</button>
