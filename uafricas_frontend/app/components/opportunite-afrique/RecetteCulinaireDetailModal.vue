<script setup lang="ts">
import { useOpportuniteAfrique } from '~/composables/useOpportuniteAfrique'
import type { RecetteCulinaireAPI } from '~/composables/useOpportuniteAfrique'

interface Props {
  isOpen: boolean
  recette: RecetteCulinaireAPI | null
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'close'): void
  (e: 'edit', recette: RecetteCulinaireAPI): void
  (e: 'delete', recette: RecetteCulinaireAPI): void
}>()

const { resoudreUrlImage } = useOpportuniteAfrique()

// Galerie : index de l'image affichée
const indexImage = ref(0)

watch(
  () => props.recette?.id,
  () => {
    indexImage.value = 0
  },
)

const images = computed(() => props.recette?.images ?? [])

const imagePrincipale = computed(() =>
  images.value.length ? resoudreUrlImage(images.value[indexImage.value]) : '',
)

const precedente = () => {
  if (!images.value.length) return
  indexImage.value = (indexImage.value - 1 + images.value.length) % images.value.length
}
const suivante = () => {
  if (!images.value.length) return
  indexImage.value = (indexImage.value + 1) % images.value.length
}

const fermer = () => emit('close')

const onKeydown = (e: KeyboardEvent) => {
  if (!props.isOpen) return
  if (e.key === 'Escape') fermer()
  else if (e.key === 'ArrowLeft') precedente()
  else if (e.key === 'ArrowRight') suivante()
}
onMounted(() => window.addEventListener('keydown', onKeydown))
onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown))
</script>

<template>
  <Teleport to="body">
    <Transition name="modal-fade">
      <div
        v-if="isOpen && recette"
        class="fixed inset-0 z-[90] flex items-center justify-center p-4"
        @click.self="fermer"
      >
        <div class="absolute inset-0 bg-black/50 backdrop-blur-sm"></div>

        <div class="relative bg-white rounded-2xl shadow-2xl w-full max-w-3xl max-h-[90vh] overflow-y-auto">
          <!-- Bouton fermer -->
          <button
            type="button"
            class="absolute top-4 right-4 z-10 w-9 h-9 flex items-center justify-center rounded-full bg-white/90 text-gray-500 hover:bg-gray-100 hover:text-gray-800 shadow transition-colors cursor-pointer"
            @click="fermer"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" class="w-5 h-5" />
          </button>

          <!-- Galerie -->
          <div class="relative h-64 sm:h-80 bg-gradient-to-br from-amber-100 to-orange-200">
            <img
              v-if="imagePrincipale"
              :src="imagePrincipale"
              :alt="recette.titre"
              class="w-full h-full object-cover"
            />
            <div v-else class="w-full h-full flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'utensils']" class="w-16 h-16 text-amber-600" />
            </div>

            <!-- Navigation galerie -->
            <template v-if="images.length > 1">
              <button
                type="button"
                class="absolute left-3 top-1/2 -translate-y-1/2 w-9 h-9 flex items-center justify-center rounded-full bg-white/85 text-gray-700 hover:bg-white shadow cursor-pointer"
                @click="precedente"
              >
                <font-awesome-icon :icon="['fas', 'chevron-left']" class="w-4 h-4" />
              </button>
              <button
                type="button"
                class="absolute right-3 top-1/2 -translate-y-1/2 w-9 h-9 flex items-center justify-center rounded-full bg-white/85 text-gray-700 hover:bg-white shadow cursor-pointer"
                @click="suivante"
              >
                <font-awesome-icon :icon="['fas', 'chevron-right']" class="w-4 h-4" />
              </button>
              <div class="absolute bottom-3 left-1/2 -translate-x-1/2 flex gap-1.5">
                <span
                  v-for="(img, i) in images"
                  :key="i"
                  class="w-2 h-2 rounded-full transition-colors"
                  :class="i === indexImage ? 'bg-white' : 'bg-white/50'"
                />
              </div>
            </template>
          </div>

          <!-- Contenu -->
          <div class="p-6">
            <h2 class="font-oswald text-3xl font-bold text-gray-900">{{ recette.titre }}</h2>

            <p
              v-if="recette.territoires_consommation"
              class="inline-flex items-center gap-1.5 text-sm text-gray-500 mt-2"
            >
              <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3.5 h-3.5 text-amber-700" />
              {{ recette.territoires_consommation }}
            </p>

            <p v-if="recette.histoire" class="text-gray-600 leading-relaxed mt-4">
              {{ recette.histoire }}
            </p>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
              <!-- Ingrédients -->
              <div v-if="recette.ingredients && recette.ingredients.length">
                <h3 class="text-sm font-semibold text-gray-700 mb-2 flex items-center gap-2">
                  <font-awesome-icon :icon="['fas', 'basket-shopping']" class="w-4 h-4 text-amber-700" />
                  Ingrédients
                </h3>
                <ul class="text-sm text-gray-600 space-y-1 list-disc list-inside">
                  <li v-for="(ing, i) in recette.ingredients" :key="i">{{ ing }}</li>
                </ul>
              </div>

              <!-- Préparation -->
              <div v-if="recette.etapes_preparation && recette.etapes_preparation.length">
                <h3 class="text-sm font-semibold text-gray-700 mb-2 flex items-center gap-2">
                  <font-awesome-icon :icon="['fas', 'list-ol']" class="w-4 h-4 text-amber-700" />
                  Mode de préparation
                </h3>
                <ol class="text-sm text-gray-600 space-y-2">
                  <li v-for="(etape, i) in recette.etapes_preparation" :key="i" class="flex gap-2">
                    <span class="font-semibold text-amber-700 shrink-0">{{ i + 1 }}.</span>
                    <span>{{ etape }}</span>
                  </li>
                </ol>
              </div>
            </div>

            <!-- Bandeau de suspension (>10 signalements) -->
            <div
              v-if="recette.suspendu"
              class="mt-6 flex items-start gap-2 rounded-md bg-orange-50 border border-orange-200 px-3 py-2 text-sm text-orange-800"
            >
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-4 h-4 mt-0.5 shrink-0" />
              <span>Contribution suspendue — en cours de vérification par la modération. Modification temporairement indisponible.</span>
            </div>

            <!-- Actions -->
            <div v-else class="flex items-center gap-4 mt-6 pt-4 border-t border-gray-100">
              <button
                type="button"
                class="inline-flex items-center gap-1.5 text-sm font-medium text-custom-chocolat hover:underline cursor-pointer"
                @click="emit('edit', recette)"
              >
                <font-awesome-icon :icon="['fas', 'pen']" class="w-3.5 h-3.5" />
                Modifier
              </button>
              <button
                type="button"
                class="inline-flex items-center gap-1.5 text-sm font-medium text-red-600 hover:underline cursor-pointer"
                @click="emit('delete', recette)"
              >
                <font-awesome-icon :icon="['fas', 'trash']" class="w-3.5 h-3.5" />
                Supprimer
              </button>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.2s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>
