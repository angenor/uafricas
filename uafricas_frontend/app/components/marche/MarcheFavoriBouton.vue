<template>
  <button
    type="button"
    class="flex items-center justify-center transition-all"
    :class="classeBouton"
    :title="actif ? 'Retirer des favoris' : 'Ajouter aux favoris'"
    :disabled="enCours"
    @click.prevent.stop="basculer"
  >
    <font-awesome-icon
      :icon="[actif ? 'fas' : 'far', 'heart']"
      :class="actif ? 'text-red-500' : 'text-gray-400'"
    />
    <span v-if="avecLibelle" class="ml-2 text-sm font-medium">
      {{ actif ? 'En favori' : 'Ajouter aux favoris' }}
    </span>
  </button>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { navigateTo } from '#app'
import { useMarcheAfricain } from '~/composables/useMarcheAfricain'
import { useUserStore } from '~/stores/user'

const props = withDefaults(
  defineProps<{
    annonceId: string
    favoriInitial?: boolean
    avecLibelle?: boolean
    variante?: 'carte' | 'detail'
  }>(),
  { favoriInitial: false, avecLibelle: false, variante: 'carte' },
)

const emit = defineEmits<{
  (e: 'change', actif: boolean): void
}>()

const userStore = useUserStore()
const { ajouterFavori, retirerFavori } = useMarcheAfricain()

const actif = ref(props.favoriInitial)
const enCours = ref(false)

const classeBouton = computed(() => {
  if (props.variante === 'detail') {
    return 'px-4 py-2.5 rounded-xl border border-gray-200 bg-white hover:bg-gray-50'
  }
  return 'w-9 h-9 rounded-full bg-white/90 shadow-sm hover:bg-white'
})

const basculer = async () => {
  if (!userStore.isAuthenticated) {
    await navigateTo('/login')
    return
  }
  enCours.value = true
  try {
    const ok = actif.value ? await retirerFavori(props.annonceId) : await ajouterFavori(props.annonceId)
    if (ok) {
      actif.value = !actif.value
      emit('change', actif.value)
    }
  } finally {
    enCours.value = false
  }
}
</script>
