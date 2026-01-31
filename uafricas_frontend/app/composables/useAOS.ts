import AOS from 'aos'

export const useAOS = () => {
  onMounted(() => {
    AOS.init({
      duration: 1000,
      once: true,
      easing: 'ease-out-cubic',
    })
  })

  onUnmounted(() => {
    AOS.refresh()
  })
}
