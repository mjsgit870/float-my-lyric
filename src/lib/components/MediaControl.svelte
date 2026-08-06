<script lang="ts">
  import { nextMedia, prevMedia, togglePlayMedia } from '$lib/utils'

  const {
    isPlaying
  }: {
    isPlaying: boolean
  } = $props()

  async function togglePlayPause() {
    try {
      await togglePlayMedia()
    } catch (error) {
      console.error('Gagal mengubah status playback:', error)
    }
  }

  async function playNext() {
    try {
      await nextMedia()
    } catch (error) {
      console.error('Gagal memutar lagu selanjutnya:', error)
    }
  }

  async function playPrev() {
    try {
      await prevMedia()
    } catch (error) {
      console.error('Gagal memutar lagu sebelumnya:', error)
    }
  }
</script>

<div class="flex items-center justify-center gap-6 mt-2">
  <!-- Tombol Previous -->
  <button
    onclick={playPrev}
    class="text-gray-400 hover:text-white transition-all duration-300"
    aria-label="Previous"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <polygon points="19 20 9 12 19 4 19 20"></polygon>
      <line x1="5" y1="19" x2="5" y2="5"></line>
    </svg>
  </button>

  <!-- Tombol Play / Pause -->
  <button
    onclick={togglePlayPause}
    class="p-3 bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 rounded-full transition-all duration-300"
    aria-label="Play/Pause"
  >
    {#if isPlaying}
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="28"
        height="28"
        viewBox="0 0 24 24"
        fill="currentColor"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <rect x="6" y="4" width="4" height="16"></rect>
        <rect x="14" y="4" width="4" height="16"></rect>
      </svg>
    {:else}
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="28"
        height="28"
        viewBox="0 0 24 24"
        fill="currentColor"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <polygon points="5 3 19 12 5 21 5 3"></polygon>
      </svg>
    {/if}
  </button>

  <!-- Tombol Next -->
  <button
    onclick={playNext}
    class="text-gray-400 hover:text-white transition-all duration-300"
    aria-label="Next"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <polygon points="5 4 15 12 5 20 5 4"></polygon>
      <line x1="19" y1="5" x2="19" y2="19"></line>
    </svg>
  </button>
</div>
