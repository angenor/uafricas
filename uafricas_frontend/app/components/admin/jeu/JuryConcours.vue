<script setup lang="ts">
/**
 * Délibération du jury (feature 014, US8) : les finalistes du vote en grand,
 * le jury en retient une à trois, dans l'ordre. Sans décision dans le délai,
 * le classement de la communauté s'applique seul.
 */
import { messageErreurConcours, type ConcoursAdminAPI, type FinalisteAPI } from '~/composables/useAdminConcours'

const props = defineProps<{ concours: ConcoursAdminAPI }>()
const emit = defineEmits<{ delibere: [concours: ConcoursAdminAPI] }>()

const { finalistes, deliberer } = useAdminConcours()
const liste = ref<FinalisteAPI[]>([])
const podium = ref<string[]>([])
const chargement = ref(true)
const enCours = ref(false)
const erreur = ref('')

const basculer = (id: string) => {
  if (podium.value.includes(id)) podium.value = podium.value.filter(p => p !== id)
  else if (podium.value.length < 3) podium.value = [...podium.value, id]
}

const confirmer = async () => {
  if (!podium.value.length || !confirm('Fixer ce podium ? Les résultats seront publiés aussitôt.')) return
  enCours.value = true
  erreur.value = ''
  try {
    const c = await deliberer(props.concours.id, podium.value)
    if (c) emit('delibere', c)
  }
  catch (e) {
    erreur.value = messageErreurConcours(e)
  }
  finally {
    enCours.value = false
  }
}

onMounted(async () => {
  try {
    liste.value = await finalistes(props.concours.id)
  }
  catch (e) {
    erreur.value = messageErreurConcours(e, 'Les finalistes n\'ont pas pu être chargés.')
  }
  finally {
    chargement.value = false
  }
})

const limite = computed(() => {
  const d = new Date(props.concours.vote_fin)
  d.setDate(d.getDate() + props.concours.jury_delai_jours)
  return d.toLocaleString('fr-FR', { day: 'numeric', month: 'long', hour: '2-digit', minute: '2-digit' })
})
</script>

<template>
  <div>
    <p v-if="concours.phase !== 'deliberation'" class="alert alert-info text-sm">
      Le jury délibère après la clôture du vote, jusqu'au {{ limite }}. Au-delà, le classement de la communauté s'applique.
    </p>
    <template v-else>
      <p class="mb-4 text-sm text-base-content/70">
        Choisissez une à trois photos parmi les finalistes, dans l'ordre du podium. Délai : {{ limite }}.
      </p>
      <div v-if="erreur" class="alert alert-error mb-4 text-sm" role="alert">{{ erreur }}</div>
      <div v-if="chargement" class="skeleton h-64 w-full" />
      <div v-else class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
        <button
          v-for="f in liste"
          :key="f.participation_id"
          type="button"
          class="card border-2 bg-base-100 text-left"
          :class="podium.includes(f.participation_id) ? 'border-primary' : 'border-base-300'"
          @click="basculer(f.participation_id)"
        >
          <figure class="relative bg-base-200">
            <img :src="urlMedia(f.media_url) ?? undefined" alt="" class="h-52 w-full object-contain">
            <span v-if="podium.includes(f.participation_id)" class="badge badge-primary absolute left-2 top-2">
              {{ podium.indexOf(f.participation_id) + 1 }}{{ podium.indexOf(f.participation_id) === 0 ? 're' : 'e' }} place
            </span>
          </figure>
          <div class="card-body gap-1 p-3 text-sm">
            <p class="font-medium">{{ f.auteur }}</p>
            <p v-if="f.legende" class="text-base-content/70">« {{ f.legende }} »</p>
            <p class="text-xs text-base-content/60">{{ f.rang }}e du vote · {{ f.taux }} % sur {{ f.duels }} duels</p>
          </div>
        </button>
      </div>
      <div class="mt-4 flex justify-end">
        <button type="button" class="btn btn-primary btn-sm" :disabled="enCours || !podium.length" @click="confirmer">
          Fixer le podium ({{ podium.length }}/3)
        </button>
      </div>
    </template>
  </div>
</template>
