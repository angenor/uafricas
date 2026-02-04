<template>
  <Transition name="fade">
    <div v-if="url" class="fixed inset-0 z-50 flex items-center justify-center">
      <!-- Overlay -->
      <div
        class="absolute inset-0 bg-black/70 backdrop-blur-xs"
        @click="$emit('close')"
      ></div>

      <!-- Modal content - Full height -->
      <div class="relative w-full h-full max-w-5xl mx-auto my-8 animate-zoomIn">
        <div class="bg-white rounded-xl overflow-hidden shadow-2xl h-full flex flex-col">
          <!-- Header -->
          <div class="bg-gradient-to-r from-custom-chocolat to-amber-800 py-3 px-4 flex items-center justify-between">
            <h2 class="text-white font-bold">Visualisation du document</h2>
            <div class="flex space-x-2">
              <button
                v-if="acces === 'Téléchargeable'"
                @click="downloadPDF"
                class="px-3 py-1 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors flex items-center"
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                </svg>
                Télécharger
              </button>
              <button
                @click="$emit('close')"
                class="p-1 hover:bg-red-600/20 rounded-full transition-colors"
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
          </div>

          <!-- PDF viewer body -->
          <div class="flex-1 bg-gray-100 overflow-hidden">
            <iframe
              :src="`${url}#toolbar=0`"
              class="w-full h-full"
              frameborder="0"
            ></iframe>
          </div>

          <!-- Page navigation footer -->
          <div class="bg-gray-100 border-t border-gray-200 px-4 py-3 flex justify-between items-center">
            <button
              class="px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors flex items-center"
              @click="previousPage"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
              </svg>
              Page précédente
            </button>
            <div class="text-gray-600">
              Page <span class="font-bold">{{ currentPage }}</span> / <span>{{ totalPages }}</span>
            </div>
            <button
              class="px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors flex items-center"
              @click="nextPage"
            >
              Page suivante
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 ml-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
const props = defineProps<{
  url: string | null
  acces?: string
}>()

defineEmits<{
  close: []
}>()

// Mock page navigation state (visual only - iframe handles actual PDF navigation)
const currentPage = ref(1)
const totalPages = ref(10)

function previousPage() {
  if (currentPage.value > 1) {
    currentPage.value--
  }
}

function nextPage() {
  if (currentPage.value < totalPages.value) {
    currentPage.value++
  }
}

function downloadPDF() {
  if (props.url) {
    const link = document.createElement('a')
    link.href = props.url
    link.setAttribute('download', 'document.pdf')
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  }
}
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

@keyframes zoomIn {
  from {
    transform: scale(0.95);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}

.animate-zoomIn {
  animation: zoomIn 0.3s ease-out forwards;
}
</style>
