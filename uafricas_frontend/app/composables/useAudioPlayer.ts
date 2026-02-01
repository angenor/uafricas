import type { RadioStation } from '~/mocks/radios'

export interface AudioPlayerState {
  isPlaying: boolean
  volume: number
  isMuted: boolean
  currentStationIndex: number
  isLoading: boolean
}

export const useAudioPlayer = (stations: RadioStation[]) => {
  // State
  const audioRef = ref<HTMLAudioElement | null>(null)
  const isPlaying = ref(false)
  const volume = ref(0.7)
  const isMuted = ref(false)
  const currentStationIndex = ref(0)
  const isLoading = ref(false)

  // Computed
  const currentStation = computed(() => stations[currentStationIndex.value])

  // Methods
  const play = () => {
    if (audioRef.value) {
      isLoading.value = true
      audioRef.value.play()
        .then(() => {
          isPlaying.value = true
          isLoading.value = false
        })
        .catch((error) => {
          console.error('Erreur lors de la lecture:', error)
          isLoading.value = false
        })
    }
  }

  const pause = () => {
    if (audioRef.value) {
      audioRef.value.pause()
      isPlaying.value = false
    }
  }

  const togglePlay = () => {
    if (isPlaying.value) {
      pause()
    } else {
      play()
    }
  }

  const nextStation = () => {
    currentStationIndex.value = (currentStationIndex.value + 1) % stations.length
    updateSource()
  }

  const previousStation = () => {
    currentStationIndex.value = (currentStationIndex.value - 1 + stations.length) % stations.length
    updateSource()
  }

  const selectStation = (index: number) => {
    if (index >= 0 && index < stations.length) {
      currentStationIndex.value = index
      updateSource()
      if (!isPlaying.value) {
        play()
      }
    }
  }

  const setVolume = (newVolume: number) => {
    volume.value = Math.max(0, Math.min(1, newVolume))
    if (audioRef.value) {
      audioRef.value.volume = volume.value
    }
  }

  const toggleMute = () => {
    if (audioRef.value) {
      audioRef.value.muted = !audioRef.value.muted
      isMuted.value = audioRef.value.muted
    }
  }

  const updateSource = () => {
    if (audioRef.value && currentStation.value) {
      audioRef.value.src = currentStation.value.streamUrl
      if (isPlaying.value) {
        play()
      }
    }
  }

  const handleCanPlay = () => {
    isLoading.value = false
  }

  const handleError = (error: Event) => {
    console.error('Erreur audio:', error)
    isLoading.value = false
    isPlaying.value = false
  }

  const handleEnded = () => {
    nextStation()
  }

  // Initialize audio element
  const initAudio = (audioElement: HTMLAudioElement) => {
    audioRef.value = audioElement
    audioRef.value.volume = volume.value
    if (currentStation.value) {
      audioRef.value.src = currentStation.value.streamUrl
    }
  }

  // Watchers
  watch(volume, (newVolume) => {
    if (audioRef.value) {
      audioRef.value.volume = newVolume
    }
  })

  // Cleanup
  onUnmounted(() => {
    if (audioRef.value) {
      audioRef.value.pause()
      audioRef.value.src = ''
    }
  })

  return {
    // Refs
    audioRef,
    isPlaying,
    volume,
    isMuted,
    currentStationIndex,
    isLoading,

    // Computed
    currentStation,

    // Methods
    play,
    pause,
    togglePlay,
    nextStation,
    previousStation,
    selectStation,
    setVolume,
    toggleMute,
    initAudio,
    handleCanPlay,
    handleError,
    handleEnded
  }
}
