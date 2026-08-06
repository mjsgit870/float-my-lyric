<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window'
  import { Client, type LyricLine } from 'lrclib-api'
  import { onDestroy, onMount } from 'svelte'
  import { flip } from 'svelte/animate'
  import { fly } from 'svelte/transition'
  import { mediaControls } from 'tauri-plugin-media-api'

  const client = new Client()
  const appWindow = getCurrentWindow()

  let localPosition = $state(0)
  let isPlaying = $state(false)
  let lastOsPosition = $state(-1)

  let currentTitle = $state('')
  let currentArtist = $state('')

  let lyrics = $state<LyricLine[]>([])

  let metadataInterval: ReturnType<typeof setInterval>
  let positionInterval: ReturnType<typeof setInterval>
  let uiInterval: ReturnType<typeof setInterval>

  let lastTick = performance.now()

  const activeIndex = $derived.by(() => {
    for (let i = lyrics.length - 1; i >= 0; i--) {
      const line = lyrics[i]
      if (line.startTime !== undefined && localPosition >= line.startTime) {
        return i
      }
    }
    return -1
  })

  const visibleLyrics = $derived.by(() => {
    if (lyrics.length === 0) return []

    const center = activeIndex === -1 ? 0 : activeIndex

    const start = Math.max(0, center - 1)
    const end = Math.min(lyrics.length, center + 2)

    return lyrics.slice(start, end).map((line, i) => {
      const originalIndex = start + i
      return {
        ...line,
        id: `${originalIndex}-${line.startTime || 0}`,
        isActive: originalIndex === activeIndex,
      }
    })
  })

  async function getLyric(title: string, artist: string) {
    if (!title || !artist) return

    try {
      const synced = await client.getSynced({
        track_name: title,
        artist_name: artist,
      })
      lyrics = synced || []
    } catch (error) {
      console.error('Lirik tidak ditemukan atau gagal diambil:', error)
      lyrics = []
    }
  }

  async function closeApp() {
    await appWindow.close()
  }

  onMount(async () => {
    try {
      metadataInterval = setInterval(async () => {
        const metadata = await mediaControls.getMetadata()
        if (
          metadata &&
          (metadata.title !== currentTitle || metadata.artist !== currentArtist)
        ) {
          currentTitle = metadata.title || ''
          currentArtist = metadata.artist || ''
          getLyric(currentTitle, currentArtist)
        }
      }, 1000)

      positionInterval = setInterval(async () => {
        const osPosition = await mediaControls.getPosition()
        const status = await mediaControls.getPlaybackStatus()

        isPlaying = status === 'playing'

        if (osPosition !== lastOsPosition) {
          localPosition = osPosition
          lastOsPosition = osPosition
        }
      }, 250)

      lastTick = performance.now()

      uiInterval = setInterval(() => {
        const now = performance.now()
        const delta = (now - lastTick) / 1000
        lastTick = now

        if (isPlaying) {
          localPosition += delta
        }
      }, 100)
    } catch (error) {
      console.error(error)
    }
  })

  onDestroy(() => {
    clearInterval(metadataInterval)
    clearInterval(positionInterval)
    clearInterval(uiInterval)
  })
</script>

<div class="relative p-4 flex flex-col items-center w-full h-full overflow-hidden">
  <button
    onclick={closeApp}
    class="absolute top-0 right-0 z-50 p-2 text-gray-400 hover:text-red-400 hover:bg-red-400/20 transition-all duration-300"
    aria-label="Tutup Aplikasi"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="20"
      height="20"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <line x1="18" y1="6" x2="6" y2="18"></line>
      <line x1="6" y1="6" x2="18" y2="18"></line>
    </svg>
  </button>

  <p class="text-white font-bold text-lg drop-shadow-md text-center">
    {currentTitle || 'Tidak ada lagu'}
  </p>
  <p class="text-gray-400 text-sm drop-shadow-md text-center">
    {currentArtist}
  </p>

  <!-- Container lirik dengan tinggi tetap agar window tidak melompat-lompat -->
  <div
    class="relative flex flex-col items-center justify-center gap-1 w-full overflow-hidden"
  >
    {#if lyrics.length > 0}
      <!-- Gunakan keyed each block (line.id) agar Svelte bisa mendeteksi elemen untuk dianimasikan -->
      {#each visibleLyrics as line (line.id)}
        <p
          animate:flip={{ duration: 400 }}
          in:fly={{ y: 20, duration: 400 }}
          out:fly={{ y: -20, duration: 400 }}
          class="
            text-center drop-shadow-md transition-all duration-500 ease-in-out absolute
              {line.isActive
            ? 'text-emerald-400 text-xl font-bold opacity-100 scale-100 relative'
            : 'text-gray-400 text-lg opacity-40 scale-90 relative'}
          "
        >
          {line.text}
        </p>
      {/each}
    {:else if currentTitle}
      <p class="text-gray-500 italic drop-shadow-md text-center">
        Lirik tidak tersedia
      </p>
    {/if}
  </div>
</div>
